import random
import time
import matplotlib.pyplot as plt

def insertion_sort(array, l, h):
    """Insertion sort algorithm"""
    for i in range(l + 1, h + 1):
        key = array[i]
        j = i
        while j > l and array[j - 1] > key:
            array[j] = array[j - 1]
            j -= 1
        array[j] = key

def merge(left, right):
    """Merge two sorted subarrays into one sorted array"""
    merged = []
    i = 0
    j = 0
    while i < len(left) and j < len(right):
        if left[i] <= right[j]:
            merged.append(left[i])
            i += 1
        else:
            merged.append(right[j])
            j += 1
    while i < len(left):
        merged.append(left[i])
        i += 1
    while j < len(right):
        merged.append(right[j])
        j += 1
    return merged

def hybrid_sort(array, l, h, k):
    """Hybrid sort algorithm"""
    if h - l + 1 <= k:
        insertion_sort(array, l, h)
    else:
        mid = l + (h - l) // 2
        hybrid_sort(array, l, mid, k)
        hybrid_sort(array, mid + 1, h, k)
        left = array[l:mid + 1]
        right = array[mid + 1:h + 1]
        merged = merge(left, right)
        array[l:h + 1] = merged

def generate_random_array(n):
    """Generate a random array of size n"""
    return [random.randint(0, 10000) for _ in range(n)]

def generate_sorted_array(n):
    """Generate a sorted array of size n"""
    return list(range(n))

def measure_time(array, k):
    """Measure the time taken to sort the array using hybrid sort with threshold k"""
    start_time = time.time()
    hybrid_sort(array, 0, len(array) - 1, k)
    end_time = time.time()
    return end_time - start_time

def main():
    k_values = [5, 10, 20, 50, 100]
    n_values = [100, 1000, 5000, 10000, 20000]

    # Results dictionaries to store times for different scenarios
    results_random = {}
    results_sorted = {}

    for k in k_values:
        results_random[k] = []
        results_sorted[k] = []
        for n in n_values:
            # Measure time for random arrays
            array = generate_random_array(n)
            duration = measure_time(array, k)
            results_random[k].append(duration)
            print(f"HybridSort with K = {k} and random array of n = {n} took: {duration:.6f} seconds")

            # Measure time for sorted arrays
            array = generate_sorted_array(n)
            duration = measure_time(array, k)
            results_sorted[k].append(duration)
            print(f"HybridSort with K = {k} and sorted array of n = {n} took: {duration:.6f} seconds")

    # Plot results for random arrays
    for k, times in results_random.items():
        plt.plot(n_values, times, label=f'K = {k}')

    plt.xlabel('Array Length (n)')
    plt.ylabel('Time (seconds)')
    plt.title('Hybrid Sort Performance on Random Arrays')
    plt.legend()
    plt.grid(True)
    plt.savefig('hybrid_sort_performance_random.png')
    plt.show()

    # Plot results for sorted arrays
    for k, times in results_sorted.items():
        plt.plot(n_values, times, label=f'K = {k}')

    plt.xlabel('Array Length (n)')
    plt.ylabel('Time (seconds)')
    plt.title('Hybrid Sort Performance on Sorted Arrays')
    plt.legend()
    plt.grid(True)
    plt.savefig('hybrid_sort_performance_sorted.png')
    plt.show()

if __name__ == "__main__":
    main()