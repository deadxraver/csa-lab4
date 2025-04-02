# My CPU Model & Shersh-lang

## Shersh-lang documentation

### Syntax

1. `(<function> <arg1> <arg2>...<argn>)`
2. `(объяв <name> [<arg1> <arg2>...<argn>] (<function> <farg1> <farg2>...<fargn>))`
3. `(пост <name> (<value>))`

Functions can also be used as args, but should be wrapped in brackets, e.g.

```lisp
(пост one_const (1))
(объяв sum [x y] (+ x y))
(объяв print_plus_one [x] (наружу (sum x one_const)))

(print_plus_one 5)
```

### Keywords

* `пост` - constant / function declaration
* `объяв` - same as `пост`, but expects args in square brackets
* `внутрь` - function, returns string line from `stdin`
* `наружу` - function, takes variable amount of strings or chars in args, prints them in `stdout`
* `@хр` - annotation for tail recursion optimisation
* `себя` - recursively call function
* `+`/`слож` - function, takes variable amount of integers and returns their sum
* `-`/`вычит` - function, takes variable amount of integers and returns their subtraction
* `*`/`произв` - function, takes variable amount of integers and returns their product
* `/`/`частн` - function, takes variable amount of integers and returns their division
* `=`/`равны` - function, takes variable amount of args, returns `1` if they are equal, `0` if not
* `!=`/`неравны` - function, takes variable amount of args, returns `0` if they are equal, `1` if not

_list will be updated_
