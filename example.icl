module Example

import Yapcol
import StdMisc
import StdOverloaded
import StdString
import Data.Maybe

pFoo :: Parser String String
pFoo =  is "foo"

pBar :: Parser String String
pBar = is "bar"

pFoos :: Parser String [String]
pFoos = many1 pFoo

pTest :: (Maybe String) -> Parser String String
pTest (Just s) = satisfy ((==) s)
pTest Nothing = fail "FAIL"

Start = run (choice [pFoo, pBar]) ["foo","foo","foo","bar"]