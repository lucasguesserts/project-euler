# Problem 1

## Problem description

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.

**Answer:** 233168

## Strategy - check multiplicity

for each i in 0 to 1000, if i is multiple of 3 or 5, add to sum.

## Strategy - Sum multiples

Sum the multiples of 3
Sum the multiples of 5
Subtract the multiples of 15
