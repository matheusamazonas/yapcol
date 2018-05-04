definition module Yapcol

import Data.Either
import Data.Functor
import Control.Applicative
import Control.Monad

:: Parser t a = Parser ([t] -> (Either String a, [t]))

instance Functor (Parser t) 
instance Applicative (Parser t) 
instance Alternative (Parser t)
instance Monad (Parser t) 

parse :: (Parser t a) [t] -> Either String a
run :: (Parser t a) [t] -> (Either String a, [t])

satisfy :: (t -> Bool) -> Parser t t
(<?>) infix 1 :: (Parser t a) String -> Parser t a
fail :: String -> Parser t a
many0 :: (Parser t a) -> Parser t [a]
many1 :: (Parser t a) -> Parser t [a]
opt :: (Parser t a) -> Parser t ()
optMaybe :: (Parser t a) -> Parser t (Maybe a)
is :: t -> Parser t t | == t
choice :: [(Parser t a)] -> Parser t a
any :: Parser t t
lookAhead :: (Parser t a) -> Parser t a
between :: (Parser t o) (Parser t c) (Parser t a) -> Parser t a
skipMany0 :: (Parser t a) -> Parser t ()
skipMany1 :: (Parser t a) -> Parser t ()
sepBy0 :: (Parser t a) (Parser t s) -> Parser t [a]
sepBy1 :: (Parser t a) (Parser t s) -> Parser t [a]
endBy0 :: (Parser t a) (Parser t s) -> Parser t [a]
endBy1 :: (Parser t a) (Parser t s) -> Parser t [a]