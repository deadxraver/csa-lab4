# My CPU Model & Shersh-lang

## Shersh-lang documentation

### Syntax

1. `(<function> <arg1> <arg2>...<argn>)`
2. `(fun <name> [<arg1> <arg2>...<argn>] (<function> <farg1> <farg2>...<fargn>))`
3. `(val <name> (<value>))`

Functions can also be used as args, but should be wrapped in brackets, e.g. \
```
(val one_const (1))
(fun sum [x y] (+ x y))
(fun print_plus_one [x] (out (sum x one_const)))

(print_plus_one 5)
```

### Keywords

* `val` - constant / function declaration
* `fun` - same as `val`, but expects args in square brackets
* `in` - function, returns string line from `stdin`
* `out` - function, takes variable amount of strings or chars in args, prints them in `stdout`
* `+` - function, takes variable amount of integers and returns their sum
* `-` - function, takes variable amount of integers and returns their subtraction
* `*` - function, takes variable amount of integers and returns their product
* `/` - function, takes variable amount of integers and returns their division

_list will be updated_