use std::io::{self, BufRead};

fn is_safe(report: &[i32]) -> Result<(), String> {
    if report.len() < 2 {
        return Err("Too short to determine direction".into());
    }

    let mut direction = report[1] - report[0];
    if direction == 0 || direction.abs() > 3 {
        return Err(format!("Invalid first step: {}", direction));
    }

    let mut increasing = direction > 0;

    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];
        if diff == 0 {
            return Err(format!("Equal values at positions {} and {}", i - 1, i));
        }
        if diff.abs() > 3 {
            return Err(format!("Jump too large between positions {} and {}: {}", i - 1, i, diff));
        }
        if (diff > 0) != increasing {
            return Err(format!("Direction change at position {}: {}", i, diff));
        }
    }

    Ok(())
}

fn is_safe_with_dampener(report: &[i32]) -> bool {
    if is_safe(report).is_ok() {
        return true;
    }

    for i in 0..report.len() {
        let mut modified = report.to_vec();
        modified.remove(i);
        if is_safe(&modified).is_ok() {
            return true;
        }
    }

    false
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut safe_count = 0;
    let mut unsafe_count = 0;

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        if is_safe_with_dampener(&levels) {
            println!("Report {}: SAFE", i + 1);
            safe_count += 1;
        } else {
            println!("Report {}: UNSAFE", i + 1);
            unsafe_count += 1;
        }
    }

    println!("\nSummary:");
    println!("Safe reports: {}", safe_count);
    println!("Unsafe reports: {}", unsafe_count);
}
