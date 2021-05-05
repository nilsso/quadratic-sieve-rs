from math import gcd, floor, log
from itertools import count

def is_prime(x):
    for d in range(2, x):
        if x % d == 0:
            return False
    return True

# Pollard rho factorization
def pollard_rho(n, x=2):
    y = 2
    d = 1
    g = lambda x: (x * x + 1) % n
    while d == 1:
        x = g(x)
        y = g(g(y))
        d = gcd(abs(x - y), n)
    return d if d != n else None

def factor(n):
    if is_prime(n):
        return { n: 1 }
    if n % 2 == 0:
        e = 0
        while n % 2 == 0:
            n //= 2
            e += 1
        f = { 2: e }
        for p, e in factor(n // 2**e).items():
            if not p in f:
                f[p] = 0
            f[p] += e
        return f

    p = None
    x = 2
    while not p:
        p = pollard_rho(n, x)
        x += 1
    f = factor(p)
    for p, e in factor(n // p).items():
        f[p] += e
    return f

if __name__ == '__main__':
    # from sys import argv
    # n = int(argv[1])
    # n = 2 * 3 * 5 * 7 * 11
    # n = 2 * 2 * 2 * 5 * 5 * 5
    n = 2**2 * 3**3
    f = factor(n)
    print(n, f)
