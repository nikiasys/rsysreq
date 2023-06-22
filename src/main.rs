use std::process::Command;
use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    if let Ok(username) = env::var("USER") {
        println!("Username: {}", username);
    } else {
        println!("Username: not found.");
    }
    if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
        let mut distro_name = None;
        let mut distro_version = None;
        for line in os_release.lines() {
            if line.starts_with("ID=") {
                distro_name = Some(line.replace("ID=", ""));
            } else if line.starts_with("VERSION_ID=") {
                distro_version = Some(line.replace("VERSION_ID=", ""));
            }
            if distro_name.is_some() && distro_version.is_some() {
                break;
            }
        }
        if let Some(name) = distro_name {
            println!("Linux Distribution: {}", name);
        }
        if let Some(version) = distro_version {
            println!("Version: {}", version);
        }
    } else {
        println!("Linux distribution information not found.");
    }
    let output = Command::new("uname")
        .arg("-r")
        .output()
        .expect("Failed to run command.");

    let kernel_version = String::from_utf8_lossy(&output.stdout);
    println!("Kernel Version: {}", kernel_version.trim());

    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("model name") {
                    let model_name = line.split(":").nth(1).map(|s| s.trim());
                    if let Some(model_name) = model_name {
                        println!("CPU Model: {}", model_name);
                        break;
                    }
                }
            }
        }
    }
    let output = Command::new("nproc").output().expect("Failed to Run Command.");
    let nproc = String::from_utf8_lossy(&output.stdout);
    println!("Number of CPUs: {}", nproc.trim());

    let output = Command::new("free").arg("-h").output().expect("Failed to Run Command.");
    let free = String::from_utf8_lossy(&output.stdout);
    let mem_total = free.lines().skip(1).next().unwrap().split_whitespace().nth(1).unwrap();
    println!("Total Memory: {}", mem_total);

    let output = Command::new("df")
        .arg("-h")
        .arg("/")
        .output()
        .expect("Failed to run command.");

    let df = String::from_utf8_lossy(&output.stdout);
    let disk_space_line = df.lines().skip(1).next().unwrap();
    let disk_space_parts: Vec<&str> = disk_space_line.split_whitespace().collect();
    let disk_usage = disk_space_parts[2];
    let total_space = disk_space_parts[1];
    let available_space = disk_space_parts[3];

    println!("Disk Usage: {}", disk_usage);
    println!("Total Disk Space: {}", total_space);
    println!("Available Disk Space: {}", available_space);

    let output = Command::new("sh")
        .arg("-c")
        .arg("lsmod | grep -o '^[^ ]*' | grep -E '^(amdgpu|nvidia|nouveau|radeon)'")
        .output()
        .expect("GPU Driver: Failed to run command.");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        let drivers: Vec<&str> = stdout.trim().split('\n').collect();
        let drivers_combined = drivers.join(", ");
        println!("GPU Drivers: {}", drivers_combined);
    } else {
        println!("GPU Driver: not found. {}", stderr);
    }
    let output = Command::new("lspci").arg("-v").output().expect("Failed to Run Command.");
    let lspci = String::from_utf8_lossy(&output.stdout);
    let mut gpu_found = false;
    for line in lspci.lines() {
        if line.contains("VGA compatible controller") {
            let gpu_model_start = line.find(": ").unwrap() + 2;
            let gpu_model = &line[gpu_model_start..];
            println!("GPU Model: {}", gpu_model);
            gpu_found = true;
            break;
        }
    }
    if !gpu_found {
        println!("GPU Model: not found.")
    }
    if let Ok(wayland_display) = env::var("WAYLAND_DISPLAY") {
        println!("Display Server: Wayland ({})", wayland_display);
    } else {
        println!("Display Server: Xorg");
    };
}
