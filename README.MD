# My CPU Model & Shersh-lang

## Shersh-lang documentation

### Syntax

1. `(<function> <arg1> <arg2>...<argn>)`
2. `(объяв <name> [<arg1> <arg2>...<argn>] (<function> <farg1> <farg2>...<fargn>))`
3. `(падика <name> (<value>))`

Functions can also be used as args, but should be wrapped in brackets, e.g. \
```
(падика one_const (1))
(объяв sum [x y] (+ x y))
(объяв print_plus_one [x] (наружу (sum x one_const)))

(print_plus_one 5)
```

### Keywords

* `падика` - constant / function declaration
* `объяв` - same as `падика`, but expects args in square brackets
* `внутрь` - function, returns string line from `stdin`
* `наружу` - function, takes variable amount of strings or chars in args, prints them in `stdout`
* `+` - function, takes variable amount of integers and returns their sum
* `-` - function, takes variable amount of integers and returns their subtraction
* `*` - function, takes variable amount of integers and returns their product
* `/` - function, takes variable amount of integers and returns their division

_list will be updated_