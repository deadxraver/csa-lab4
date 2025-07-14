# My CPU Model & Alpha-Shersh-Murderer & Shersh-lang (is suspended)

## Alpha-Shersh-Murderer (ASM)

### Documentation

Alpha-Shersh-Murderer (ASM) is a low-level programming language using f18a-based ISA with a little bit of fun.
ISA offers data manipulations using data and return stacks + 2 registers.
Macro is not supported (maybe one day but not rn).

#### Syntax Example

Pretty regular asm syntax
1. `add` - regular syntax no arg instruction
2. `push_imm 0xDEAD_1234` - regular syntax one arg instruction
3. `падика` - roflosyntax no arg instruction

```asm
секция .данные

input_addr:     сд 0x00
output_addr:    сд 0x04


секция .текст

старт:
  положи        0x1234
  ; TODO: finish example code after finishing with syntax
  останов
```

#### Keywords

##### Non-Instruction Keywords:
* `;` - comment start
* `section` (`секция`) - sets up a code section, can be `.text` (`.текст`) for code, `.data` (`.данные`) for data
* `.org` (`.аддр`) - sets up start address for section
* `db` (`бд`), `dw` (`сд`) - define data size in `.data` (`.данные`) section (byte and word respectively)
* `main` (`старт`) - label name for entrance point (like `_start` in normal languages)

##### Instruction Keywords:
* `push_imm` (`положи`) - one arg command, pushes the immediate value to the data stack
_List will be updated in time_
* `push_rand` (`падика`) - push random word to stack
* `halt` (`останов`) - halt the machine

## Shersh-lang (is not currently developing - see previous commits)

### Documentation

Shersh-lang (SRS) is a high-level lisp-based syntax programming language, which is translated into Alpha-Shersh-Murderer (ASM).

#### Syntax Example

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

#### Keywords

* `пост` - constant declaration
* `объяв` - function declaration, expects args in square brackets
* `внутрь` - function, returns string line from `stdin`
* `наружу` - function, takes variable amount of strings or chars in args, prints them in `stdout`
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

### Start

#### Installation

If you don't have cargo, get it: `# apt install cargo -y`

To build formater, translator and executor run `cargo build --profile release`

After the compilation finishes you can find your executables here: `./target/release/`

If you want to run tests, simply run `cargo test`

#### Building Your First Program

##### Formater

To see the formated code run `srs_format filename.srs`

To inplace it add a `-i` (or `--inplace`) key: `srs_format -i filename.srs`

Congrats! Now your code looks pretty

##### Translator

To build your program run `srs_translate filename.srs`

Now you have a binary `filename.o` to be executed by an _executor_

##### Executor

_To be done yet_
