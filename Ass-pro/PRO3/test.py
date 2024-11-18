import random
import time
import matplotlib.pyplot as plt
from math import log2

# --- Bottom-Up DP Algorithm ---

def knapsack_bottom_up(weights, values, W):
    n = len(weights)
    dp = [[0 for _ in range(W + 1)] for _ in range(n + 1)]
    for i in range(1, n + 1):
        for w in range(W + 1):
            if weights[i - 1] <= w:
                dp[i][w] = max(dp[i - 1][w], dp[i - 1][w - weights[i - 1]] + values[i - 1])
            else:
                dp[i][w] = dp[i - 1][w]
    return dp[n][W]

# --- Top-Down DP (Memoization) Algorithm ---

def knapsack_top_down(weights, values, W):
    n = len(weights)
    memo = [[-1 for _ in range(W + 1)] for _ in range(n + 1)]

    def knapsack_recursive(i, w):
        if i == 0 or w == 0:
            return 0
        if memo[i][w] != -1:
            return memo[i][w]
        if weights[i - 1] <= w:
            memo[i][w] = max(knapsack_recursive(i - 1, w), knapsack_recursive(i - 1, w - weights[i - 1]) + values[i - 1])
        else:
            memo[i][w] = knapsack_recursive(i - 1, w)
        return memo[i][w]

    return knapsack_recursive(n, W)

# --- Verification Function ---

def verify_knapsack(weights, values, W_values):
    for W in W_values:
        top_down_result = knapsack_top_down(weights, values, W)
        bottom_up_result = knapsack_bottom_up(weights, values, W)
        assert top_down_result == bottom_up_result, f"Mismatch for W = {W}: {top_down_result} != {bottom_up_result}"
    print("Knapsack problem verified for all given capacities.")

# --- Plotting Functions ---

def measure_time_knapsack(knapsack_func, weights, values, W_values):
    times = []
    for W in W_values:
        start_time = time.time()
        knapsack_func(weights, values, W)
        elapsed_time = time.time() - start_time
        times.append(elapsed_time)
    return times

def plot_time_vs_capacity(W_values, bottom_up_times, top_down_times):
    plt.figure(figsize=(10, 5))
    plt.plot(W_values, bottom_up_times, label='Bottom-Up DP', marker='o')
    plt.plot(W_values, top_down_times, label='Top-Down DP', marker='x')
    plt.xlabel('Knapsack Capacity (W)')
    plt.ylabel('Time (seconds)')
    plt.title('Knapsack DP: Time vs Capacity')
    plt.legend()
    plt.grid(True)
    plt.savefig('knapsack_time_vs_capacity.png')
    plt.show()

def plot_pseudopolynomial_behavior(W_values, bottom_up_times, top_down_times):
    bit_lengths = [int(log2(W)) + 1 for W in W_values]  # Calculate bit lengths of W_values
    plt.figure(figsize=(10, 5))
    plt.plot(bit_lengths, bottom_up_times, label='Bottom-Up DP', marker='o')
    plt.plot(bit_lengths, top_down_times, label='Top-Down DP', marker='x')
    plt.xlabel('Bit Length of Knapsack Capacity (log2(W))')
    plt.ylabel('Time (seconds)')
    plt.title('Pseudopolynomial Behavior of Knapsack DP Algorithms')
    plt.legend()
    plt.grid(True)
    plt.savefig('pseudopolynomial_behavior.png')
    plt.show()

def plot_special_inputs_performance(n_values, W, num_runs=5):
    results = {'bottom_up': [], 'top_down': []}
    
    for n in n_values:
        weights = [random.randint(1, 10) for _ in range(n)]
        values = [random.randint(1, 10) for _ in range(n)]
        
        bottom_up_times = []
        top_down_times = []
        
        for _ in range(num_runs):
            start_time = time.time()
            knapsack_bottom_up(weights, values, W)
            bottom_up_times.append(time.time() - start_time)
            
            start_time = time.time()
            knapsack_top_down(weights, values, W)
            top_down_times.append(time.time() - start_time)
        
        results['bottom_up'].append(sum(bottom_up_times) / num_runs)
        results['top_down'].append(sum(top_down_times) / num_runs)
    
    plt.figure(figsize=(10, 5))
    plt.plot(n_values, results['bottom_up'], label='Bottom-Up DP', marker='o')
    plt.plot(n_values, results['top_down'], label='Top-Down DP', marker='x')
    plt.xlabel('Number of Items (n)')
    plt.ylabel('Time (seconds)')
    plt.title(f'Performance of Knapsack DP Algorithms (W = {W})')
    plt.legend()
    plt.grid(True)
    plt.savefig('knapsack_special_inputs_performance.png')
    plt.show()

# --- Main Execution ---

def main():
    weights = [random.randint(1, 10) for _ in range(10)]
    values = [random.randint(1, 10) for _ in range(10)]
    W_values = [2**i for i in range(1, 15)]  # Different values of W to show pseudopolynomial behavior

    # Measure and plot time vs capacity for random inputs
    bottom_up_times = measure_time_knapsack(knapsack_bottom_up, weights, values, W_values)
    top_down_times = measure_time_knapsack(knapsack_top_down, weights, values, W_values)

    plot_time_vs_capacity(W_values, bottom_up_times, top_down_times)
    plot_pseudopolynomial_behavior(W_values, bottom_up_times, top_down_times)

    verify_knapsack(weights, values, W_values)

    # Measure and plot performance on special inputs
    n_values = [10, 20, 30, 40, 50]
    W_special = 100  # Fix a reasonable W value for this experiment
    plot_special_inputs_performance(n_values, W_special, num_runs=5)

if __name__ == "__main__":
    main()
