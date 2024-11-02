# oxide_linux

A minimalist utility library for fetching common Linux system information in Rust.

## Features
- **Get Linux Kernel Version**: Retrieve the Linux kernel version using `uname`.
- **Get System Uptime**: Fetch system uptime from `/proc/uptime`.
- **Get Available Memory**: Parse `/proc/meminfo` to get available memory in kilobytes.

## Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
oxide_linux = "0.1.0"
```
## Usage
Here’s how to use oxide_linux:
```Rustuse oxide_linux::{get_kernel_version, get_system_uptime, get_available_memory};

fn main() {
    // Get the Linux kernel version
    match get_kernel_version() {
        Some(version) => println!("Kernel Version: {}", version),
        None => println!("Failed to get kernel version"),
    }

    // Get the system uptime
    match get_system_uptime() {
        Ok(uptime) => println!("System Uptime: {:.2} seconds", uptime),
        Err(e) => println!("Error getting uptime: {}", e),
    }

    // Get the available memory
    match get_available_memory() {
        Ok(memory) => println!("Available Memory: {} kB", memory),
        Err(e) => println!("Error getting available memory: {}", e),
    }
}
```
## License
This project is licensed under the MIT OR Apache-2.0 license

##Author
Ben Santora <bensatlantik@gmail.com>