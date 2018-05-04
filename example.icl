module Example

import Yapcol
import StdMisc
import StdOverloaded
import StdString

pFoo :: Parser String String
pFoo = satisfy ((==) "foo")

pBar :: Parser String String
pBar = satisfy ((==) "bar")

pFoos :: Parser String [String]
pFoos = many1 pFoo

Start = run (pFoos) ["foo","foo","foo","bar"]