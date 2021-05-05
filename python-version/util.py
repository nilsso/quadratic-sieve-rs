from itertools import count
from functools import reduce
from operator import mul

def prod(x):
    return reduce(mul, x)

def make_prod_mod(n):
    return lambda x: reduce(lambda a, b: (a * b) % n)

# Iterator adaptor to take a specified number of item.
# @param n Number of iterates to take
# @param it Iterator to adapt
# @return Adapted iterator
def take(n, it):
    for (i, x) in zip(range(n), it):
        yield x

# Iterator adaptor that yields while a predicate succeeds.
# @param pred Predicate `(it::item)->bool`
# @param it Iterator to adapt
# @return Adapted iterator
def take_while(pred, it):
    for x in it:
        if not pred(x):
            break
        yield x

# Skip n elements of iterator (advance by n items).
# @param n Number of elements to skip
# @param it Iterator
# @return The iterator having taken (or having attempted to take) n items
def skip(n, it):
    for _ in range(n):
        next(it)
    return it

# Counting iterator which yields elements expanding around a starting value.
# @param x (default=0) Value at which to start
# @return Iterator which yields values expanding from x
#
# Examples:
# ```python
# print(list(take(10, count_around())))
# # [0, 1, -1, 2, -2, 3, -3, 4, -4, 5]
# ```
#
# ```python
# print(list(take(10, count_around(100))))
# # [100, 101, 99, 102, 98, 103, 97, 104, 96, 105]
# ```
def count_around(x=0):
    yield x
    for i in count(1):
        yield x + i
        yield x - i

# Sum the values of two dictionaries.
# @param a First dictionary
# @param b Second dictionary
# @return Dictionary with values from a and b added (keys may not be lexicographically ordered)
#
# Examples:
# ```python
# a = { 2: 3, 5: 1 }
# b = { 3: 1, 7: 2 }
# print(dict_sum(a, b))
# # {2: 3, 5: 1, 3: 1, 7: 2}
# ```
def dict_sum(a, b):
    for k, v in b.items():
        a[k] = a.get(k, 0) + v
    return a

