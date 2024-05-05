### Final Project for DS210: Programming In Rust 

Link to dataset: https://snap.stanford.edu/data/roadNet-CA.html

**Final Project Writeup:**

The primary objectives of this project include implementing graph algorithms such as Dijkstra's algorithm and Breadth-First Search (BFS) to find shortest paths and compute various graph metrics. Additionally, Computing Clustering coefficients and Density of a Subgraph a secondary Objective. 

To Run this code: 
- Download the Dataset (link above)
- Download the project files and make sure that the dataset is in the project files
- Navigate to the project directory in your terminal.
- Compile and execute the code as you would any other rust project
- Follow the user prompts and the code will provide various graph insights. 

The project utilizes real-world graph data stored in a .txt file (roadNet-CA.txt). This file contains information about the road network in California, representing nodes and their connections. The program reads this data to construct a graph for analysis.

Dijkstra's algorithm is employed to find the shortest path between two nodes in the graph. The user provides input for the start and end nodes, and the algorithm computes the shortest distance between them. Error handling ensures the validity of user inputs.

Graph Analysis: 
- Graph density measures the ratio of existing edges to the total possible edges in the graph. The program calculates graph density to provide insights into the connectivity of the graph.
- Clustering coefficient quantifies the degree to which nodes in a graph tend to cluster together. The program finds the clustering coefficient to analyze the local structure of the graph.

To manage computational complexity and derive more insights, a subset of the larger graph is analyzed. Key steps in subgraph analysis include:

Subgraph Creation: A subset of the original graph is created to reduce computation time.
- The size of the subgraph is determined to strike a balance between accuracy and efficiency.

Average Distance Analysis:
- Average distance from a specific node to all other nodes in the subgraph is computed using Dijkstra's algorithm.
- Average distance between all nodes in the subgraph is calculated through BFS.
