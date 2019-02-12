import Data.Bits
import Numeric

groupSize n l = if length l < n then [] else [(take n l)] ++ groupSize n (drop n l)

toBin :: Int -> Int -> [Int] -> [Int]
toBin h g t = map (\s -> if s == (t2!!0) then 0 else 1) t2 where t2 = groupSize g (drop h t)

artoi = foldl (\n i -> (shift n 1) .|. i) 0

toHex :: Int -> Int -> [Int] -> [String]
toHex h g l = map (\i -> "0x" ++ showHex i "") $ map artoi $ groupSize 8 (toBin h g l)

checkSize h g l = shortList == (concatMap id $ groupSize g shortList)
    where shortList = drop h l
