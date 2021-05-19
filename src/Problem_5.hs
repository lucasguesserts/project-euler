module Problem_5
(
    satisfyCondition,
    solution,
) where

satisfyCondition x = and [x `mod` n == 0 | n <- [1..20]]

solution = head $ filter satisfyCondition [2..]
