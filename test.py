def floor_sqrt(n):
    if n < 0:
        raise ValueError("Input must be a non-negative integer.")
    
    low, high = 1, n
    
    while low <= high:
        print("low = ", low)
        print("high = ", high)
        mid = (low + high) // 2
        print("mid :",mid)
        mid_squared = mid * mid
        
        if mid_squared == n:
            return mid
        elif mid_squared < n:
            low = mid + 1
            print("low :",low)
        else:
            high = mid - 1
            print("high :",high)
        print("\n")
    return high
 #Works but need to automate the finding of initial values somehow. 
n = 100
print(floor_sqrt(n))  # Output: 3
