implementation module yapcol

import yapcol
import StdOverloaded
import StdFunc
import StdTuple
from Data.Func import $

unP :: (Parser t a) -> [t] -> (Either String a, [t])
unP (Parser p) = p

// ---------- Instances ----------

instance Functor (Parser t) where
	fmap f (Parser p) = Parser \s -> case p s of
		(Right a, s) = (Right (f a), s)
		(Left m,s) = (Left m,s)

instance Applicative (Parser t) where
	pure a = Parser \s -> (Right a, s)
	(<*>) (Parser mf) (Parser ma) = Parser \s -> case mf s of
		(Right f,s) = case ma s of
			(Right a,s) = (Right (f a),s)
			(Left m,s) = (Left m,s)
		(Left m,s) = (Left m,s)

instance Monad (Parser t) where
	bind (Parser ma) f = Parser \s -> case ma s of
		(Right a, s) = unP (f a) s
		(Left m,s) = (Left m,s)

// ---------- Top Functions ----------

parse :: (Parser t a) [t] -> Either String a
parse p i = fst $ run p i

run :: (Parser t a) [t] -> (Either String a, [t])
run (Parser p) i = case p i of
	(Right x,s) = (Right x,s)
	(Left m,s) = (Left m,s)

Start = 1
