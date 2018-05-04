implementation module Yapcol

import Yapcol
import StdMisc
import StdOverloaded
import StdFunc
import StdTuple
import StdList
import Data.Maybe
from Data.Func import $

// ---------- Instances ----------

instance Functor (Parser t) where
	fmap f p = pure f <*> p

instance Applicative (Parser t) where
	pure a = Parser \s -> (Right a, s)
	(<*>) mf ma = mf >>= \f -> ma >>= \a -> pure $ f a

instance Alternative (Parser t) where
	empty = fail "Failed alternative"
	(<|>) (Parser p1) (Parser p2) = Parser \s -> case p1 s of
		(Right x,s) = (Right x,s)
		_ = p2 s

instance Monad (Parser t) where
	bind (Parser ma) f = Parser \s -> case ma s of
		(Right a,s) = run (f a) s
		(Left m,s) = (Left m,s)

// ---------- Top Functions ----------

parse :: (Parser t a) [t] -> Either String a
parse p i = fst $ run p i

run :: (Parser t a) [t] -> (Either String a, [t])
run (Parser p) i = p i

// ---------- Combinators ----------

satisfy :: (t -> Bool) -> Parser t t
satisfy p = Parser \s -> case s of
	[] = (Left "Empty input",[])
	[t:ts]
		| p t = (Right t, ts)
		= (Left "Unable to parse.", ts)

fail :: String -> Parser t a
fail e = Parser \s -> (Left e,s)

many0 :: (Parser t a) -> Parser t [a]
many0 p = many1 p <|> pure []

many1 :: (Parser t a) -> Parser t [a]
many1 p = (\x xs -> [x : xs]) <$> p <*> many0 p

opt :: (Parser t a) -> Parser t ()
opt p = (p >>| pure ()) <|> pure ()

optMaybe :: (Parser t a) -> Parser t (Maybe a)
optMaybe p = pure <$> p
