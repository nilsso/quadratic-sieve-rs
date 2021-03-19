Factorizing integers is an age-old problem stemming from
the fundamental theorem of arithmetic:
that every positive integer has a unique prime factorization.[^fta]
Numerical number theorists have for centuries endeavoured to construct faster factoring algorithms;
one such algorithm developed within the last several decades is the *quadratic sieve*.

> The quadratic sieve algorithm (QS) is an integer factorization algorithm and, in practice, the
> second fastest method known (after the general number field sieve). It is still the fastest for
> integers under 100 decimal digits or so, and is considerably simpler than the number field sieve.
> It is a general-purpose factorization algorithm, meaning that its running time depends solely on
> the size of the integer to be factored, and not on special structure or properties. It was
> invented by Carl Pomerance in 1981 as an improvement to Schroeppel's linear sieve.[^qs]

[^fta]: https://en.wikipedia.org/wiki/Fundamental_theorem_of_arithmetic
[^qs]: https://en.wikipedia.org/wiki/Quadratic_sieve

# Notes

### Quadratic residue

An integer $q$ is called a *quadratic residue* modulo $n$ if it is congruent to a perfect square modulo $n$;
i.e., if there exists an integer $x$ such that:
\[
    x^2 \equiv q\pmod n.
\]
Otherwise, $q$ is called a *quadratic nonresidue* modulo $n$.

