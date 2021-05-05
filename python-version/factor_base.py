from primes import primes
from legendre import legendre, legendre_primes
from util import skip

# Generate factor base
#
# Generates a factor base from -1 and primes with Legendre symbol (n|p)=1 that is integer b smooth.
#
# @param n Legendre symbol operand
# @param b Smoothness bound
# @return Factor base
def factor_base(n, t):
    # res = [-1]
    # if n % 2 == 0:
        # res += [2]
    # pi = skip(1, primes()) # skip 2
    res = [-1]
    pi = primes()
    p = next(pi)
    while p <= t:
        # if legendre(n, p) == 1:
        if legendre(n, p) != -1:
            res += [p]
        p = next(pi)
    return res
    # return [-1] + [p for p
            # in take_while(lambda p: p <= t, primes())
            # if legendre(n, p) == 1]

def factor_base_of_l_primes(n, l):
    return [-1] + list(take(l, legendre_primes(n)))

