import requests
from bs4 import BeautifulSoup
import random

# Function to get text from Project Gutenberg
def get_text_from_gutenberg(url):
    response = requests.get(url)
    soup = BeautifulSoup(response.content, 'html.parser')
    text = soup.get_text()
    return text

# Function to create character arrays from text
def create_character_arrays(text, lengths, num_arrays_per_length):
    character_arrays = {}
    for length in lengths:
        arrays = []
        for _ in range(num_arrays_per_length):
            start_index = random.randint(0, len(text) - length - 1)
            arrays.append(list(text[start_index:start_index + length]))
        character_arrays[length] = arrays
    return character_arrays

# Example usage
gutenberg_url = 'https://www.gutenberg.org/files/84/84-0.txt'  # Frankenstein by Mary Shelley
text = get_text_from_gutenberg(gutenberg_url)

lengths = [1000 * i for i in range(1, 11)]  # Arrays of length 1000, 2000, ..., 10000
num_arrays_per_length = 50
character_arrays = create_character_arrays(text, lengths, num_arrays_per_length)

# Save the character arrays to disk
import pickle
with open('character_arrays.pkl', 'wb') as f:
    pickle.dump(character_arrays, f)
