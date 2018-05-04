definition module yapcol

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