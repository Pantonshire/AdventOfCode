import Text.Read (readMaybe)

data Rule a = Range a Int Int | Positions a Int Int
            deriving (Show)

type Parser = String -> Maybe (Rule Char)

main :: IO ()
main = readFile "input" >>= \contents ->
       let ls = filter (not . null) (lines contents) in
       putStrLn ("Part 1: " ++ showSatisfiesCount Range ls)  >>
       putStrLn ("Part 2: " ++ showSatisfiesCount Positions ls)
     where showSatisfiesCount f ls' = show $ countMaybe id $ checkAll (parse f) ls'

checkAll :: Parser -> [String] -> Maybe [Bool]
checkAll f ls = foldMaybes (map (check f) ls)

check :: Parser -> String -> Maybe Bool
check f cs = f r' >>= \r ->
             return (satisfies r p)
           where (r',p') = splitOn (== ':') cs
                 p = dropWhile (== ' ') p'

satisfies :: Eq a => Rule a -> [a] -> Bool
satisfies (Range x min max) ys = min <= n && n <= max
                               where zs = filter (== x) ys
                                     n = length zs
satisfies (Positions x i j) ys = xor (occursAtPos ys x (i-1)) (occursAtPos ys x (j-1))

parse :: (Char -> Int -> Int -> Rule Char) -> Parser
parse g cs = readMaybe f >>= \no1 ->
             readMaybe s >>= \no2 ->
             singleton ms >>= \c ->
             return (g c no1 no2)
           where (f,r) = splitOn (== '-') cs
                 (s,ms) = splitOn (== ' ') r

singleton :: [a] -> Maybe a
singleton (x:[]) = Just x
singleton _ = Nothing

splitOn :: (a -> Bool) -> [a] -> ([a],[a])
splitOn _ [] = ([],[])
splitOn p (x:xs) | p x = ([],xs)
                 | otherwise = let (ys,zs) = splitOn p xs in
                               (x:ys,zs)

foldMaybes :: [Maybe a] -> Maybe [a]
foldMaybes [] = return []
foldMaybes (Nothing:_) = Nothing
foldMaybes ((Just x):mxs) = foldMaybes mxs >>= \xs ->
                            return (x:xs)

countMaybe :: (a -> Bool) -> Maybe [a] -> Maybe Int
countMaybe p mxs = mxs >>= \xs -> return (length (filter p xs))

xor :: Bool -> Bool -> Bool
xor = (/=)

occursAtPos :: Eq a => [a] -> a -> Int -> Bool
occursAtPos [] _ _ = False
occursAtPos (x:_) y 0 = x == y
occursAtPos (_:xs) y n = occursAtPos xs y (n-1)
