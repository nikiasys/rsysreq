use std::process::Command;
use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
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

    let output = Command::new("df").arg("-h").output().expect("Failed to Run Command.");
    let df = String::from_utf8_lossy(&output.stdout);
    let total_space = df.lines().skip(1).next().unwrap().split_whitespace().nth(1).unwrap();
    println!("Total Disk Space: {}", total_space);

    let output = Command::new("lspci").arg("-v").output().expect("Failed to Run Command.");
    let lspci = String::from_utf8_lossy(&output.stdout);
    let mut driver_found = false;
    for line in lspci.lines() {
        if line.starts_with("VGA compatible controller:") {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() > 1 {
                println!("GPU Driver: {}", parts[1]);
                driver_found = true;
                break;
            }
        }
    }
    if !driver_found {
        println!("GPU Driver: not found.");
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
    }
}
