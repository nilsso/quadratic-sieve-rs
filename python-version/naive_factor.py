from math import ceil, sqrt

from util import dict_sum, take_while
from primes import primes

# Divide an integer while it is divisible.
# @param x The dividend
# @param d The divisor
# @return Tupple of the remainder from dividing x by d^e, and e
def div_while(x, d):
    i = 0
    while x % d == 0:
        x //= d
        i += 1
    return x, i

# Integer prime-power factorization (via naive trial division).
# @param n Integer to factor
# @return Prime-power factorization of n
def naive_factor(n):
    f = dict()
    if n < 0:
        n = -n
        f[-1] = 1
    root_n = int(sqrt(n))
    for p in take_while(lambda p: n > 1 and p <= root_n, primes()):
        n, i = div_while(n, p)
        if i > 0:
            f = dict_sum(f, {p: i})
    return f if n == 1 else dict_sum(f, {n: 1})

if __name__ == '__main__':
    import sys
    n = int(sys.argv[1])
    print(naive_factor(n))
    # print(naive_factor(n))
