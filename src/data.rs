// This module handles the parsing and processing of prices
// data from a TSV file. It provides:
//
// - `EmploymentRecord`: a struct to deserialize each data row.
// - `clean_value`: cleans and converts string values to f64.
// - `read_employment_data`: reads the TSV, cleans the data, 
//   aggregates it by country, and returns the average price
//   value for each country as a HashMap.
use std::collections::HashMap;
use csv::ReaderBuilder;
use serde::Deserialize;

// Defines a struct to represent each price data record from the CSV file
#[derive(Debug, Deserialize)]
pub struct EmploymentRecord {
    #[serde(rename = "Country or Area")]
    pub country: String,
    #[serde(rename = "Value")]
    pub value: Option<String>,
}

// Converts a possibly formatted string value into an Option<f64>
pub fn clean_value(value: &Option<String>) -> Option<f64> {
    value.as_ref()
        .map(|v| v.replace(',', "")) // Remove commas from numeric strings
        .and_then(|v| v.parse::<f64>().ok()) // Try parsing to f64
}

pub fn read_employment_data(file_path: &str) -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    // Read the TSV file
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_path(file_path)?;

    let mut country_totals: HashMap<String, Vec<f64>> = HashMap::new();
    // Parse and clean data, storing price values by country
    for result in rdr.deserialize() {
        let record: Result<EmploymentRecord, _> = result;
        if let Ok(rec) = record {
            if let Some(val) = clean_value(&rec.value) {
                country_totals.entry(rec.country.clone()).or_default().push(val);
            }
        }
    }
    // Compute average price per country
    let mut country_avg = HashMap::new();
    for (country, values) in country_totals {
        let avg = values.iter().sum::<f64>() / values.len() as f64;
        country_avg.insert(country, avg);
    }

    Ok(country_avg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Test the clean_value function with different input formats
    #[test]
    fn test_clean_value() {
        assert_eq!(clean_value(&Some("1,000.0".to_string())), Some(1000.0));
        assert_eq!(clean_value(&Some("2.5".to_string())), Some(2.5));
        assert_eq!(clean_value(&Some("invalid".to_string())), None);
        assert_eq!(clean_value(&None), None);
    }

    // Test the read_employment_data function by writing a temporary file and verifying averages
    #[test]
    fn test_read_employment_data_manual_file() {
        let test_path = "test_data.tsv";
        let content = "Country or Area\tValue\nUSA\t1,000.0\nUSA\t2,000.0\nCanada\t1,500.0\n";
        
        fs::write(test_path, content).expect("Failed to write test file");

        let result = read_employment_data(test_path).expect("Failed to read employment data");

        assert_eq!(result.len(), 2);
        assert!((result["USA"] - 1500.0).abs() < 1e-6);
        assert!((result["Canada"] - 1500.0).abs() < 1e-6);

        fs::remove_file(test_path).expect("Failed to delete test file");
    }
}
