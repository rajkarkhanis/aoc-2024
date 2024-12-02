use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_input(file_path: &str) -> Result<(Vec<i64>, Vec<i64>), String> {
    let file =
        File::open(file_path).map_err(|e| format!("Failed to open file '{}': {}", file_path, e))?;

    let reader = BufReader::new(file);
    let mut left_column: Vec<i64> = Vec::with_capacity(1000);
    let mut right_column: Vec<i64> = Vec::with_capacity(1000);

    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                let parts: Vec<&str> = line.split("   ").collect();

                if parts.len() != 2 {
                    eprintln!("Malormatted line, doesn't have exactly two parts: {}", line);
                    continue;
                }

                let left_result = parts[0].trim().parse::<i64>();
                let right_result = parts[1].trim().parse::<i64>();

                match (left_result, right_result) {
                    (Ok(left), Ok(right)) => {
                        left_column.push(left);
                        right_column.push(right);
                    }
                    (Err(e), _) => eprintln!("Error parsing left column of line {}: {}", line, e),
                    (_, Err(e)) => eprintln!("Error parsing right column of line{}: {}", line, e),
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }

    Ok((left_column, right_column))
}

fn calculate_distance(left_column: &[i64], right_column: &[i64]) -> Result<i64, String> {
    if left_column.len() != right_column.len() {
        return Err("Vectors have different lengths, cannot calculate distance.".to_string());
    }

    let mut left_column_sorted = left_column.to_vec();
    let mut right_column_sorted = right_column.to_vec();
    left_column_sorted.sort_by(|a, b| a.cmp(b)); // Use `cmp` for i64 sorting
    right_column_sorted.sort_by(|a, b| a.cmp(b));

    let mut total_distance = 0;
    for (left, right) in left_column_sorted.iter().zip(right_column_sorted.iter()) {
        total_distance += (left - right).abs();
    }

    Ok(total_distance)
}

fn calculate_similarity_score(left_column: &[i64], right_column: &[i64]) -> Result<i64, String> {
    if left_column.len() != right_column.len() {
        return Err("Vectors have different lengths, cannot calculate distance.".to_string());
    }

    let mut right_count: HashMap<i64, i64> = HashMap::new();
    for &num in right_column {
        *right_count.entry(num).or_insert(0) += 1;
    }

    let mut similarity_score: i64 = 0;
    for &left_num in left_column {
        if let Some(&count) = right_count.get(&left_num) {
            similarity_score += left_num * count;
        }
    }

    Ok(similarity_score)
}

fn main() {
    match get_input("./src/input.txt") {
        Ok((left_column, right_column)) => {
            // Calculate total distance
            match calculate_distance(&left_column, &right_column) {
                Ok(total_distance) => {
                    println!("Total Distance: {}", total_distance);
                }
                Err(e) => {
                    eprintln!("Error calculating distance: {}", e);
                }
            }

            // Calculate similarity score
            match calculate_similarity_score(&left_column, &right_column) {
                Ok(similarity_score) => {
                    println!("Similarity Score: {}", similarity_score);
                }
                Err(e) => {
                    eprintln!("Error calculating similarity score: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
