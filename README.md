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
  положи        0x1234        ; положили такие 0x1234 на стек
  падика                      ; сгенерировали такие рандомное число и положили на стек
  сложи                       ; сложили такие 0x1234 и рандомное число
  положи        output_addr   ; положили такие указатель на адрес вывода
  подтяни                     ; подтянули такие по указателю адрес вывода
  парень_достань_мне_в_а      ; положили такие этот адрес в регистр а
  сохрани_по_а                ; записали такие результат сложения в вывод
  останов                     ; остановили такие поцессор
```

```asm
section .data

input_addr:     dw 0x00
output_addr:    dw 0x04


section .text

main:
  push_imm      0x1234        ; stack.push(0x1234)
  push_rand                   ; stack.push(rand())
  add                         ; stack.push(0x1234 + rand())
  push_imm      output_addr   ; stack.push(&stdout)
  fetch                       ; stack.push(stdout)
  pop_a                       ; a = stdout
  store_a                     ; print(0x1234 + rand())
  halt                        ; halt the processor
```

#### Keywords

##### Non-Instruction Keywords:
* `;` - comment start
* `section` (`секция`) - sets up a code section, can be `.text` (`.текст`) for code, `.data` (`.данные`) for data
* `.org` (`.аддр`) - sets up start address for section
* `db` (`бд`), `dw` (`сд`) - define data size in `.data` (`.данные`) section (byte and word respectively)
* `main` (`старт`) - label name for entrance point (like `_start` in normal languages)

##### Instruction Keywords:
###### Register-to-Stack:
* `push_imm` (`толкни`) - one arg command, pushes the immediate value to the data stack
* `push_a` (`толкни_а`) - push value from register A to the data stack
* `push_b` (`толкни_б`) - push value from register B to the data stack
* `pop_a` (`парень_достань_мне_в_а`) - pop value from data stack and put it in register A
* `pop_b` (`достань_в_б`) - pop value from data stack and put it in register B
* `fetch_a` (`подтяни_а`) - push value to data stack from memory by address in register A
* `fetch_b` (`подтяни_б`) - push value to data stack from memory by address in register B
* `fetch` (`подтяни`) - push value to data stack from memory by address on data stack top
* `store_a` (`сохрани_по_а`) - pop value from data stack and store it by address in register A
* `store_b` (`сохрани_по_б`) - pop value from data stack and store it by address in register B
###### Stack-to-Stack:
* `dup` - duplicate value on data stack
* `d2r` - pop value from data stack and push it to return stack
* `r2d` - pop value from return stack and push it to data stack
* `swp` - swap top 2 values on data stack
###### ALU Operations:
* `add` - pop 2 values from data stack and push their sum
* `sub` - pop 2 values from data stack and push their subtraction
* `mul` - pop 2 values from data stack and push their product
* `div` - pop 2 values from data stack and push their division result
* `and` - pop 2 values from data stack and push their bitwise AND result
* `or` - pop 2 values from data stack and push their bitwise OR result
* `xor` - pop 2 values from data stack and push their bitwise XOR result
* `not` - pop 1 value from data stack and push its bitwise NOT
* `cmp` - set flags on the result of subtraction of top 2 values but not save the result and pop values
###### Control Flow:
* `call` - call subroutine
* `call_top` - jump to the address stored on top of the data stack and store return address on return stack
* `beqz` - jump if zero (zero flag set)
* `bnz` - jump if not zero (zero flag not set)
* `bn` - branch if negative (negative flag set)
* `bp` - branch if positive (negative and zero flags not set)
* `ret` - return from subroutine
_Control flow is being designed_
###### Other:
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
