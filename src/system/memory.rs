use crate::{
    logger::{self, log_once},
    types::{CpuInfo, SystemData},
};

const BYTES_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;

#[derive(Debug)]
pub struct System {
    system: sysinfo::System,
}

//TODO: Добавить отправку загрузки CPU
impl System {
    pub fn new() -> Self {
        let mut system = sysinfo::System::new_all();

        system.refresh_memory();
        Self { system }
    }

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

    fn get_cpu_data(&self) -> Result<CpuInfo, String> {
        logger::log_once(format!("CPU len {}", self.system.cpus().len()), |msg| {
            logger::info(msg)
        });

        if let Some(cpu) = self.system.cpus().first() {
            logger::log_once(
                format!(
                    "CPU brand: {}\nCPU frequency: {} MHz",
                    cpu.brand(),
                    cpu.frequency()
                ),
                |msg| logger::info(msg),
            );

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
    fn default() -> Self {
        Self::new()
    }
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / BYTES_TO_GB
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_gb_conversion() {
        // 1 GB = 1073741824 bytes
        assert_eq!(bytes_to_gb(1073741824), 1.0);

        // 2 GB
        assert_eq!(bytes_to_gb(2147483648), 2.0);

        // 0.5 GB
        assert_eq!(bytes_to_gb(536870912), 0.5);

        // 0 bytes
        assert_eq!(bytes_to_gb(0), 0.0);
    }

    #[test]
    fn test_ram_data_structure() {
        let mut ram = System::new();
        let data = ram.get_info();

        // Проверяем, что значения находятся в разумных пределах
        assert!(data.total_memory > 0.0, "Total memory should be positive");
        assert!(
            data.used_memory >= 0.0,
            "Used memory should be non-negative"
        );
        assert!(
            data.available_memory >= 0.0,
            "Available memory should be non-negative"
        );
        assert!(
            data.usage_memory >= 0.0 && data.usage_memory <= 100.0,
            "Usage percentage should be between 0 and 100, got {}",
            data.usage_memory
        );
    }

    #[test]
    fn test_ram_usage_calculation() {
        let mut ram = System::new();
        let data = ram.get_info();

        // Проверяем формулу: used + available должно примерно равняться total
        let sum = data.used_memory + data.available_memory;
        let diff = (sum - data.total_memory).abs();

        // Допускаем небольшую погрешность (например, 0.1 GB)
        assert!(
            diff < 0.1,
            "Sum of used ({}) and available ({}) should approximately equal total ({})",
            data.used_memory,
            data.available_memory,
            data.total_memory
        );
    }

    #[test]
    fn test_ram_default() {
        let ram1 = System::default();
        let ram2 = System::new();

        // Оба способа создания должны работать
        assert!(format!("{:?}", ram1).contains("System"));
        assert!(format!("{:?}", ram2).contains("System"));
    }
}
