mod data;

use std::collections::HashMap;
use std::io::{self, Write};
use data::{read_employment_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "/opt/app-root/src/homeworks/project/prices.csv";

    let country_avg = read_employment_data(file_path)?;
    let countries: Vec<String> = country_avg.keys().cloned().collect();
    let lower_to_original: HashMap<String, String> = countries
        .iter()
        .map(|c| (c.to_lowercase(), c.clone()))
        .collect();
 
    // Prompt user to enter a country and display its centrality metrics
    print!("Enter a country: ");
    io::stdout().flush()?;
    let mut input_country = String::new();
    io::stdin().read_line(&mut input_country)?;
    let input_country = input_country.trim().to_lowercase();

    if let Some(original_name) = lower_to_original.get(&input_country) {
        let country_index = countries.iter().position(|c| c == original_name).unwrap();
        println!("\n--- Selected Country Centrality Report ---");
        println!("Country: {}", original_name);
        println!("Degree Centrality: {} (Global Avg: {:.2})", degree[country_index], avg_degree);
        println!("Closeness Centrality: {:.4} (Global Avg: {:.4})", closeness[country_index], avg_closeness);
        println!("Betweenness Centrality: {:.4} (Global Avg: {:.4})", betweenness[country_index], avg_betweenness);
    } else {
        println!("Country '{}' is not in the list.", input_country);
    }

    Ok(())
}
