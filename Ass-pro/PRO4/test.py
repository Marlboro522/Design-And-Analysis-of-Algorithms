import pandas as pd
import os

def load_graph(file_path):
    df = pd.read_csv(file_path, header=None)
    return df

def count_nodes_edges(graph):
    num_nodes = graph.shape[0]
    num_edges = (graph != -1).sum().sum() // 2  # Since the graph is undirected
    return num_nodes, num_edges

def process_all_graphs(directory):
    results = []
    for graph_type in ['type_1', 'type_2', 'type_3']:
        type_directory = os.path.join(directory, graph_type)
        for file_name in os.listdir(type_directory):
            if file_name.endswith('.csv'):
                file_path = os.path.join(type_directory, file_name)
                graph = load_graph(file_path)
                num_nodes, num_edges = count_nodes_edges(graph)
                results.append((graph_type, file_name, num_nodes, num_edges))
    return results

# Example usage:
directory = 'graphs'
results = process_all_graphs(directory)
for result in results:
    print(f"Graph Type: {result[0]}, File: {result[1]}, Nodes: {result[2]}, Edges: {result[3]}")
