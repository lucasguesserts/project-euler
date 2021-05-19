# Problem 4

## Problem description

A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.

**Answer:** 906609

## Formalization

$$
m: \mathbb{N}^* \rightarrow \mathbb{N}\\
m(n) =
max
\left\{
    i \in \mathbb{N}
    \ |
    \ 1 \leq \dfrac{n}{10^i}
\right\}
=
 \lfloor \log_{10}(n) \rfloor
$$

$m(n)$ is the magnitude of $n$.

$$
\mathcal{P} =
\left\{
n \in \mathbb{N}
\ |
\ n \text{ is a palindromic number}
\right\}
$$

$\mathcal{P}$ is the set of all palindromic numbers.
Examples of palindromic numbers: 1001, 2332, 47374.

Let $n \in \mathbb{N}^*$. Then:

$$
n =
\displaystyle\sum\limits_{i=0}^{m(n)}
c_i \cdot 10^{i}
$$

One can compute $c_i$ using the following function:

$$
\mathcal{C} :
\mathbb{N} \times \mathbb{N}
\rightarrow
\mathbb{N}
\\
\mathcal{C}(i, n)
=
c_i
=
\left\lfloor
    \dfrac{n}{10^i}
\right\rfloor
-
10 \cdot
\left\lfloor
    \dfrac{n}{10^{i+1}}
\right\rfloor
$$

And then it is possible do define whether
$n \in \mathbb{N}$
is or not a palindromic number:

$$
n \in \mathcal{P}
\leftrightarrow
c_i = c_{m(n)-i}
,\quad
\forall i \in
\left\{
    0,
    \cdots,
    \left\lfloor \dfrac{m(n)}{2} \right\rfloor
\right\}
$$

## Strategy - all

- Take a list of all numbers
  which are the result of a product
  between two 3-digit numbers.
- Take only the ones which are palindromic.
- Take the maximum of them.
