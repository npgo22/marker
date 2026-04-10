use std::env;
use std::fs;
use std::process::{exit, Child, Command};
use std::thread;
use std::time::Duration;

const GPU_STAGGER_DELAY_SECS: u64 = 5;

fn required_env(name: &str) -> String {
    match env::var(name) {
        Ok(value) if !value.trim().is_empty() => value,
        _ => {
            eprintln!("Please set the {} environment variable.", name);
            exit(1);
        }
    }
}

fn parse_positive_i32(name: &str, value: &str) -> i32 {
    match value.parse::<i32>() {
        Ok(v) if v > 0 => v,
        _ => {
            eprintln!("{} must be a positive integer.", name);
            exit(1);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an input folder.");
        exit(1);
    }
    if args.len() < 3 {
        eprintln!("Please provide an output folder.");
        exit(1);
    }

    let input_folder = &args[1];
    let output_folder = &args[2];

    if let Err(err) = fs::create_dir_all(output_folder) {
        eprintln!("Failed to create output folder {}: {}", output_folder, err);
        exit(1);
    }

    let num_devices_raw = required_env("NUM_DEVICES");
    let num_workers_raw = required_env("NUM_WORKERS");
    let num_devices = parse_positive_i32("NUM_DEVICES", &num_devices_raw);
    let num_workers = parse_positive_i32("NUM_WORKERS", &num_workers_raw);
    let num_workers_arg = num_workers.to_string();

    let mut children: Vec<(i32, Child)> = Vec::new();

    for i in 0..num_devices {
        println!("Running marker on GPU {}", i);
        let mut command = Command::new("marker");
        command
            .env("CUDA_VISIBLE_DEVICES", i.to_string())
            .env("DEVICE_NUM", i.to_string())
            .env("NUM_DEVICES", &num_devices_raw)
            .env("NUM_WORKERS", &num_workers_raw)
            .arg(input_folder)
            .arg("--output_dir")
            .arg(output_folder)
            .arg("--num_chunks")
            .arg(&num_devices_raw)
            .arg("--chunk_idx")
            .arg(i.to_string())
            .arg("--workers")
            .arg(&num_workers_arg);

        match command.spawn() {
            Ok(child) => children.push((i, child)),
            Err(err) => {
                eprintln!("Failed to start marker for GPU {}: {}", i, err);
                exit(1);
            }
        }

        thread::sleep(Duration::from_secs(GPU_STAGGER_DELAY_SECS));
    }

    let mut failed = false;
    for (gpu, mut child) in children {
        match child.wait() {
            Ok(status) if status.success() => {}
            Ok(status) => {
                eprintln!("marker failed on GPU {} with status {}", gpu, status);
                failed = true;
            }
            Err(err) => {
                eprintln!("Failed waiting for marker on GPU {}: {}", gpu, err);
                failed = true;
            }
        }
    }

    if failed {
        exit(1);
    }
}
