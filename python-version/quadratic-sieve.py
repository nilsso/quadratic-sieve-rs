from math import gcd, sqrt, floor, ceil, prod, log
from itertools import count, combinations
from functools import reduce
from random import randrange

from legendre import legendre
from util import take, take_while, skip
from factor_base import factor_base, factor_base_of_l_primes
from primes import primes
from naive_factor import naive_factor

from operator import mul

mul_mod = lambda a, b: (a * b) % n
prod = lambda x: reduce(mul, x)
prod_mod = lambda x: reduce(mul_mod, x)

# Check if an integer is a $k^\mathrm{th}$ integer power.
# @param n Integer to check
# @param k Power to check
# @return Integer x such that $n == x^k$, or `None`
def is_kth_power(n, k):
    assert(n >= 2 and k >= 2)
    r = n**(1 / k)
    for x in reversed(range(floor(r) + 1)):
        if abs(r - x) <= 5 / 8:
            if x == 0 or abs(r - x) >= 1/4:
                return None
            if n == x**k:
                return x
    return None

# Check if an integer is a perfect integer power.
# @param n Integer to check
# @return Tupple $(x,k)$ where $e\ge 1$ and $n == x^k$
def is_perfect_power(n):
    primes = primes_up_to(int(log(2 * n, 2)))
    for p in primes:
        x = is_kth_power(n, p)
        if x and x > 0:
            return (x, p)
    return None

# Positive/negative ascending integer iterator.
def count_near(x, step=1):
    yield x
    for i in count(1):
        yield x + i * step
        yield x - i * step

# @param x Integer to check smoothness of
# @param B Factor base
# @param f Optional pre-computed prime-power factorization
def smooth(x, B, f=None):
    if not f:
        f = naive_factor(x)
    return max(f.values()) <= max(B)

def is_smooth(f, fb):
    return max(f.keys()) <= max(fb)

# @param mat Matrix
# @param i Matrix subset row indices
# @param modulus Addition operation modulus
def is_ld(mat, i, modulus=None):
    n = len(mat[0])
    z = [0] * n
    v = [sum(mat[r][c] for r in i) for c in range(n)]
    if modulus:
        v = [v[i] % modulus for i in range(len(v))]
    return z == v

# @param mat Matrix
# @param modulus Addition operation modulus
def ld_subsets(mat, modulus=None):
    row_indices = range(len(mat))
    for m in range(2, len(mat)):
        for i in combinations(row_indices, m):
            if is_ld(mat, i, modulus):
                yield i

# @param n Composite, non-perfect power integer
# @param B Factor base
def quadratic_sieve(n, B, I=1):
    m = floor(sqrt(n))
    t = len(B)
    coords = { p: i for i, p in enumerate(B) }
    pairs = []
    removed = []
    vects = []
    c = 0
    # TODO: Need helper for trying different subsets, instead of removing randomly and losing
    # the removed forever
    # x_iter = count_near(int(sqrt(n)), I)
    # x_iter = count(int(sqrt(n)))
    x_iter = count(1)
    while True:
        # if c > 5:
            # I += 1
            # x_iter = count_near(int(sqrt(n)), I)
            # c = 0
        while len(pairs) <= t:
            xi = next(x_iter)
            yi = (xi)**2 - n
            # yi = (xi + m)**2 - n
            f = naive_factor(yi) # TODO
            # print(xi, yi, f, is_smooth(f, B))
            if is_smooth(f, B):
                E = [0] * t
                # print(xi, yi, f)
                # print(f)
                for (p, e) in f.items():
                    j = coords[p]
                    E[j] = e
                v = [e % 2 for e in E]
                pairs += [(xi, yi, E)]
                # pairs += [(xi + m, yi, E)]
                vects += [v]
        print(c)
        for i in range(len(pairs)):
            print(pairs[i], vects[i])
        for indices in ld_subsets(vects, 2):
            # for i in indices:
                # print(pairs[i])
            x = prod_mod(pairs[i][0] for i in indices)
            l = [sum(pairs[i][2][j] for i in indices) // 2 for j in range(t)]
            y = prod_mod(B[j]**l[j] for j in range(t))
            if x % n != y % n and x % n != (-y) % n:
                # return gcd(x - y, n)
                # print((x + y) % n, gcd(x + y, n), (x - y) % n, gcd(x - y, n))
                return gcd(x + y, n), gcd(x - y, n)
            # print(indices)
        # return None
        i = randrange(0, len(pairs))
        print(f"removed {pairs[i]} {vects[i]}")
        pairs = pairs[:i] + pairs[i+1:]
        vects = vects[:i] + vects[i+1:]
        c += 1

def factor(n):
    if is_prime(n):
        return [n]
    B = factor_base(ceil(sqrt(n)), n)
    # B = factor_base(n, n)
    f = is_perfect_power(n)
    if f:
        return f # if n is a perfect power, return (p, e)
    f = quadratic_sieve(n, B)
    n //= f
    fs = []
    fs += [n] if is_prime(n) else factor(n)
    fs += [f] if is_prime(f) else factor(f)
    return fs

# NOTE
# Currently fails for:
# 94
# 92 (for b < 11)
# 90
# 12346
# (likely because of the vector removal process)
# (when n = two primes (e.g. 86 = 2 * 43))
if __name__ == '__main__':
    import sys
    n = int(sys.argv[1])
    b = int(sys.argv[2])
    # n = 16843009
    # print(legendre(22, 2))
    # n = 24961
    B = factor_base(n, b)
    # B = factor_base(n, n*4)
    # B = factor_base(n, int(sqrt(n)))
    # B = factor_base_of_l_primes(n, 5)
    print(B)
    print(quadratic_sieve(n, B))
    # print(quadratic_sieve(n, B))
    # print(floor(sqrt(n)))
    # print(factor_trial_div(-42))
    # main()
