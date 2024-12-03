use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input(file_path: &str) -> Result<Vec<Vec<i64>>, String> {
    let file = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;

    let reader = BufReader::new(file);
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read a line: {}", e))?;

        let numbers = line
            .split_whitespace()
            .map(|num| {
                num.parse::<i64>()
                    .map_err(|e| format!("Error parsing number {}: {}", num, e))
            })
            .collect::<Result<Vec<i64>, String>>()?;

        result.push(numbers);
    }

    Ok(result)
}

fn is_report_safe(report: &Vec<i64>) -> bool {
    let mut is_ascending = true;
    let mut is_descending = true;

    for i in 0..report.len() - 1 {
        if report[i] >= report[i + 1] {
            is_ascending = false;
        }

        if report[i] <= report[i + 1] {
            is_descending = false;
        }
    }

    for i in 0..report.len() - 1 {
        let diff = (report[i + 1] - report[i]).abs();
        if diff != 1 && diff != 2 && diff != 3 {
            return false;
        }
    }

    is_ascending || is_descending
}

fn is_report_safe_without_one_level(report: &Vec<i64>) -> bool {
    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);

        if is_report_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn count_safe_reports(reports: &Vec<Vec<i64>>) -> i64 {
    let mut count: i64 = 0;

    for report in reports {
        if is_report_safe(report) || is_report_safe_without_one_level(report) {
            count += 1;
        }
    }

    count
}

fn main() {
    match get_input("./src/input.txt") {
        Ok(numbers) => {
            let result = count_safe_reports(&numbers);
            println!("Safe report count: {}", result);
        }
        Err(e) => eprintln!("Something wrong: {}", e),
    }
}
