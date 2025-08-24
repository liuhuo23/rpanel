use std::fs::File;
use std::io::{BufRead, BufReader};

/// 物理内存信息
pub struct MemInfo {
    pub total_kb: u64,
    pub used_kb: u64,
    pub usage_ratio: f64, // 0.0~1.0
}

/// 获取物理内存总大小和使用率
pub fn get_mem_info() -> MemInfo {
    let file = File::open("/proc/meminfo");
    let mut total = 0u64;
    let mut free = 0u64;
    let mut available = 0u64;
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("MemTotal:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    total = val.parse().unwrap_or(0);
                }
            } else if line.starts_with("MemFree:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    free = val.parse().unwrap_or(0);
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    available = val.parse().unwrap_or(0);
                }
            }
        }
    }
    // Linux 推荐用 available 计算可用内存
    let used = if available > 0 && available < total {
        total - available
    } else if free < total {
        total - free
    } else {
        0
    };
    let usage_ratio = if total > 0 {
        used as f64 / total as f64
    } else {
        0.0
    };
    MemInfo {
        total_kb: total,
        used_kb: used,
        usage_ratio,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mem_info() {
        let mem_info = get_mem_info();
        println!(
            "Total: {} KB, Used: {} KB, Usage: {:.2}%",
            mem_info.total_kb,
            mem_info.used_kb,
            mem_info.usage_ratio * 100.0
        );
        assert!(mem_info.total_kb > 0);
        assert!(mem_info.used_kb <= mem_info.total_kb);
        assert!(mem_info.usage_ratio >= 0.0 && mem_info.usage_ratio <= 1.0);
    }
}
