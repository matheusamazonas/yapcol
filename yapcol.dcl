definition module yapcol

import Data.Either
import Data.Functor
import Control.Applicative
import Control.Monad

:: Parser t a = Parser ([t] -> (Either String a, [t]))

parse :: (Parser t a) [t] -> Either String a
run :: (Parser t a) [t] -> (Either String a, [t])
