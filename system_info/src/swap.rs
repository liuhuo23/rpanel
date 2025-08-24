use std::fs::File;
use std::io::{BufRead, BufReader};

/// 交换空间信息
pub struct SwapInfo {
    pub total_kb: u64,
    pub used_kb: u64,
    pub usage_ratio: f64, // 0.0~1.0
}

/// 获取交换空间总大小和使用率
pub fn get_swap_info() -> SwapInfo {
    let file = File::open("/proc/meminfo");
    let mut total = 0u64;
    let mut free = 0u64;
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("SwapTotal:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    total = val.parse().unwrap_or(0);
                }
            } else if line.starts_with("SwapFree:") {
                if let Some(val) = line.split_whitespace().nth(1) {
                    free = val.parse().unwrap_or(0);
                }
            }
        }
    }
    let used = if free < total { total - free } else { 0 };
    let usage_ratio = if total > 0 {
        used as f64 / total as f64
    } else {
        0.0
    };
    SwapInfo {
        total_kb: total,
        used_kb: used,
        usage_ratio,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_swap_info() {
        let info = get_swap_info();
        // swap 总量大于等于0，已用大于等于0，使用率在0~1之间
        assert!(info.usage_ratio >= 0.0 && info.usage_ratio <= 1.0);
        println!(
            "swap total: {} KB, used: {} KB, usage: {:.2}%",
            info.total_kb,
            info.used_kb,
            info.usage_ratio * 100.0
        );
    }
}
