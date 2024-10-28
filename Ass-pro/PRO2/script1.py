import random
import matplotlib.pyplot as plt

# Timinng it was connfusing
comparison_count = 0

def insertion_sort(array, l, h):
    """Insertion Sort for a subarray from index l to h."""
    global comparison_count
    for i in range(l + 1, h + 1):
        key = array[i]
        j = i
        while j > l and array[j - 1] > key:
            comparison_count += 1  # Counting the comparison
            array[j] = array[j - 1]
            j -= 1
        array[j] = key
        comparison_count += 1  # Final comparison when while loop ends

def merge(left, right):
    """Merge two sorted subarrays into one sorted array."""
    global comparison_count
    merged = []
    i, j = 0, 0
    while i < len(left) and j < len(right):
        comparison_count += 1  # Counting the comparison
        if left[i] <= right[j]:
            merged.append(left[i])
            i += 1
        else:
            merged.append(right[j])
            j += 1
    return merged + left[i:] + right[j:]

def hybrid_sort(array, k):
    """Hybrid Sort algorithm that uses Merge Sort and Insertion Sort based on threshold K."""
    n = len(array)
    if n <= k:
        insertion_sort(array, 0, n - 1)
    else:
        mid = n // 2
        left, right = array[:mid], array[mid:]
        hybrid_sort(left, k)
        hybrid_sort(right, k)
        array[:] = merge(left, right)

""" Tests for verification of HybridSort with inbuilt algo """

def verify_sort(k_values, n_values, num_tests=5):
    """Verify that HybridSort sorts the array correctly for various K and n values."""
    for k in k_values:
        for n in n_values:
            for _ in range(num_tests):
                array = [random.randint(0, 10000) for _ in range(n)]
                sorted_copy = sorted(array)  # Use Python's sorted to get the correct result
                hybrid_sort(array, k)  # Sort the array using HybridSort
                assert array == sorted_copy, f"Error: HybridSort did not sort correctly for K={k}, n={n}"
    print("HybridSort verified: all tests passed.")

# For plotting using comparison count instead of time

def plot_comparison_counts(k_values, n_values, num_runs=5, sorted_array=False):
    """plot the comparison counts of HybridSort as a function of K and n."""
    results = {k: [] for k in k_values}

    for k in k_values:
        for n in n_values:
            total_comparisons = 0
            for _ in range(num_runs):
                global comparison_count
                comparison_count = 0  # Reset comparison count
                array = list(range(n)) if sorted_array else [random.randint(0, 10000) for _ in range(n)]
                hybrid_sort(array[:], k)
                total_comparisons += comparison_count
            avg_comparisons = total_comparisons / num_runs
            results[k].append(avg_comparisons)
            print(f"HybridSort with K = {k}, n = {n}, {'sorted' if sorted_array else 'random'} array had {avg_comparisons:.2f} comparisons on average")

    for k, comparisons in results.items():
        plt.plot(n_values, comparisons, label=f'K = {k}')
    
    plt.xlabel('Array Length (n)')
    plt.ylabel('Average Comparisons')
    title_suffix = "Sorted Arrays" if sorted_array else "Random Arrays"
    plt.title(f'Average Comparison Count of HybridSort for {title_suffix}')
    plt.legend()
    plt.grid(True)
    filename = f'average_comparisons_{"sorted" if sorted_array else "random"}.png'
    plt.savefig(filename)
    plt.show()

def plot_optimal_k_comparisons(k_values, n_values, num_runs=5, sorted_array=False):
    """plot the optimal K as a function of n based on comparison counts."""
    optimal_k_values = []

    for n in n_values:
        best_comparisons = float('inf')
        best_k = None
        for k in k_values:
            total_comparisons = 0
            for _ in range(num_runs):
                global comparison_count
                comparison_count = 0  # Reset comparison count
                # Use a sorted array if specified, otherwise generate a random array
                array = list(range(n)) if sorted_array else [random.randint(0, 10000) for _ in range(n)]
                hybrid_sort(array[:], k)
                total_comparisons += comparison_count
            avg_comparisons = total_comparisons / num_runs
            if avg_comparisons < best_comparisons:
                best_comparisons = avg_comparisons
                best_k = k
        optimal_k_values.append(best_k)
        print(f"Optimal K for n = {n} ({'sorted' if sorted_array else 'random'} array) is {best_k}")

    # Plotting the optimal K values for each n
    plt.plot(n_values, optimal_k_values, marker='o')
    plt.xlabel('Array Length (n)')
    plt.ylabel('Optimal K')
    title_suffix = "Sorted Arrays" if sorted_array else "Random Arrays"
    plt.title(f'Optimal K as a Function of Array Length n for {title_suffix}')
    plt.grid(True)
    filename = f'optimal_k_comparisons_{"sorted" if sorted_array else "random"}.png'
    plt.savefig(filename)
    plt.show()

def main():
    k_values = [5, 10, 20, 50, 99]
    n_values = [100, 500, 1000, 5000, 10000]

    # Verify the correctness of HybridSort for all K and n values
    print("Verifying correctness of HybridSort...")
    verify_sort(k_values, n_values)

    # Deliverable 2: Average comparisons for random arrays
    print("Plotting average comparisons for random arrays...")
    plot_comparison_counts(k_values, n_values, num_runs=5, sorted_array=False)

    # Deliverable 4: Average comparisons for sorted arrays
    print("Plotting average comparisons for sorted arrays...")
    plot_comparison_counts(k_values, n_values, num_runs=5, sorted_array=True)

    # Deliverable 3: Optimal K as a function of n for random arrays
    print("Plotting optimal K as a function of n (based on comparisons)...")
    plot_optimal_k_comparisons(k_values, n_values, num_runs=5)

    print("Plotting optimal K as a function of n for sorted arrays (based on comparisons)...")
    plot_optimal_k_comparisons(k_values, n_values, num_runs=5, sorted_array=True)

if __name__ == "__main__":
    main()
