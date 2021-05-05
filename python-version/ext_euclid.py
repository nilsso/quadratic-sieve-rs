def ext_euclid(a, b):
    prev = [a, 1, 0]
    curr = [b, 0, 1]
    while curr[0] != 0:
        q = prev[0] // curr[0]
        for i in range(3):
            temp = curr[i]
            curr[i] = prev[i] - q * temp;
            prev[i] = temp
    return prev

def gcd(a, b):
    d, _, _ = ext_euclid(a, b)
    return d

def inv(a, b):
    d, x, _ = ext_euclid(a, b)
    return x % b if d == 1 else None

if __name__ == '__main__':
    import sys
    a = int(sys.argv[1])
    b = int(sys.argv[2])
    print(ext_euclid(a, b))
    print(gcd(a, b))
    print(inv(a, b))
