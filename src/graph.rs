// This module constructs a graph based on price similarity between countries. 
// Each node represents a country, and an edge is created between two countries if their average price 
// values are within a specified threshold.
//
// Provided:
// - build_adjacency_matrix: Creates an adjacency matrix where 
//    edges represent price value similarity.
use std::collections::HashMap;

// Builds an adjacency matrix connecting countries with average price values within a certain threshold of difference
pub fn build_adjacency_matrix(countries: Vec<String>, country_avg: &HashMap<String, f64>, threshold: f64) -> Vec<Vec<i32>> {
    let mut matrix = vec![vec![0; countries.len()]; countries.len()];

    for (i, c1) in countries.iter().enumerate() {
        for (j, c2) in countries.iter().enumerate() {
            if i != j {
                let diff = (country_avg[c1] - country_avg[c2]).abs();
                if diff < threshold {
                    matrix[i][j] = 1;
                }
            }
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test adjacency matrix with countries having price averages within and outside the threshold
    #[test]
    fn test_build_adjacency_matrix_basic() {
        let mut averages = HashMap::new();
        averages.insert("USA".to_string(), 1000.0);
        averages.insert("Canada".to_string(), 1050.0);
        averages.insert("Mexico".to_string(), 1300.0);

        let countries = vec!["USA".to_string(), "Canada".to_string(), "Mexico".to_string()];

        let matrix = build_adjacency_matrix(countries.clone(), &averages, 100.0);

        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);

        // USA and Canada are within 50 of each other → connected
        assert_eq!(matrix[0][1], 1);
        assert_eq!(matrix[1][0], 1);

        // USA and Mexico differ by 300 → not connected
        assert_eq!(matrix[0][2], 0);
        assert_eq!(matrix[2][0], 0);

        // Canada and Mexico differ by 250 → not connected
        assert_eq!(matrix[1][2], 0);
        assert_eq!(matrix[2][1], 0);

        // Diagonal should always be 0 (no self-connections)
        assert_eq!(matrix[0][0], 0);
        assert_eq!(matrix[1][1], 0);
        assert_eq!(matrix[2][2], 0);
    }

    // Test with a high threshold where all nodes should be connected
    #[test]
    fn test_build_adjacency_matrix_all_connected() {
        let mut averages = HashMap::new();
        averages.insert("A".to_string(), 100.0);
        averages.insert("B".to_string(), 200.0);
        averages.insert("C".to_string(), 300.0);

        let countries = vec!["A".to_string(), "B".to_string(), "C".to_string()];

        let matrix = build_adjacency_matrix(countries, &averages, 1000.0);

        for i in 0..3 {
            for j in 0..3 {
                if i != j {
                    assert_eq!(matrix[i][j], 1);
                } else {
                    assert_eq!(matrix[i][j], 0);
                }
            }
        }
    }
}