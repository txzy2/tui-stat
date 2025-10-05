//! Provides system information gathering functionality.
//! This module uses the sysinfo crate to collect information about the system's 
//! memory usage, CPU information, and other hardware details.

use crate::{
    logger::{self, log_once},
    types::{CpuInfo, SystemData},
};

/// Conversion factor from bytes to gigabytes
const BYTES_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;

/// Represents a system information collector that provides access to hardware and memory information.
/// 
/// This struct wraps the sysinfo::System struct and provides methods for retrieving
/// system information such as memory usage, CPU details, and other hardware specs.
#[derive(Debug)]
pub struct System {
    system: sysinfo::System,
}

impl System {
    /// Creates a new System instance with all system information initialized.
    /// 
    /// This function initializes a sysinfo::System object with all available information
    /// and refreshes the memory information for immediate access.
    /// 
    /// # Returns
    /// 
    /// A new instance of System with all system information loaded.
    pub fn new() -> Self {
        let mut system = sysinfo::System::new_all();

        system.refresh_memory();
        Self { system }
    }

    /// Retrieves comprehensive system information including memory and CPU details.
    /// 
    /// This function refreshes system memory information and calculates various
    /// memory-related metrics including total, used, and available memory in GB,
    /// as well as memory usage percentage. It also gathers CPU information.
    /// 
    /// # Returns
    /// 
    /// A SystemData struct containing memory and CPU information.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// let mut system = System::new();
    /// let info = system.get_info();
    /// println!("Memory usage: {}%", info.usage_memory);
    /// ```
    pub fn get_info(&mut self) -> SystemData {
        log_once("get_info init", |msg| logger::info(msg));
        self.system.refresh_memory();

        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let available_memory = self.system.available_memory();

        let usage_percent = if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        };

        let cpu = self.get_cpu_data().unwrap_or_default();

        SystemData {
            total_memory: bytes_to_gb(total_memory),
            used_memory: bytes_to_gb(used_memory),
            available_memory: bytes_to_gb(available_memory),
            usage_memory: usage_percent,
            cpu,
        }
    }

    /// Retrieves CPU information from the system.
    /// 
    /// This function extracts information about the first CPU in the system,
    /// including the total number of CPUs, the frequency of the first CPU,
    /// and the CPU brand/model string.
    /// 
    /// # Returns
    /// 
    /// A Result containing either CpuInfo with CPU details or an error string
    /// if no CPU information is available.
    fn get_cpu_data(&self) -> Result<CpuInfo, String> {
        if let Some(cpu) = self.system.cpus().first() {
            logger::log_once(format!("CPU info {:?}", cpu), |msg| logger::info(msg));

            Ok(CpuInfo {
                len: self.system.cpus().len(),
                frequency: cpu.frequency(),
                brand: cpu.brand().to_string(),
            })
        } else {
            Err("Cpu information not found".to_string())
        }
    }
}

impl Default for System {
    /// Provides a default System instance using the new() method.
    /// 
    /// This allows creating a System instance using Default::default().
    fn default() -> Self {
        Self::new()
    }
}

/// Converts a value in bytes to gigabytes.
/// 
/// This function takes a value in bytes and converts it to gigabytes using
/// the BYTES_TO_GB constant which represents the number of bytes in a gigabyte.
/// 
/// # Arguments
/// 
/// * `bytes` - The value in bytes to convert
/// 
/// # Returns
/// 
/// The corresponding value in gigabytes as a floating point number.
fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / BYTES_TO_GB
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_bytes_to_gb_conversion() {
//         // 1 GB = 1073741824 bytes
//         assert_eq!(bytes_to_gb(1073741824), 1.0);
//
//         // 2 GB
//         assert_eq!(bytes_to_gb(2147483648), 2.0);
//
//         // 0.5 GB
//         assert_eq!(bytes_to_gb(536870912), 0.5);
//
//         // 0 bytes
//         assert_eq!(bytes_to_gb(0), 0.0);
//     }
//
//     #[test]
//     fn test_ram_data_structure() {
//         let mut ram = System::new();
//         let data = ram.get_info();
//
//         // Проверяем, что значения находятся в разумных пределах
//         assert!(data.total_memory > 0.0, "Total memory should be positive");
//         assert!(
//             data.used_memory >= 0.0,
//             "Used memory should be non-negative"
//         );
//         assert!(
//             data.available_memory >= 0.0,
//             "Available memory should be non-negative"
//         );
//         assert!(
//             data.usage_memory >= 0.0 && data.usage_memory <= 100.0,
//             "Usage percentage should be between 0 and 100, got {}",
//             data.usage_memory
//         );
//     }
//
//     #[test]
//     fn test_ram_usage_calculation() {
//         let mut ram = System::new();
//         let data = ram.get_info();
//
//         // Проверяем формулу: used + available должно примерно равняться total
//         let sum = data.used_memory + data.available_memory;
//         let diff = (sum - data.total_memory).abs();
//
//         // Допускаем небольшую погрешность (например, 0.1 GB)
//         assert!(
//             diff < 0.1,
//             "Sum of used ({}) and available ({}) should approximately equal total ({})",
//             data.used_memory,
//             data.available_memory,
//             data.total_memory
//         );
//     }
//
//     #[test]
//     fn test_ram_default() {
//         let ram1 = System::default();
//         let ram2 = System::new();
//
//         // Оба способа создания должны работать
//         assert!(format!("{:?}", ram1).contains("System"));
//         assert!(format!("{:?}", ram2).contains("System"));
//     }
// }
