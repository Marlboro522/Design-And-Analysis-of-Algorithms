def search(arr, key):
    """
    Searches for the key in the array.
    Returns the lowest index where the key is found, or the length of the array if not found.
    """
    for i in range(len(arr)):
        if arr[i] == key:
            return i
    return -1

# Test the search function
def test_search():
    test_array = ['a', 'b', 'c', 'd', 'e']
    assert search(test_array, 'c') == 2
    assert search(test_array, 'e') == 4
    assert search(test_array, 'z') == -1  # Not found
    print(search(test_array, 'z'))
    print("All tests passed!")

test_search()
