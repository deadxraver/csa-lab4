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
* `лок` - local variable declaration
* `внутрь` - function, returns string line from `stdin`
* `наружу` - function, takes variable amount of strings or chars in args, prints them in `stdout`
* `@хр` - annotation for tail recursion optimisation
* `себя` - recursively call function
* `+`/`слож` - inline function, takes variable amount of integers and returns their sum
* `-`/`вычит` - inline function, takes 2 integers and returns their subtraction
* `*`/`произв` - inline function, takes variable amount of integers and returns their product
* `/`/`частн` - inline function, takes 2 integers and returns their division
* `%`/`остат` - inline function, takes 2 integers and returns their remainder
* `=`/`равны` - inline function, takes 2 args, returns `1` if they are equal, `0` if not
* `!=`/`неравны` - inline function, takes 2 of args, returns `0` if they are equal, `1` if not
* `?`/`если` - inline function, if arg is truthy, executes statement after, else skips it

_list will be updated_
