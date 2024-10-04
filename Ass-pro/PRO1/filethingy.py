import pickle

with open('character_arrays.pkl', 'rb') as f:
    character_arrays = pickle.load(f)

def print_character_arrays_summary(character_arrays, max_items=50, max_length=50):
    print("Character Arrays Summary:")
    for length, arrays in character_arrays.items():
        print(f"\nArray Length: {length}")
        print(f"Number of Arrays: {len(arrays)}")
        for i, array in enumerate(arrays[:max_items]):
            #Uncomment the below line to print the length of each array to be sure. 
            # print(len(array))
            array_str = ''.join(array[:max_length])
            if len(array) > max_length:
                array_str += "..."
            print(f"  Array {i+1}: {array_str}")

print_character_arrays_summary(character_arrays)
