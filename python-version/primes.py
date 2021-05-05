from math import sqrt

# Is some integer prime (via naive trial division).
# @param x Integer to check
def is_prime(x):
    for d in range(2, int(sqrt(x)) + 1):
        if x % d == 0:
            return False
    return True

def next_prime(x):
    while not is_prime(x):
        x += 1
    return x

# Prime integer iterator
def primes(start=None):
    x = start if start else 2
    while True:
        x = next_prime(x)
        yield x
        x += 1

# Primes up to some integer n (via sieve of Eratosthenes)
def primes_up_to(n):
    flags = [False, False] + [True] * (n - 2) # [0: false, 1: false, 2:true, ...]
    for i in range(n):
        if flags[i]:
            for j in range(2*i, n, i):
                flags[j] = False
    return [i for (i, f) in enumerate(flags) if f]

