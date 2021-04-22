from math import gcd, sqrt, floor, ceil, prod, log
from itertools import count, combinations
from functools import reduce
from random import random

# Take number of iterates from some iterator.
# @param it Iterator
# @param n Number of iterates to take
def take(it, n):
    for _ in range(n):
        yield next(i)

def take_while(it, pred):
    for x in it:
        if not pred(x):
            break
        yield x

def filter(it, pred):
    for x in it:
        if pred(x):
            yield x

# Is some integer prime (via naive trial division).
# @param x Integer to check
def is_prime(x):
    for d in range(2, floor(sqrt(x)) + 1):
        if x % d == 0:
            return False
    return True

# Prime integer iterator
def primes():
    x = 2
    while True:
        if is_prime(x):
            yield x
        x += 1

# Primes up to some integer n (via sieve of Eratosthenes)
def primes_up_to(n):
    flags = [False, False] + [True] * (n - 2)
    for i in range(n):
        if flags[i]:
            for j in range(2*i, n, i):
                flags[j] = False
    return [i for (i, f) in enumerate(flags) if f]

# Is an integer a quadratic residue.
# @param a Integer to determine
# @param m Modulo
def is_quadratic_residue(a, m):
    for x in range(1, m + 1):
        if (x * x) % m == a % m:
            return True
    return False

# Legendre symbol of $p$ over $a$.
# @param p
# @param a
def legendre(p, a):
    if a % p == 0:
        return 0
    elif is_quadratic_residue(p, a):
        return 1
    else:
        return -1

# Take first from iterator when predicate is true.
# @param it Iterator
# @param pred Predicate `(it::item)->bool`
def first(it, pred):
    for x in it:
        if pred(x):
            return x
    return None

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


# Factorbase up to t-th prime.
# @param t Index of last prime
def factor_base_t(t):
    B = [-1]
    j = 1
    i = primes()
    while j < t:
        B += [next(i)]
        j += 1
    return B

# Factorbase up to t-th prime less-than or equal to some upper limit.
# @param x Upper bound
def factor_base_x(x, n):
    return [-1] + [p for p
            in take_while(primes(), lambda p: p <= x)
            if legendre(n, p) == 1]

# Positive/negative ascending integer iterator.
def countpm():
    yield 0
    for x in count(1):
        yield x
        yield -x

# Integer prime-power factorization (via naive trial division).
# @param x Integer to factor
def factor_trial_div(x):
    f = []
    if x < 0:
        f += [(-1, 1)]
        x = -x
    p_i = primes()
    while x > 1:
        p = next(p_i)
        e = 0
        while x % p == 0 and x > 1:
            x //= p
            e += 1
        if e > 0:
            f += [(p, e)]
    return f

# @param x Integer to check smoothness of
# @param B Factor base
# @param f Optional pre-computed prime-power factorization
def smooth(x, B, f=None):
    pt = B[-1]
    if not f:
        f = factor_trial_div(x)
    return all(p <= pt for (p, _) in f)

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

def quadratic_sieve(n, B):
    f = is_perfect_power(n)
    if f:
        return f
    # print(n)
    # print(B)
    m = floor(sqrt(n))
    t = len(B)
    coords = { p: i for i, p in enumerate(B) }
    pairs = []
    removed = []
    vects = []
    x_iter = countpm()
    while True:
        while len(pairs) < t + 1:
            x = next(x_iter)
            b = (x + m)**2 - n
            f = factor_trial_div(b) # TODO
            is_smooth = smooth(b, B, f)
            if is_smooth:
                a = x + m
                E = [0] * t
                v = [0] * t
                for p, e in f:
                    i = coords[p]
                    E[i] = e
                    v[i] = e % 2
                pairs += [(a, b, E)]
                vects += [v]
        for i in range(len(pairs)):
            print(i, pairs[i], vects[i])
        mul_mod = lambda a, b: (a * b) % n
        for indices in ld_subsets(vects, 2):
            x = reduce(mul_mod, [pairs[i][0] for i in indices])
            l = [sum(pairs[i][2][j] for i in indices) // 2 for j in range(t)]
            py = reduce(mul_mod, [B[i]**l[i] for i in range(t)])
            ny = -py % n
            # print(indices, x, py)
            if x != py and x != ny:
                return gcd(x - py, n)
        # return None
        pairs = pairs[1:]
        vects = vects[1:]

def factor(n):
    if is_prime(n):
        return [n]
    B = factor_base_x(ceil(sqrt(n)), n)
    # B = factor_base_x(n, n)
    f = quadratic_sieve(n, B)
    print(f)
    n //= f
    fs = []
    fs += [n] if is_prime(n) else factor(n)
    fs += [f] if is_prime(f) else factor(f)
    return fs

def main():
    n = 2 * 3 * 7 # 42
    # n = 2 * 3 * 5 * 7 * 11
    f = []
    print(n)
    while n > 1 and not is_prime(n):
        p = factor(n)
        n //= p
        f += [p]
    print(f, n)
    if n > 1:
        f += [n]
    print(f)
    for p in f:
        print(p, is_prime(p), is_perfect_power(p))

if __name__ == '__main__':
    n = 42
    B = factor_base_x(ceil(sqrt(n)), n)
    print(n, B)
    print(quadratic_sieve(n, B))
    # print(floor(sqrt(n)))
    # print(factor_trial_div(-42))
    # main()
