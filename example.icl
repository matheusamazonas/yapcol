module example

import yapcol
import StdOverloaded
import StdString

pFoo :: Parser String String
pFoo = satisfy ((==) "foo")

pBar :: Parser String String
pBar = satisfy ((==) "bar")

Start = run (pFoo <|> pBar) ["bar"]