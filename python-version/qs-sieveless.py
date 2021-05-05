from math import ceil, gcd, log, sqrt
from functools import reduce
from itertools import count

from util import count_around, skip, take, take_while
from legendre import legendre_primes
from naive_factor import naive_factor
from naive_linalg import ld_subsets

prod_mod = lambda xs, m: reduce(lambda a, b: (a * b) % m, xs)
sum_mod  = lambda xs, m: reduce(lambda a, b: (a + b) % m, xs)

# Iterator over potential factor bases for n
# @param n Integer to factor (and the Legendre modulo)
# @yields Potential factor bases (increasing in size by one prime at a time)
def factor_base_iter(n, b=None):
    # lower_bound = b if b else ceil(sqrt(n))
    lower_bound = b if b else ceil(log(n, 10))
    fb = [-1] + list(take_while(lambda p: p <= lower_bound, legendre_primes(n)))
    ps = legendre_primes(n, fb[-1] + 1)
    yield fb
    for p in ps:
        fb += [p]
        yield fb

# @param f A factoraztion dictionary
# @param fb Factor base
# @return If factorization f is smooth respect factor base fb
def is_smooth(f, fb):
    return max(f.keys()) <= max(fb)

# @param n Integer to factor
# @param fb Factor base
# @yields Pairs (xi, yi) for which factorization of yi is fb smooth
def smooth_iter(n, fb, lim=20):
    m = int(sqrt(n))
    for i, xi in enumerate(count_around(m)):
        yi = xi**2 - n
        # yi = (xi + m)**2 - n
        f = naive_factor(yi)
        if is_smooth(f, fb):
            yield (xi, yi, f)
            # yield (xi + m, yi, f)
        if i == lim:
            break
    yield None

def find_smooth(n, fb, lim=20):
    try:
        xis, yis, evecs = zip(*take(len(fb)+1, smooth_iter(n, fb, lim)))
        emat = [[evec.get(p, 0) for p in fb] for evec in evecs]
        bmat = [[a % 2 for a in row] for row in emat]
        return (xis, yis, emat, bmat)
    except:
        return None

def instructive_qs(n, b=None, lim=100):
    from colorama import Fore, Back, Style

    m = int(sqrt(n))
    print(f'Beginning attempt to factor {n} via quadratic sieve')

    # Just some printing constants
    t1 = "(product of xi's modulo n)"
    t2 = "(sum of the exponent vectors divided by 2)"
    t3 = "(product of {fb_j}^{l_j})"

    # TODO:
    # 1. Detect prime
    # 2. Detect perfect power

    fbi = factor_base_iter(n, b)
    # fb = next(fbi)
    # print(f'Selected initial factor base {fb}')

    for fb in fbi:
        t = len(fb)
        print(Fore.MAGENTA + f'Selected factor base fb = {fb}')
        print(Fore.RED + f'    Looking for {len(fb)+1} (xi,yi) pairs with yi {max(fb)}-smooth.')
        smooths = find_smooth(n, fb, lim)
        if not smooths:
            print(Fore.RED + f"    Gave up looking after {lim} xi's!" + Style.RESET_ALL)
            print('    Expanding the factor base and re-running...')
            continue
        xis, yis, emat, bmat = smooths
        print(Fore.BLUE + '    Found...' + Style.RESET_ALL)
        f_strings = [
                '·'.join(f'({p}^{e})' if e > 1 else f'({p})'
                    for (p, e) in zip(fb, evec) if e > 0)
                    for evec in emat ]
        xi_w = max(len(str(xi)) for xi in xis)
        yi_w = max(len(str(yi)) for yi in yis)
        f_string_w = max(len(f_string) for f_string in f_strings)
        fmt = f'    ({{}}) xi = {{:>{xi_w}}} yi = {{:>{yi_w}}} = {{:{f_string_w}}} (with binary exponent vector {{}})'
        for i in range(len(xis)):
            xi, yi, evec, bvec = [m[i] for m in [xis, yis, emat, bmat]]
            print(fmt.format(i, xi, yi, f_strings[i], bvec))
        print(Fore.GREEN + '    Searching for linearly dependent subsets of the binary vectors.' + Style.RESET_ALL)
        for indices in ld_subsets(bmat, 2):
            print(Fore.BLUE + f'    Found LD subset...' + Style.RESET_ALL)
            for i in indices:
                print(f'    ({i}) {bmat[i]}')
            x = prod_mod([xis[i] for i in indices], n)
            l = [sum(emat[i][j] for i in indices) // 2 for j in range(t)]
            y = prod_mod([p**e for (p, e) in zip(fb, l)], n)
            print('        Calculated...')
            s1 = f'        x = {x} = {"·".join(f"({xis[i]})" for i in indices)}'
            s2 = f'        l = {l}'
            s3 = f'        y = {y} = {"·".join(f"({p}^{e})" for p, e in zip(fb, l) if e > 0)}'
            max_w = max(len(s1), max(len(s2), len(s3)))
            fmt = f'{{:{max_w}}} {{}}'
            print(fmt.format(s1, t1))
            print(fmt.format(s2, t2))
            print(fmt.format(s3, t3))
            if x % n != y % n and x % n != (-y) % n:
                print('        ' + Fore.BLUE + 'Success!' + Style.RESET_ALL)
                print(f' (x % n == {x%n} != {y%n} == y % n) and (x % n == {x%n} != {(-y)%n} == y % n)')
                f1 = gcd(x + y, n)
                f2 = gcd(x - y, n)
                print('        We\'ve found non-trivial factors:')
                print(f'          gcd(x + y, n) = {f1}, and')
                print(f'          gcd(x - y, n) = {f2}')
                return (f1, f2)
            else:
                print('        ' + Fore.RED + 'Failed' + Style.RESET_ALL)
                print(f' (x % n == {x%n} == {y%n} == y % n) and (x % n == {x%n} == {(-y)%n} == y % n)')
        print(Fore.RED + 'Exhausted all LD subsets for this set of xi\'s' + Style.RESET_ALL)
        print('Expanding the factor base and re-running...')

# Entry point
if __name__ == '__main__':
    import sys
    n = int(sys.argv[1])
    lim = int(sys.argv[2])
    b = int(sys.argv[3]) if len(sys.argv) > 3 else None
    instructive_qs(n, b, lim)
