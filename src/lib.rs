use std::fs::File;
use std::io::{self, Read};
use std::process::Command;

/// Returns the Linux kernel version as a String.
pub fn get_kernel_version() -> Option<String> {
    let output = Command::new("uname").arg("-r").output().ok()?;
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Returns the system uptime in seconds as a f64.
pub fn get_system_uptime() -> io::Result<f64> {
    let mut file = File::open("/proc/uptime")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    content.split_whitespace()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to parse /proc/uptime"))
        .and_then(|s| s.parse::<f64>().map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid uptime format")))
}

/// Returns the available memory in kilobytes as a u64.
pub fn get_available_memory() -> io::Result<u64> {
    let mut file = File::open("/proc/meminfo")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    for line in content.lines() {
        if line.starts_with("MemAvailable:") {
            return line.split_whitespace()
                .nth(1)
                .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to parse available memory"))
                .and_then(|s| s.parse::<u64>().map_err(|_| io::Error::new(io::ErrorKind::Other, "Invalid memory format")));
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "MemAvailable not found in /proc/meminfo"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_kernel_version() {
        let version = get_kernel_version();
        assert!(version.is_some(), "Kernel version should not be None");
        println!("Kernel Version: {:?}", version);
    }

    #[test]
    fn test_get_system_uptime() {
        let uptime = get_system_uptime();
        assert!(uptime.is_ok(), "Failed to get system uptime");
        println!("System Uptime: {:.2} seconds", uptime.unwrap());
    }

    #[test]
    fn test_get_available_memory() {
        let memory = get_available_memory();
        assert!(memory.is_ok(), "Failed to get available memory");
        println!("Available Memory: {} kB", memory.unwrap());
    }
}