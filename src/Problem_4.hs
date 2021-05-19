module Problem_4
(
    maxPalindromic
) where

magnitude :: Double -> Int
magnitude n = floor (logBase 10 n)

coefficient :: Int -> Double -> Int
coefficient i n = floor (n/10^i) - (10 * floor (n/10^(i+1)))

palindromic :: Double -> Bool
palindromic n = all
    (==True)
    [
        coefficient i n == coefficient (magnitude n - i) n
        | i <- [0..(magnitude $ n / 2)]
    ]

allThreeDigits :: [Double]
allThreeDigits = [999, 998..100]

allProducts :: [Double]
allProducts = [ m*n | m <- allThreeDigits, n <- allThreeDigits, n <= m ]

palindromicProducts :: [Double]
palindromicProducts = filter palindromic allProducts

maxPalindromic :: Double
maxPalindromic = maximum palindromicProducts
