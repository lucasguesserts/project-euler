import Problem_3

main :: IO ()
main = (print . last . primeFactors) 600851475143
