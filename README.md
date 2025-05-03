Global Consumer Price Centrality
Jasmin Alizadeh-Shabdiz
CDSDS 210 – Final Project
A. Overview
This project explores which countries are most structurally central in a global network based on consumer prices. It uses graph theory to calculate degree, closeness, and betweenness centrality—all common metrics in network analysis.
Nodes: Countries


Edges: Drawn between countries with similar average price values


The goal is to understand which countries play central roles in the price similarity network, both regionally and globally.

B. Dataset
Source: UN Data - LABORSTA


File: prices.csv (TSV format)


Size: ~1–2 MB


Note: Due to size, this file is not on GitHub. It should be added manually to the project directory.



C. How It Works
Data Cleaning


Loaded the file using the csv crate with .delimiter(b'\t') for TSV support


Parsed rows into a struct using serde


Removed commas from numeric strings and parsed as floats


Averaged prices per country


Graph Construction


Countries are connected if their average values differ by less than 100.0


This forms an adjacency matrix representing the similarity graph


Centrality Metrics


Degree: Number of direct connections


Closeness: How close a country is to all others on average


Betweenness: How often a country lies on the shortest path between others


User Interaction


Prompts for a country and compares its centrality scores to the global average



D. Code Structure
All code is in main.rs but divided logically:
Data Loading and Cleaning


Adjacency Matrix Creation


Centrality Calculations


Command-line Prompt and Output


Key types and functions:
EmploymentRecord: Struct for parsing rows


clean_value(): Cleans and parses string values


build_adjacency_matrix(): Builds the graph


degree_centrality(), closeness_centrality(), betweenness_centrality(): Centrality functions



E. Tests
Run tests with:
cargo test

Test coverage includes:
Data Cleaning


Handles numeric strings with commas


Validates grouping and averaging


Graph Construction


Checks correct edge creation based on threshold


Confirms symmetric and valid matrices


Centrality Metrics


Degree: Validates connection counting


Closeness: Uses triangle graphs for equal distance


Betweenness: Uses path graphs to verify middle node importance



F. Example Output
Enter a country: Germany

Germany's centrality:
Degree: 82 (Global Avg: 67)
Closeness: 0.0345 (Global Avg: 0.0263)
Betweenness: 0.0921 (Global Avg: 0.0450)

Interpretation: Germany is more central than average across all metrics, indicating its prices are similar to many countries, easily reachable, and influential in connecting 
others.

G. Running the Program
Build:
cargo build

Run:
cargo run

You'll be prompted to enter a country name (e.g., Mexico). Input is case-insensitive. No extra setup or arguments are needed.

H. Notes on AI Assistance
I used ChatGPT to understand how to load and parse TSV files in Rust. It helped me figure out the difference between CSV and TSV parsing and introduced me to using serde for struct 
deserialization. I followed that approach in my implementation.

Note on commit history: Since I was unfamiliar with GitHub workflows, I completed the entire project locally before making any commits. To simulate meaningful progress, I created three 
logical checkpoints by separately staging and committing different components (such as data loading, leaderboard logic, and the write-up). Although the commits were made after development, 
all code was built and tested incrementally throughout the process.
