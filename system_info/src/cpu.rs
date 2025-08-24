use std::fs::File;
use std::io::{BufRead, BufReader};

/// 读取 /proc/cpuinfo 获取 Linux 下的 CPU 核心数
pub fn count() -> usize {
    let file = File::open("/proc/cpuinfo");
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        let mut count = 0;
        for line in reader.lines().flatten() {
            if line.starts_with("processor") {
                count += 1;
            }
        }
        count
    } else {
        1 // 读取失败时默认返回1
    }
}

/// 读取 /proc/stat 获取 CPU 使用时间
/// 读取 /proc/stat 第一行，返回 (total, idle)
fn get_cpu_times() -> (u64, u64) {
    let file = File::open("/proc/stat");
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            if line.starts_with("cpu ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                // cpu user nice system idle iowait irq softirq steal guest guest_nice
                //  0   1    2    3      4    5      6   7       8     9     10
                if parts.len() >= 5 {
                    let user: u64 = parts[1].parse().unwrap_or(0);
                    let nice: u64 = parts[2].parse().unwrap_or(0);
                    let system: u64 = parts[3].parse().unwrap_or(0);
                    let idle: u64 = parts[4].parse().unwrap_or(0);
                    let iowait: u64 = parts.get(5).and_then(|s| s.parse().ok()).unwrap_or(0);
                    let irq: u64 = parts.get(6).and_then(|s| s.parse().ok()).unwrap_or(0);
                    let softirq: u64 = parts.get(7).and_then(|s| s.parse().ok()).unwrap_or(0);
                    let steal: u64 = parts.get(8).and_then(|s| s.parse().ok()).unwrap_or(0);
                    let total = user + nice + system + idle + iowait + irq + softirq + steal;
                    let idle_all = idle + iowait;
                    return (total, idle_all);
                }
            }
        }
        (0, 0)
    } else {
        (0, 0)
    }
}

/// 获取 CPU 使用量，1000 代表 1 核满载，2核满载为 2000
pub fn usage() -> u32 {
    let (total1, idle1) = get_cpu_times();
    std::thread::sleep(std::time::Duration::from_millis(100));
    let (total2, idle2) = get_cpu_times();
    let total_delta = total2.saturating_sub(total1);
    let idle_delta = idle2.saturating_sub(idle1);
    if total_delta == 0 {
        return 0;
    }
    let usage_ratio = (total_delta - idle_delta) as f64 / total_delta as f64;
    // 1000 代表 1 核满载
    (usage_ratio as f64 * 1000.0).round() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_count() {
        assert_eq!(count(), 16);
    }

    #[test]
    fn test_get_cpu_time() {
        let (total, idle) = get_cpu_times();
        assert!(total >= 0);
        assert!(idle >= 0);
    }
    #[test]
    fn test_usage() {
        let usage = usage();
        println!("{}", usage);
        assert!(usage <= 1000);
    }
}
