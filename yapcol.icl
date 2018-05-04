implementation module yapcol

import yapcol
import StdOverloaded
import StdFunc
import StdTuple
from Data.Func import $

unP :: (Parser t a) -> [t] -> (Either String a, [t])
unP (Parser p) = p
// ---------- Top Functions ----------

parse :: (Parser t a) [t] -> Either String a
parse p i = fst $ run p i

run :: (Parser t a) [t] -> (Either String a, [t])
run (Parser p) i = case p i of
	(Right x,s) = (Right x,s)
	(Left m,s) = (Left m,s)

Start = 1
