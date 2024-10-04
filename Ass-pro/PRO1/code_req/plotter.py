import matplotlib.pyplot as plt
import pickle
import requests
from bs4 import BeautifulSoup
import random
def search(arr, key):
    """
    Searches for the key in the array.
    Returns the lowest index where the key is found, or the length of the array if not found.
    """
    for i in range(len(arr)):
        if arr[i] == key:
            return i
    return len(arr)

# Test the search function
def test_search():
    test_array = ['a', 'b', 'c', 'd', 'e']
    assert search(test_array, 'c') == 2
    assert search(test_array, 'e') == 4
    assert search(test_array, 'z') == len(test_array) 
    print("All tests passed!")

test_search()

# Dataset generation
def get_text_from_gutenberg(url):
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')
    text = soup.get_text()
    text = ''.join(filter(str.isprintable, text)) 
    return text

def create_character_arrays(text, lengths, num_arrays_per_length):
    character_arrays = {}
    used_indices = set()  

    for length in lengths:
        arrays = []
        while len(arrays) < num_arrays_per_length:
            start_index = random.randint(0, len(text) - length - 1)
            if start_index not in used_indices:
                arrays.append(list(text[start_index:start_index + length]))
                for i in range(start_index, start_index + length):
                    used_indices.add(i)
        character_arrays[length] = arrays

    return character_arrays

gutenberg_url = 'https://www.gutenberg.org/cache/epub/40745/pg40745.txt' 
text = get_text_from_gutenberg(gutenberg_url)

lengths = [1000 * i for i in range(1, 11)]  
num_arrays_per_length = 50
character_arrays = create_character_arrays(text, lengths, num_arrays_per_length)

with open('character_arrays.pkl', 'wb') as f:
    pickle.dump(character_arrays, f)

with open('character_arrays.pkl', 'rb') as f:
    character_arrays = pickle.load(f)

characters = ['e', 'm', 'q', '%']

def measure_runtime(character_arrays, character):
    worst_case_runtimes = []
    best_case_runtimes = []
    average_case_runtimes = []

    for length, arrays in character_arrays.items():
        runtimes = []
        for array in arrays:
            runtime = search(array, character)
            runtimes.append(runtime)
        
        worst_case_runtimes.append(max(runtimes))
        best_case_runtimes.append(min(runtimes))
        average_case_runtimes.append(sum(runtimes) / len(runtimes))

    return worst_case_runtimes, best_case_runtimes, average_case_runtimes

def plot_runtimes(lengths, runtimes, title, ylabel):
    plt.figure()
    for character, runtime in runtimes.items():
        plt.plot(lengths, runtime, label=character)
    plt.xlabel('Array Length')
    plt.ylabel(ylabel)
    plt.title(title)
    plt.legend()
    plt.grid(True)
    plt.savefig(f"{title.replace(' ', '_').lower()}.png")

lengths = [1000 * i for i in range(1, 11)]
runtimes = {char: measure_runtime(character_arrays, char) for char in characters}

# worst-case runtimes
worst_case_runtimes = {char: runtimes[char][0] for char in characters}
plot_runtimes(lengths, worst_case_runtimes, 'Worst-Case Runtimes (Index)', 'Runtime (Index)')

# best-case runtimes
best_case_runtimes = {char: runtimes[char][1] for char in characters}
plot_runtimes(lengths, best_case_runtimes, 'Best-Case Runtimes (Index)', 'Runtime (Index)')

# average-case runtimes
average_case_runtimes = {char: runtimes[char][2] for char in characters}
plot_runtimes(lengths, average_case_runtimes, 'Average-Case Runtimes (Index)', 'Runtime (Index)')

print("Plots generated and saved as PNG files.")

def count_character_frequencies(character_arrays, characters):
    frequencies = {char: [] for char in characters}
    for length, arrays in character_arrays.items():
        for char in characters:
            total_count = sum(array.count(char) for array in arrays)
            frequencies[char].append(total_count)
    return frequencies

def plot_frequencies(lengths, frequencies, title):
    plt.figure()
    for character, counts in frequencies.items():
        plt.plot(lengths, counts, label=character)
    plt.xlabel('Array Length')
    plt.ylabel('Frequency')
    plt.title(title)
    plt.legend()
    plt.grid(True)
    plt.savefig(f"{title.replace(' ', '_').lower()}.png")

def print_frequencies(frequencies, lengths):
    print("Character Frequencies in Dataset:")
    for char, counts in frequencies.items():
        print(f"\nCharacter: '{char}'")
        for length, count in zip(lengths, counts):
            print(f"  Array Length {length}: {count} occurrences")

frequencies = count_character_frequencies(character_arrays, characters)
plot_frequencies(lengths, frequencies, 'Character Frequencies in Dataset')
print_frequencies(frequencies, lengths)

print("Frequency plots generated and saved as PNG files.")
