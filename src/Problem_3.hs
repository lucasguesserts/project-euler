module Problem_3
(
    primeFactors
) where

isPrime :: Integer -> Bool
isPrime n | n < 2 = False
isPrime n = all (\p -> n `mod` p /= 0) . takeWhile ((<= n) . (^ 2)) $ primes

primes :: [Integer]
primes = 2 : filter isPrime [3..]

primeFactors :: Integer -> [Integer]
primeFactors n = iter n primes where
    iter n (p:_) | n < p^2 = [n | n > 1]
    iter n ps@(p:ps') =
        let (d, r) = n `divMod` p
        in if r == 0 then p : iter d ps else iter n ps'
