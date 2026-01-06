use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn load_csv_data<P>(path: P) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let values: Vec<String> = line
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        data.push(values);
    }

    Ok(data)
}