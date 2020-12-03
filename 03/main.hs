data Tile = Empty | Tree
          deriving (Show, Eq)
type Map = [[Tile]]
type Slope = (Int,Int)

main :: IO ()
main = readFile "input" >>= \contents ->
       let ls = filter (not . null) (lines contents) in
       let mrs = (readMap ls >>= \m -> return $ map ((flip toboggan) m) slopes) in
       maybeIO putStrLns (mrs >>= \rs -> return $ map (\(s,r) -> show s ++ ": " ++ show r) (zip slopes rs)) >>
       maybeIO putStrLn (mrs >>= \rs -> return (show (product rs)))
     where slopes = [(1,1),(3,1),(5,1),(7,1),(1,2)]

maybeIO :: Show a => (a -> IO ()) -> Maybe a -> IO ()
maybeIO _ Nothing = pure ()
maybeIO f (Just x) = f x

putStrLns :: [String] -> IO ()
putStrLns [] = pure ()
putStrLns (l:ls) = putStrLn l >> putStrLns ls

toboggan :: Slope -> Map -> Int
toboggan s = trees . (path s)

trees :: [Tile] -> Int
trees ts = length (filter (== Tree) ts)

path :: Slope -> Map -> [Tile]
path _ [] = []
path s@(i,j) rs = head (head rs) : path s (drop j (map (drop i) rs))

readMap :: [String] -> Maybe Map
readMap ls = foldMaybes (map (foldMaybes . (map readTile)) ls) >>= \rs ->
             return (map repeatList rs)

readTile :: Char -> Maybe Tile
readTile '.' = Just Empty
readTile '#' = Just Tree
readTile _ = Nothing

repeatList :: [a] -> [a]
repeatList [] = []
repeatList xs = repeatList' xs xs
              where repeatList' ys [] = repeatList' ys ys
                    repeatList' ys (z:zs) = z : repeatList' ys zs

foldMaybes :: [Maybe a] -> Maybe [a]
foldMaybes [] = return []
foldMaybes (Nothing:_) = Nothing
foldMaybes ((Just x):mxs) = foldMaybes mxs >>= \xs ->
                            return (x:xs)
