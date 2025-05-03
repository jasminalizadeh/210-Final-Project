// This module provides functions to compute centrality metrics
// for nodes in a graph represented as an adjacency matrix.
//
// Includes:
// - Degree Centrality: Number of direct neighbors per node.
// - Closeness Centrality: Inverse of the average shortest path 
//   length from a node to all others using BFS.
// - Betweenness Centrality: Measures how often a node appears 
//   on shortest paths between other nodes.
use std::collections::VecDeque;

// Computes degree centrality: number of direct connections each node has
pub fn degree_centrality(matrix: &Vec<Vec<i32>>) -> Vec<usize> {
    matrix.iter()
        .map(|row| row.iter().filter(|&&x| x == 1).count())
        .collect()
}

// Performs Breadth-First Search to compute shortest paths from a starting node
fn bfs_shortest_path(matrix: &Vec<Vec<i32>>, start: usize) -> Vec<Option<usize>> {
    let n = matrix.len();
    let mut dist = vec![None; n]; // Distance from start to each node
    let mut queue = VecDeque::new();

    dist[start] = Some(0);
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        for (v, &connected) in matrix[u].iter().enumerate() {
            if connected == 1 && dist[v].is_none() {
                dist[v] = dist[u].map(|d| d + 1);
                queue.push_back(v);
            }
        }
    }

    dist
}

// Computes closeness centrality: inverse of the average shortest path length
pub fn closeness_centrality(matrix: &Vec<Vec<i32>>) -> Vec<f64> {
    let n = matrix.len();
    let mut centrality = vec![0.0; n];

    for i in 0..n {
        let dists = bfs_shortest_path(matrix, i);
        let sum_distances: usize = dists.iter().filter_map(|&d| d).filter(|&d| d > 0).sum();

        if sum_distances > 0 {
            centrality[i] = (n as f64 - 1.0) / sum_distances as f64;
        }
    }

    centrality
}

// Computes betweenness centrality
pub fn betweenness_centrality(matrix: &Vec<Vec<i32>>) -> Vec<f64> {
    let n = matrix.len();
    let mut centrality = vec![0.0; n];

    for s in 0..n {
        let mut stack = Vec::new();
        let mut p: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut sigma = vec![0; n];// Number of shortest paths to each node
        let mut dist = vec![-1; n];// Distance from source
        sigma[s] = 1;
        dist[s] = 0;

        let mut queue = VecDeque::new();
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            for (w, &connected) in matrix[v].iter().enumerate() {
                if connected == 1 {
                    if dist[w] < 0 {
                        queue.push_back(w);
                        dist[w] = dist[v] + 1;
                    }
                    if dist[w] == dist[v] + 1 {
                        sigma[w] += sigma[v];
                        p[w].push(v);
                    }
                }
            }
        }

        let mut delta = vec![0.0; n]; // Dependency scores
        while let Some(w) = stack.pop() {
            for &v in &p[w] {
                delta[v] += (sigma[v] as f64 / sigma[w] as f64) * (1.0 + delta[w]);
            }
            if w != s {
                centrality[w] += delta[w];
            }
        }
    }
    // Divide by 2 for undirected graphs
    for c in &mut centrality {
        *c /= 2.0;
    }

    centrality
}


// Unit tests to verify correctness of centrality calculations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_centrality() {
        let matrix = vec![
            vec![0, 1, 1],
            vec![1, 0, 0],
            vec![1, 0, 0],
        ];
        let degrees = degree_centrality(&matrix);
        assert_eq!(degrees, vec![2, 1, 1]);
    }

    #[test]
    fn test_closeness_centrality() {
        let matrix = vec![
            vec![0, 1, 1],
            vec![1, 0, 1],
            vec![1, 1, 0],
        ];
        let closeness = closeness_centrality(&matrix);
        let expected = vec![1.0, 1.0, 1.0];
        for (c, e) in closeness.iter().zip(expected.iter()) {
            assert!((c - e).abs() < 1e-6);
        }
    }

    #[test]
    fn test_betweenness_centrality() {
        let matrix = vec![
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 1],
            vec![0, 0, 1, 0],
        ];
        let bc = betweenness_centrality(&matrix);
        assert!(bc[1] > bc[0]); // Node 1 should have higher centrality than 0
        assert!(bc[2] > bc[3]); // Node 2 is more central than 3
    }
}
