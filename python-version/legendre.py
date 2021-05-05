from primes import primes

# Is an integer a quadratic residue.
# @param a An integer
# @param m Modulus
# @return If integer a is a quadratic residue modulo m
# def is_quadratic_residue(a, m):
    # for x in range(1, m + 1):
        # if (x * x) % m == a % m:
            # return True
    # return False

# Legendre symbol
# @param a An integer
# @param p An odd prime
# @return The Legendre symbol of integer a with respsect to odd prime p
# def legendre(a, p):
    # if a % p == 0:
        # return 0
    # elif is_quadratic_residue(a, p):
        # return 1
    # else:
        # return -1

# Legendre symbol of an integer modulo an odd prime
#
# @param n An integer
# @param p An odd prime
# @return The Legendre symbol of integer n modulo odd prime p
def legendre(n, p):
    return pow(n, (p - 1) // 2, p)

# Is an integer a quadratic residue modulo a prime.
#
# Uses the Legendre symbol (i.e. Euler's criterion).
# Note: every integer is a quadratic residue modulo 2.
#
# @param n An integer
# @param p A prime (not necessarily odd, i.e. can be 2)
# @return Whether integer n is a quadratic residue modulo odd prime p
def is_quadratic_residue(n, p):
    return legendre(n, p) == 1 if p > 2 else True

# Iterator over primes for which an integer is a quadratic residue.
#
# @param n An integer
# @param start (default=2) Starting integer (nearest prime greater-than or equal to this)
# @yields Primes $p_j,p_{j+1},\ldots$ for which integer n is a quadratic residue modulo $p_i$
#   for $i>=j$
def legendre_primes(n, start=2):
    for p in primes(start):
        if is_quadratic_residue(n, p):
            yield p

if __name__ == '__main__':
    import sys
    a = int(sys.argv[1])
    p = int(sys.argv[2])
    print(legendre(a, p))
