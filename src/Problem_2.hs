module Problem_2
(
    recursionSolution,
    closedFormSolution,
) where

-- recursion

recursionSolution :: Integer
recursionSolution = sum (takeWhile (<4000000) (filter even fibonacciSequence)) where
    fibonacci :: Integer -> Integer
    fibonacci n
        | n==0 = 1
        | n==1 = 2
        | otherwise = fibonacci (n-1) + fibonacci (n-2)

    fibonacciSequence :: [Integer]
    fibonacciSequence = [fibonacci x | x <- [0..]]

-- closed form

closedFormSolution :: Integer
closedFormSolution = sum (takeWhile (<4000000) (filter even fibonacciSequence)) where
    phi :: Double
    phi = (1 + sqrt 5) / 2

    fibonacci :: Integer -> Integer
    fibonacci n = round (phi^n / sqrt 5)

    fibonacciSequence :: [Integer]
    fibonacciSequence = [fibonacci x | x <- [0..]]

