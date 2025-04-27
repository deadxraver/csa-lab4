# My CPU Model & Shersh-lang

## PROGRESS:

- formatter - **DONE** ✅
- translator - **IN PROGRESS** 🕐
- executor - **NOT STARTED** ❌

## Shersh-lang Documentation

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

* `пост` - constant declaration
* `объяв` - function declaration, expects args in square brackets
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
* `!=`/`неравны` - inline function, takes 2 args, returns `0` if they are equal, `1` if not
* `>`/`больше` - inline function, takes 2 args, returns `1` if first one is greater, `0` if not
* `?`/`если` - inline function, if arg is truthy, executes statement after, else skips it
* `<<`/`сдвг` - perform a left shift (if you need a right shift, just pass a negative number)

## Start

### Installation

If you don't have cargo, get it: `# apt install cargo -y`

To build formater, translator and executor run `cargo build --profile release`

After the compilation finishes you can find your executables here: `./target/release/`

If you want to run tests, simply run `cargo test`

### Building Your First Program

#### Formater

To see the formated code run `srs_format filename.srs`

To inplace it add a `-i` (or `--inplace`) key: `srs_format -i filename.srs`

Congrats! Now your code looks pretty

#### Translator

To build your program run `srs_translate filename.srs`

Now you have a binary `filename.o` to be executed by an _executor_

#### Executor

_To be done yet_