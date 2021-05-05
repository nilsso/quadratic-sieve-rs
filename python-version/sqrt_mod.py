from naive_factor import div_while
from legendre import is_quadratic_residue

# Infinite iterator over repeated mod-squarings
# @param a Integer to square
# @param m Modulus
# @yields a^(2^i) (mod m) for i in [1,2,...]
def squarings(a, m):
    while True:
        a = (a * a) % m
        yield a

# Get two quadratic roots (or double root) of an integer modulo a prime, if they exists.
#
# Algorithm is the Tonelli-Shanks algorithm:
# https://en.wikipedia.org/wiki/Tonelli%E2%80%93Shanks_algorithm
# http://rosettacode.org/wiki/Tonelli-Shanks_algorithm
#
# @param n Integer whose roots to return
# @param p Prime modulus
# @return (r1,r2) Two quadratic root of n modulo p, or (None, None)
def sqrt_mod(n, p):
    if p == 2:
        # Any integer has a double root 0 or 1 modulo 2
        r = n % 2
        return (r, r)
    if not is_quadratic_residue(n, p):
        return (None, None)
    # express p - 1 as q*2^s (where q is odd)
    q, s = div_while(p - 1, 2)
    if s == 1:
        r = pow(n, (p+1)//4, p)
        return (r, p - r)
    # find a quadratic-non residue modulo p (in this case the first)
    z = next(filter(lambda z: not is_quadratic_residue(z, p), range(2, p)))
    m = s
    c = pow(z, q, p)
    t = pow(n, q, p)
    r = pow(n, (q+1)//2, p)
    while (t - 1) % p != 0:
        # find least 0 < i < m st. t^(2^i) ≡ 1 (mod p)
        i = next(i+1 for (i, t2) in enumerate(squarings(t, p)) if (t2 - 1) % p == 0)
        b = pow(c, 1 << (m - i - 1), p) # b = c^(2^(m-i-1))
        r = (r * b) % p
        c = (b * b) % p
        t = (t * c) % p
        m = i
    return (r, p - r)

if __name__ == '__main__':
    import sys
    n = int(sys.argv[1])
    p = int(sys.argv[2])
    print(sqrt_mod(n, p))
