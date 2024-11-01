use sysinfo::{System, SystemExt};

/// Returns the Linux kernel version as a `String`, or `None` if it fails.
pub fn get_kernel_version() -> Option<String> {
    let sys = System::new_all();
    sys.kernel_version()
}

/// Returns the system uptime in seconds, or an error if it fails.
pub fn get_system_uptime() -> Result<f64, String> {
    let sys = System::new();  // Initialize System
    Ok(sys.uptime() as f64)    // Use instance method correctly
}

/// Returns the available memory in kilobytes, or an error if it fails.
pub fn get_available_memory() -> Result<u64, String> {
    let sys = System::new_all();
    Ok(sys.available_memory())
}
