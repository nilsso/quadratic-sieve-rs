from copy import deepcopy
from itertools import chain, combinations 

# Is a row subset of a "matrix" is linearly dependent
# @param mat An n by m array
# @param indices Indices of matrix mat row subset
# @param modulus Addition operation modulus
# @return If the subset is linearly dependent
def is_ld(mat, indices, modulus=None):
    n = len(mat[0])
    rows_sum = [sum(mat[i][j] for i in indices) for j in range(n)]
    if not modulus:
        return all(a == 0 for a in rows_sum)
    return all(a % modulus == 0 for a in rows_sum)

# Iterator over indices of linearly dependent row subsets of a "matrix"
#
# NOTE: Not the fastest; instead of using linear algebra,
# we just iterate over all combinations of the row indices.
#
# @param mat An n by m array
# @param modulus Addition operation modulus
# @yields Linearly dependent submatrix row indices
def ld_subsets(mat, modulus=None):
    indices = range(len(mat))
    for m in range(2, len(mat)):
        for subset_indices in combinations(indices, m):
            if is_ld(mat, subset_indices, modulus):
                yield subset_indices

# Transpose an m by n "matrix"
def transpose(mat):
    n = len(mat)
    m = len(mat[0])
    return [[mat[i][j] for i in range(n)] for j in range(m)]

def to_echelon(mat):
    mat = deepcopy(mat)
    m = len(mat)
    n = len(mat[0])

    def first_nonzero(j):
        for i in range(m):
            if mat[i][j] != 0:
                return i
        return None

    for j in range(n):
        k = first_nonzero(j)
        if k == None:
            continue
        for i in range(k + 1, m):
            if mat[i][j] != 0:
                for l in range(n):
                    mat[i][l] = (mat[i][l] + mat[k][l]) % 2
    return mat

def trim(mat):
    return [r for r in mat if any(a for a in r)]

if __name__ == '__main__':
    A = [
            [1, 0, 1],
            [0, 1, 0],
            [1, 1, 1],
            [1, 0, 1] ]
    # A = [
            # [1, 1, 0, 0, 0, 1],
            # [1, 0, 0, 0, 1, 0],
            # [0, 1, 0, 1, 0, 0],
            # [1, 1, 0, 0, 0, 1],
            # [1, 0, 0, 1, 0, 1],
            # [0, 1, 0, 0, 1, 1],
            # [1, 1, 0, 1, 1, 0] ]

    import sympy
    from itertools import product
    from util import skip

    def ld_subsets(mat):
        A = sympy.Matrix(mat).transpose().echelon_form() % 2
        n = A.shape[1]
        b = sympy.zeros(n, 1)
        solutions = sympy.linsolve((A, b), sympy.symbols(f't:{n}'))
        parameters = solutions.free_symbols
        for solution in solutions:
            for values in skip(1, product(*([range(2)]*len(parameters)))):
                subs = {t: v for (t, v) in zip(parameters, values)}
                subbed = [a % 2 for a in solution.subs(subs)]
                yield list(i for (i, bit) in enumerate(subbed) if bit)

    for indices in ld_subsets(A):
        print(indices)

