module Problem_1
(
    checkMultiplicity,
    sumMultiples,
) where

limit :: Integer
limit = 1000 - 1

-- check multiplicity strategy

checkMultiplicity :: Integer
checkMultiplicity = sum [x | x <- [1..limit], (x `mod` 3 == 0) || (x `mod` 5 == 0)]

-- sum multiples strategy

sum_3 :: Integer
sum_3 = sum [3, 6..limit]

sum_5 :: Integer
sum_5 = sum [5, 10..limit]

sum_15 :: Integer
sum_15 = sum [15, 30..limit]

sumMultiples :: Integer
sumMultiples = sum_3 + sum_5 - sum_15
