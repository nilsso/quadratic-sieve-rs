from primes import primes
from math import ceil, gcd, sqrt
from copy import deepcopy
from util import make_prod_mod, prod, take
from itertools import combinations
from sqrt_mod import sqrt_mod

# Iterate primes p and quadratic roots of n modulo p for whom roots exist
#
# @param n Integer to find quadratic solutions to
# @yields A tuple (p, (r,)) or (p, (r1,r2)) of prime p and quadratic root(s) of
#   n modulo p, or None if no roots exist
def iter_roots(n):
    for p in primes():
        r1, r2 = sqrt_mod(n, p)
        if r1 is not None:
            yield (p, (r1, r2)) if r1 != r2 else (p, (r1,))

# Factor over a factor base
#
# @param n Integer to find two non-trivial factors of
# @param fb Factor base
# @return Tuple of whatever is left after factoring over the factor base,
#   and a vector of exponents (corresponding to primes in the factor base)
def factor(n, fb):
    f = { p: 0 for p in fb }
    for p in fb:
        while n % p == 0:
            n //= p
            f[p] += 1
    return (n, [e for (p, e) in f.items()])

# Find p_b smooth square numbers within a sieving interval
#
# @param n Integer to find two non-trivial factors of
#   (but also the quadratic polynomial linear term)
# @param b Desired factor base length
# @param I Sieving interval
# @return List of sieved x's, of y's (corresponding in parallel with the x's
#   as smooth pairs), and the factor base
def find_smooth(n, b, I):
    m = ceil(sqrt(n))
    # Sieve from sieving interval
    V = [(x + m)**2 - n for x in range(I)]
    # In tandem, find b primes p for which n has roots modulo p
    p_root_pairs = list(take(b, iter_roots(n)))
    fb, roots = zip(*p_root_pairs)
    # Sieve!
    for (p, roots) in p_root_pairs:
        for r in roots:
            start = (r - m) % p
            for i in range(start, len(V), p):
                while V[i] % p == 0:
                    V[i] //= p
    # Construct, collect and return the x's and y's (which are smooth)
    xs = [i + m for (i, v) in enumerate(V) if abs(v) == 1]
    ys = [x**2 - n for x in xs]
    return (xs, ys, fb)


# Construct exponent matrices
#
# Returns a tuple of two matrices:
#   - first a matrix where the i'th row consists of the exponents of the
#     factor base primes for the factorization of the i'th smooth number,
#   - and second the first matrix reduced modulo 2.
#
# @param smooths Found smooth numbers
# @param fb Factor base (over which the smooths are smooth)
# @return Exponent matrix and exponent matrix modulo 2
def construct_exponent_mats(smooths, fb):
    m = len(smooths)
    n = len(fb)
    emat = [[0]*n for _ in range(m)]
    bmat = deepcopy(emat)
    # bmat_aug = [[0]*n_ for _ in range(m)]
    for (i, y) in enumerate(smooths):
        # bmat_aug[i][m+i-1] = 1
        rem, f = factor(y, fb)
        # check to make sure that factored entirely over the factor base
        assert(rem == 1) # if not, there's a problem!
        for (j, e) in enumerate(f):
            emat[i][j] = e
            bmat[i][j] = e % 2
    return (emat, bmat)

# Augment a matrix
#
# Consider this example...
# ┌   ┐             ┌       ┐
# │a b│ (augmented) │a b│1 0│
# │c d│     ==>     │c d│0 1│
# └   ┘             └       ┘
#
# @param mat Matrix to augment
# @return Matrix mat augmented
def augment(mat):
    m = len(mat)
    n = len(mat[0])
    n_ = m + n
    mat_aug = [[0]*n_ for _ in range(m)]
    for i in range(m):
        mat_aug[i][m+i-1] = 1
        for j in range(n):
            mat_aug[i][j] = mat[i][j]
    return mat_aug

# Find first row whose j'th element is non-zero
#
# @param mat Matrix to search
# @param j Column in which to search
# @param start (default=j) Row index from where begin searching
# @return Index i of first row where mat_(ij) is non-zero, or None if no
#   such row was found.
def find_nonzero_in_col(mat, j, start=None):
    for i in range(start if start else j, len(mat)):
        if mat[i][j] != 0:
            return i
    return None # I don't like implicit None returns!

# Find first column whose i'th element is non-zero
#
# @param mat Matrix to search
# @param i Row in which to search
# @param start (default=i) Column index from where begin searching
# @return Index j of first column where mat_(ij) is non-zero, or None if no
#   such row was found.
def find_nonzero_in_row(mat, i, start=None):
    for j in range(start if start else i, len(mat[0])):
        if mat[i][j] != 0:
            return j

# Bring binary matrix into echelon form.
#
# If the matrix to bring to echelon form is an augmented matrix, then in
# echelon form the sub-rows within the augmentation sub-matrix correspond to
# subsets of the row indices of the original matrix that are linearly dependent
# modulo 2 (i.e. in the left nullspace).
#
# @param mat Matrix to operate over
# @return Computed echelon form matrix
def to_echelon_form(mat):
    mat = deepcopy(mat)
    m = len(mat)
    n = len(mat[0])
    # Rearrange rows (move non-zero j'th columns up)
    for j in range(m):
        # if j'th row is zero in j'th col...
        if mat[j][j] == 0:
            # find i>j'th row that is non-zero in j'th col
            i = find_nonzero_in_col(mat, j)
            if i:
                # swap j'th row and i'th row
                for k in range(n):
                    temp = mat[i][k]
                    mat[i][k] = mat[j][k]
                    mat[j][k] = temp
    # Operate on rows to form echelon matrix
    for i in range(m):
        j = find_nonzero_in_row(mat, i)
        # if we've not reached all zero rows...
        if j:
            # we have a echelon for row
            for k in range(i + 1, m):
                # which we remove from (add modulo 2 to) all later rows
                # with non-zero j'th components
                if mat[k][j] != 0:
                    for l in range(n):
                        mat[k][l] = (mat[i][l] + mat[k][l]) % 2
    return mat

# Iterator over linearly dependent row index subsets of a binary matrix.
#
# (Refer to the notes in the to_echelon_form function about how to find
# linearly dependent row index subsets.)
#
# @param mat Matrix
# @yields Subset of binary matrix row indices in left-nullspace
def ld_index_iter(bmat):
    n = len(bmat[0])
    bmat_aug = augment(bmat)
    bmat_echelon = to_echelon_form(bmat_aug)
    for i in range(len(bmat_echelon)):
        if all(a == 0 for a in bmat_echelon[i][:n]):
            coords = bmat_echelon[i][n:]
            indices = [i for (i, b) in enumerate(coords) if b]
            if len(indices) > 0:
                yield indices

# Quadratic sieve
#
# My implementation of a single-polynomial quadratic sieve.
#
# @param n Integer to find two non-trivial factors of
# @param b Desired factor base length
# @param I Sieving interval
def qs(n, b, I):
    prod_mod = make_prod_mod(n)
    xs, ys, fb = find_smooth(n, b, I)
    if len(xs) <= len(fb):
        raise Exception(f'''\
With size {len(fb)} factor base found {len(xs)} (x,y) smooth pairs;
at least {len(fb)+1} are required to guarantee linear dependence.
Try increasing the factor base or the sieving interval.
''')
    for indices in combinations(range(len(xs)), len(fb)+1):
        xs_ = [xs[i] for i in indices]
        ys_ = [ys[i] for i in indices]
        emat, bmat = construct_exponent_mats(ys_, fb)
        # The augmented binary matrix is for solving for the left nullspace,
        # and thus all the subsets of linearly dependent exponent vector indices.
        for indices in ld_index_iter(bmat):
            x = prod(xs_[i] for i in indices)
            l = [sum(emat[i][j] for i in indices) // 2 for j in range(len(fb))]
            y = prod(p**e for (p, e) in zip(fb, l))
            if not x % n in [y % n, -y % n]:
                return (gcd(x + y, n), gcd(x - y, n))

if __name__ == '__main__':
    import sys
    import numpy as np

    n = int(sys.argv[1])
    b = int(sys.argv[2])
    I = int(sys.argv[3])

    print(qs(n, b, I))
