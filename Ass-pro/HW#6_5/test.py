def findMin(vec):
    n = len(vec)
    
    # Calculate sum of all elements
    total_sum = sum(vec)

    # Create a 2D array to store results of subproblems
    dp = [[False] * (total_sum + 1) for _ in range(n + 1)]

    # Initialize first column as true. 0 sum is possible
    # with all elements.
    for i in range(n + 1):
        dp[i][0] = True

    # Initialize top row, except dp[0][0], as false. With
    # 0 elements, no other sum except 0 is possible
    for j in range(1, total_sum + 1):
        dp[0][j] = False

    # Fill the partition table in bottom up manner
    for i in range(1, n + 1):
        for j in range(1, total_sum + 1):
          
            # If i'th element is excluded
            dp[i][j] = dp[i - 1][j]

            # If i'th element is included
            if vec[i - 1] <= j:
                dp[i][j] = dp[i - 1][j - vec[i - 1]]

    # Initialize difference of two sums.
    diff = float('inf')

    # Find the largest j such that dp[n][j]
    # is true where j loops from total_sum/2 to 0
    for j in range(total_sum // 2, -1, -1):
        if dp[n][j]:
            diff = total_sum - 2 * j
            break
    for row in dp:
        for element in row:
            print(element, end=" ")  # Print each element in the row, separated by spaces
    print()  # Print a newline after each row
    return diff
# Driver program to test above function
vec = [3,9,7,3]
print("The minimum difference of 2 sets is", findMin(vec))
