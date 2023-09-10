use std::process::Command;
use std::fs;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    if let Ok(username) = env::var("USER") { // USERNAME
        println!("   Username: {}", username);
    } else {
        println!("   Username: not found.");
    } // USERNAME
    if let Ok(os_release) = fs::read_to_string("/etc/os-release") { // OS
        let mut distro_name = None;
        for line in os_release.lines() {
            if line.starts_with("ID=") {
                distro_name = Some(line.replace("ID=", ""));
                break;
            }
        }
        if let Some(name) = distro_name {
            println!("   Linux Distribution: {}", name);
        } else {
            println!("   Linux Distribution: not found.");
        }
    } // OS
    let output = Command::new("uname") // KERNEL
    .arg("-r")
    .output()
    .expect("   Kernel Version: not found.");
    let kernel_version = String::from_utf8_lossy(&output.stdout);
    println!("   Kernel Version: {}", kernel_version.trim()); // KERNEL
    if let Ok(file) = File::open("/proc/cpuinfo") { // CPU MODEL
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("model name") {
                    let model_name = line.split(":").nth(1).map(|s| s.trim());
                    if let Some(model_name) = model_name {
                        println!("╭─ CPU Model: {}", model_name);
                        break;
                    }
                }
            }
        }
    } // CPU MODEL
    let output = Command::new("nproc").output().expect("╰─➤ Number of CPUs: Failed to Run Command."); // NUMBER CPUS
    let nproc: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    println!("╰─➤ Number of CPUs: {}", nproc.trim());// NUMBER CPUS
    let output = Command::new("lspci").arg("-v").output().expect("╭─ GPU Model: Failed to Run Command."); // GPU MODEL
    let lspci = String::from_utf8_lossy(&output.stdout);
    let mut gpu_found = false;
    for line in lspci.lines() {
        if line.contains("VGA compatible controller") {
            let gpu_model_start = line.find(": ").unwrap() + 2;
            let gpu_model = &line[gpu_model_start..];
            println!("╭─ GPU Model: {}", gpu_model);
            gpu_found = true;
            break;
        }
    }
    if !gpu_found {
        println!("╭─ GPU Model: not found.")
    } // GPU MODEL
      let output = Command::new("sh") // GPU DRIVER
        .arg("-c")
        .arg("lsmod | grep -o '^[^ ]*' | grep -E '^(amdgpu|nvidia|nouveau|radeon)'")
        .output()
        .expect("╰─➤ GPU Driver: Failed to run command.");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stdout.is_empty() {
        let drivers: Vec<&str> = stdout.trim().split('\n').collect();
        let drivers_combined = drivers.join(", ");
        println!("╰─➤ GPU Driver: {}", drivers_combined);
    } else {
        println!("╰─➤ GPU Driver: not found. {}", stderr);
    } // GPU DRIVER
    let output = Command::new("free").arg("-h").output().expect("   Total Memory: Failed to Run Command."); // MEM TOTAL
    let free = String::from_utf8_lossy(&output.stdout);
    let mem_total = free.lines().skip(1).next().unwrap().split_whitespace().nth(1).unwrap();
    println!("   Total Memory: {}", mem_total); // MEM TOTAL}
    }
