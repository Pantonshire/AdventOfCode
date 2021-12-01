-- AOC 2020 day 1

target :: Num a => a
target = 2020

main :: IO ()
main = readFile "input" >>= \contents ->
       let ls = filter (not . null) (lines contents) in
       let ns = map read ls :: [Int] in
       putStrLn ("2: " ++ show (answer 2 ns target)) >>
       putStrLn ("3: " ++ show (answer 3 ns target))

answer :: (Integral a, Eq b, Num b) => a -> [b] -> b -> Maybe b
answer n xs x = case combinationsSumTo n xs x of (ys:_) -> Just (product ys)
                                                 []     -> Nothing

combinationsSumTo :: (Integral a, Eq b, Num b) => a -> [b] -> b -> [[b]]
combinationsSumTo n xs x = filter ((== x) . sum) (combinations n xs)

combinations :: Integral a => a -> [b] -> [[b]]
combinations 0 _ = []
combinations 1 xs = map (:[]) xs
combinations n xs = map (uncurry (:)) (pairs xs (combinations (pred n) xs))

pairs :: [a] -> [b] -> [(a,b)]
pairs xs ys = [(x,y) | x <- xs, y <- ys]
