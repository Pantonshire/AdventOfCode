
data Tile = Empty | Tree
          deriving (Show, Eq)
type Map = [[Tile]]
type Slope = (Int,Int)

main :: IO ()
main = readFile "input" >>= \contents ->
       let ls = filter (not . null) (lines contents) in
       let mm = readMap ls in
       let mp = (mm >>= \m -> return $ path (3,1) m) in
       let mn = (mp >>= \p -> return $ trees p) in
       putStrLn (show mp) >>
       putStrLn (show mn)

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
