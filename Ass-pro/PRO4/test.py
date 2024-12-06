import csv
import os
import time
import heapq
import matplotlib.pyplot as plt

# Function to load a graph from a CSV file
def load_graph(file_path):
    with open(file_path, 'r') as f:
        reader = csv.reader(f)
        matrix = [list(map(float, row)) for row in reader]
    return matrix

# Function to count nodes and edges in a graph
def count_nodes_edges(matrix):
    num_nodes = len(matrix)
    num_edges = sum(1 for i in range(num_nodes) for j in range(i + 1, num_nodes) if matrix[i][j] != -1)
    return num_nodes, num_edges

# Function to plot timing results for Prim's and Kruskal's algorithms
def plot_timing_results(timing_results, type_name, output_dir):
    indices = [index for index, _, _ in timing_results]
    prims_times = [prims_time for _, prims_time, _ in timing_results]
    kruskals_times = [kruskals_time for _, _, kruskals_time in timing_results]

    plt.figure()
    plt.plot(indices, prims_times, 'o-', label="Prim's Time")
    plt.plot(indices, kruskals_times, 's-', label="Kruskal's Time")
    plt.title(f"Empirical Timing Analysis of MST Algorithms for {type_name}")
    plt.xlabel("Graph Index")
    plt.ylabel("Time (seconds)")
    plt.legend()
    plot_path = os.path.join(output_dir, f"timing_analysis_{type_name}.png")
    plt.savefig(plot_path)
    plt.close()

# Function to time algorithm execution
def time_algorithm(algorithm, graph):
    start = time.time()
    cost = algorithm(graph)
    end = time.time()
    return cost, end - start

# Prim's and Kruskal's algorithm implementations
def prims_algorithm(matrix):
    num_nodes = len(matrix)
    if num_nodes == 0:
        return 0
    visited = [False] * num_nodes
    min_heap = [(0, 0)]
    total_cost = 0
    while min_heap:
        cost, node = heapq.heappop(min_heap)
        if visited[node]:
            continue
        total_cost += cost
        visited[node] = True
        for neighbor in range(num_nodes):
            if not visited[neighbor] and matrix[node][neighbor] != -1:
                heapq.heappush(min_heap, (matrix[node][neighbor], neighbor))
    return total_cost

def kruskals_algorithm(matrix):
    num_nodes = len(matrix)
    if num_nodes == 0:
        return 0
    edges = []
    parent = list(range(num_nodes))
    rank = [0] * num_nodes
    for i in range(num_nodes):
        for j in range(i + 1, num_nodes):
            if matrix[i][j] != -1:
                edges.append((matrix[i][j], i, j))
    edges.sort()
    total_cost = 0
    for weight, u, v in edges:
        root1 = find(u, parent)
        root2 = find(v, parent)
        if root1 != root2:
            union(root1, root2, parent, rank)
            total_cost += weight
    return total_cost

def find(n, parent):
    if parent[n] != n:
        parent[n] = find(parent[n], parent)
    return parent[n]

def union(root1, root2, parent, rank):
    if rank[root1] > rank[root2]:
        parent[root2] = root1
    elif rank[root1] < rank[root2]:
        parent[root1] = root2
    else:
        parent[root2] = root1
        rank[root1] += 1

# Main script execution
if __name__ == "__main__":
    graphs_dir = 'graphs'
    output_dir = 'output_plots'
    os.makedirs(output_dir, exist_ok=True)

    for type_dir in ['type_1', 'type_2', 'type_3']:
        graph_files = [os.path.join(graphs_dir, type_dir, f) for f in os.listdir(os.path.join(graphs_dir, type_dir)) if f.endswith('.csv')]
        timing_results = []
        index = 1

        for file in graph_files:
            graph = load_graph(file)
            num_nodes, num_edges = count_nodes_edges(graph)
            prims_cost, prims_time = time_algorithm(prims_algorithm, graph)
            kruskals_cost, kruskals_time = time_algorithm(kruskals_algorithm, graph)
            timing_results.append((index, prims_time, kruskals_time))
            print(f"Graph: {os.path.basename(file)} | Type: {type_dir} | Nodes: {num_nodes} | Edges: {num_edges} | Prim's Cost: {prims_cost} | Prim's Time: {prims_time:.4f} s | Kruskal's Cost: {kruskals_cost} | Kruskal's Time: {kruskals_time:.4f} s")
            index += 1

        # Plot timing results for the current type
        plot_timing_results(timing_results, type_dir, output_dir)