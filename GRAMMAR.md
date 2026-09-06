# FAMT Language Grammar Specification v0.1

## Overview
FAMT is a procedural scripting language with Java-like syntax, indentation-based structure, and JIT compilation targeting a custom VM.

---

## 1. Lexical Structure

### 1.1 Whitespace and Indentation
- **Whitespace**: Space, tab, and newline are used for indentation
- **Indentation**: 4 spaces = 1 indentation level (or 1 tab)
- Significant whitespace for block structure (Python-style)

### 1.2 Comments
```
// Single-line comment
/* Multi-line comment
   can span multiple lines
*/
```

### 1.3 Keywords
```
func        // Function declaration
if          // Conditional
else        // Alternative branch
for         // Loop
while       // While loop
do          // Do-while loop
break       // Break statement
continue    // Continue statement
return      // Return from function
int         // Integer type
float       // Float type
string      // String type
bool        // Boolean type
array       // Array type
void        // Void return type
true        // Boolean literal
false       // Boolean literal
null        // Null value
```

### 1.4 Identifiers
- Start with letter or underscore: `[a-zA-Z_]`
- Followed by letters, digits, or underscores: `[a-zA-Z0-9_]*`
- Case-sensitive

### 1.5 Literals

#### Integer Literals
```
0
123
-456
0xFF          // Hexadecimal
0b1010        // Binary
0o755         // Octal
```

#### Float Literals
```
3.14
-2.5
1.0e10
2.5e-3
```

#### String Literals
```
"Hello, World!"
'Single quotes also work'
"Escape sequences: \n \t \\ \" \'"
```

#### Boolean Literals
```
true
false
```

#### Array Literals
```
[1, 2, 3, 4, 5]
["a", "b", "c"]
[true, false, true]
[]            // Empty array
```

### 1.6 Operators

#### Arithmetic Operators
```
+       // Addition
-       // Subtraction
*       // Multiplication
/       // Division
%       // Modulo
++      // Increment
--      // Decrement
```

#### Comparison Operators
```
==      // Equal
!=      // Not equal
<       // Less than
>       // Greater than
<=      // Less than or equal
>=      // Greater than or equal
```

#### Logical Operators
```
&&      // Logical AND
||      // Logical OR
!       // Logical NOT
```

#### Assignment Operators
```
=       // Assignment
+=      // Add and assign
-=      // Subtract and assign
*=      // Multiply and assign
/=      // Divide and assign
%=      // Modulo and assign
```

#### Bitwise Operators
```
&       // Bitwise AND
|       // Bitwise OR
^       // Bitwise XOR
~       // Bitwise NOT
<<      // Left shift
>>      // Right shift
```

#### Other Operators
```
.       // Member access
[]      // Array/index access
()      // Function call / Grouping
,       // Separator
:       // Type annotation
```

---

## 2. Data Types

### 2.1 Primitive Types
```
int         // 64-bit integer
float       // 64-bit floating point
string      // Unicode string
bool        // Boolean (true/false)
void        // No value (functions only)
```

### 2.2 Complex Types
```
array<T>    // Array of type T
```

### 2.3 Type Inference
The language supports type inference for variables when the type can be determined from the right-hand side.

---

## 3. Variables and Constants

### 3.1 Variable Declaration
```
int x = 10
float pi = 3.14
string name = "FAMT"
bool flag = true
array<int> numbers = [1, 2, 3, 4, 5]
```

### 3.2 Type Inference Declaration
```
x = 10              // Inferred as int
y = 3.14            // Inferred as float
z = "Hello"         // Inferred as string
w = true            // Inferred as bool
arr = [1, 2, 3]     // Inferred as array<int>
```

### 3.3 Array Declarations
```
array<int> nums = [1, 2, 3]
array<string> words = ["hello", "world"]
int[] numbers = [10, 20, 30]        // Alternative syntax
```

---

## 4. Functions

### 4.1 Function Declaration
```
func add(int a, int b) int
    return a + b

func greet(string name)
    print("Hello, " + name)

func swap(array<int> arr, int i, int j)
    int temp = arr[i]
    arr[i] = arr[j]
    arr[j] = temp
```

### 4.2 Function with No Parameters
```
func getCurrentTime() int
    return 1234567890
```

### 4.3 Function with No Return Value
```
func printMessage(string msg)
    print(msg)
```

### 4.4 Function Calls
```
add(5, 3)
greet("World")
print("Hello")
len(myArray)
```

### 4.5 Main Function
```
func main()
    print("FAMT Program Started")
    // Program code here
```

---

## 5. Control Flow

### 5.1 If-Else Statement
```
if x > 5
    print("x is greater than 5")
else if x == 5
    print("x is equal to 5")
else
    print("x is less than 5")
```

### 5.2 For Loop
```
// C-style for loop
for int i = 0; i < 10; i++
    print(i)

// For loop with multiple expressions
for int i = 0, j = 10; i < j; i++, j--
    print(i + j)
```

### 5.3 While Loop
```
while x < 100
    x = x + 1
    print(x)
```

### 5.4 Do-While Loop
```
do
    print(x)
    x = x + 1
while x < 100
```

### 5.5 Break and Continue
```
for int i = 0; i < 10; i++
    if i == 5
        break
    if i == 2
        continue
    print(i)
```

---

## 6. Operators and Expressions

### 6.1 Operator Precedence (High to Low)
```
1. ()  []  .              // Parentheses, array access, member access
2. ++  --  !  ~  -        // Unary operators
3. *  /  %                // Multiplicative
4. +  -                   // Additive
5. <<  >>                 // Bitwise shift
6. <  >  <=  >=           // Relational
7. ==  !=                 // Equality
8. &                      // Bitwise AND
9. ^                      // Bitwise XOR
10. |                     // Bitwise OR
11. &&                    // Logical AND
12. ||                    // Logical OR
13. =  +=  -=  *=  /=  %= // Assignment
```

### 6.2 Expression Examples
```
2 + 3 * 4                 // 14
(2 + 3) * 4               // 20
x > 5 && y < 10           // Logical AND
arr[0] + arr[1]           // Array access
!flag || (x && y)         // Mixed logic
```

---

## 7. String Operations

### 7.1 String Concatenation
```
"Hello" + " " + "World"   // "Hello World"
"Number: " + 42           // "Number: 42"
```

### 7.2 String Indexing
```
string s = "Hello"
s[0]                      // 'H'
s[1]                      // 'e'
```

### 7.3 String Functions (Built-in)
```
len(str)                  // Length of string
substring(str, start, end)
toUpperCase(str)
toLowerCase(str)
indexOf(str, substr)
replace(str, old, new)
```

---

## 8. Array Operations

### 8.1 Array Indexing
```
array<int> arr = [10, 20, 30]
arr[0]                    // 10
arr[2]                    // 30
arr[1] = 25               // Modify element
```

### 8.2 Array Functions (Built-in)
```
len(arr)                  // Length of array
push(arr, value)          // Add to end
pop(arr)                  // Remove from end
shift(arr)                // Remove from start
unshift(arr, value)       // Add to start
reverse(arr)              // Reverse array
sort(arr)                 // Sort array
```

---

## 9. Built-in Functions

### 9.1 I/O Functions
```
print(value)              // Print to stdout
println(value)            // Print with newline
input()                   // Read from stdin
```

### 9.2 Type Conversion Functions
```
toInt(value)              // Convert to integer
toFloat(value)            // Convert to float
toString(value)           // Convert to string
toBool(value)             // Convert to boolean
```

### 9.3 Math Functions
```
abs(x)                    // Absolute value
sqrt(x)                   // Square root
pow(x, y)                 // Power
floor(x)                  // Floor
ceil(x)                   // Ceiling
round(x)                  // Round
min(a, b)                 // Minimum
max(a, b)                 // Maximum
random()                  // Random float [0, 1)
randomInt(max)            // Random int [0, max)
```

### 9.4 Array Functions
```
len(arr)                  // Get array length
push(arr, value)          // Append element
pop(arr)                  // Remove last element
shift(arr)                // Remove first element
unshift(arr, value)       // Prepend element
reverse(arr)              // Reverse array
sort(arr)                 // Sort array in place
indexOf(arr, value)       // Find index of value
contains(arr, value)      // Check if contains value
```

---

## 10. Example Programs

### 10.1 Hello World
```famt
func main()
    print("Hello, World!")
```

### 10.2 Factorial
```famt
func factorial(int n) int
    if n <= 1
        return 1
    else
        return n * factorial(n - 1)

func main()
    print(factorial(5))
```

### 10.3 Fibonacci
```famt
func fib(int n) int
    if n <= 1
        return n
    return fib(n - 1) + fib(n - 2)

func main()
    for int i = 0; i < 10; i++
        print(fib(i))
```

### 10.4 Array Sum
```famt
func sumArray(array<int> arr) int
    int sum = 0
    for int i = 0; i < len(arr); i++
        sum = sum + arr[i]
    return sum

func main()
    array<int> numbers = [1, 2, 3, 4, 5]
    print(sumArray(numbers))
```

### 10.5 String Manipulation
```famt
func main()
    string text = "Hello, FAMT!"
    print("Original: " + text)
    print("Length: " + len(text))
    print("Uppercase: " + toUpperCase(text))
    print("Substring: " + substring(text, 0, 5))
```

### 10.6 Nested Loops with Break
```famt
func main()
    for int i = 0; i < 5; i++
        for int j = 0; j < 5; j++
            if i * j > 6
                break
            print(i + "," + j)
```

---

## 11. Scope and Lifetime

### 11.1 Global Scope
Variables declared outside functions are global.

### 11.2 Local Scope
Variables declared inside functions are local to that function.

### 11.3 Block Scope
Variables declared inside blocks (if, for, while) are scoped to that block.

```famt
func example()
    int x = 10          // Function scope
    if x > 5
        int y = 20      // Block scope
        print(y)        // OK
    print(y)            // Error: y is out of scope
```

---

## 12. Type Coercion

### 12.1 Automatic Type Conversion
- String concatenation with `+` converts operands to string
- Arithmetic operations promote types as needed
- Boolean to int: true = 1, false = 0
- Int to float: automatic promotion

### 12.2 Explicit Type Conversion
```
int x = toInt("42")
float y = toFloat(10)
string s = toString(3.14)
bool b = toBool(1)
```

---

## 13. Reserved Words That Are NOT Keywords

These words are recognized but not reserved:

```
print, println, len, push, pop, shift, unshift, reverse, sort, indexOf, contains
abs, sqrt, pow, floor, ceil, round, min, max, random, randomInt
toInt, toFloat, toString, toBool, substring, toUpperCase, toLowerCase, replace
```

---

## 14. Error Handling (v0.2+)

Reserved for future implementation:
```
try
    // Code that might fail
catch Exception e
    // Handle exception
finally
    // Cleanup
```

---

## 15. Grammar Rules (BNF-like Notation)

```
Program         ::= (FunctionDecl | Statement)*

FunctionDecl    ::= "func" Identifier "(" ParameterList? ")" ReturnType?
                    INDENT
                        Statement+
                    DEDENT

ParameterList   ::= Parameter ("," Parameter)*
Parameter       ::= Type Identifier

ReturnType      ::= Type

Block           ::= INDENT Statement+ DEDENT

Statement       ::= VarDecl
                  | Assignment
                  | IfStatement
                  | ForLoop
                  | WhileLoop
                  | DoWhileLoop
                  | BreakStatement
                  | ContinueStatement
                  | ReturnStatement
                  | ExpressionStatement

VarDecl         ::= Type Identifier "=" Expression
                  | Identifier "=" Expression

IfStatement     ::= "if" Expression
                    INDENT
                        Statement+
                    DEDENT
                    ("else" "if" Expression INDENT Statement+ DEDENT)*
                    ("else" INDENT Statement+ DEDENT)?

ForLoop         ::= "for" ForInit ";" Expression ";" ForUpdate
                    INDENT
                        Statement+
                    DEDENT

WhileLoop       ::= "while" Expression
                    INDENT
                        Statement+
                    DEDENT

DoWhileLoop     ::= "do"
                    INDENT
                        Statement+
                    DEDENT
                    "while" Expression

Expression      ::= OrExpression
OrExpression    ::= AndExpression ("||" AndExpression)*
AndExpression   ::= BitwiseOrExpr ("&&" BitwiseOrExpr)*
BitwiseOrExpr   ::= BitwiseXorExpr ("|" BitwiseXorExpr)*
BitwiseXorExpr  ::= BitwiseAndExpr ("^" BitwiseAndExpr)*
BitwiseAndExpr  ::= EqualityExpr ("&" EqualityExpr)*
EqualityExpr    ::= RelationalExpr (("==" | "!=") RelationalExpr)*
RelationalExpr  ::= ShiftExpr (("<" | ">" | "<=" | ">=") ShiftExpr)*
ShiftExpr       ::= AdditiveExpr (("<<" | ">>") AdditiveExpr)*
AdditiveExpr    ::= MultiplicativeExpr (("+" | "-") MultiplicativeExpr)*
MultiplicativeExpr ::= UnaryExpr (("*" | "/" | "%") UnaryExpr)*
UnaryExpr       ::= ("!" | "~" | "-" | "+" | "++" | "--") UnaryExpr
                  | PostfixExpr
PostfixExpr     ::= PrimaryExpr (PostfixOp)*
PostfixOp       ::= "++" | "--" | "[" Expression "]" | "(" ArgumentList? ")" | "." Identifier

PrimaryExpr     ::= Identifier
                  | Literal
                  | "(" Expression ")"
                  | FunctionCall
                  | ArrayLiteral

Type            ::= "int" | "float" | "string" | "bool" | "void"
                  | "array" "<" Type ">"
                  | Type "[]"

Literal         ::= IntegerLiteral
                  | FloatLiteral
                  | StringLiteral
                  | BooleanLiteral
                  | NullLiteral

ArrayLiteral    ::= "[" (Expression ("," Expression)*)? "]"
```

---

## 16. Notes

- **Indentation is Mandatory**: Python-style indentation
- **Semicolons are Optional**: Newline terminates statements
- **Type Inference**: Variables can be declared without explicit types
- **JIT Compilation**: Programs are compiled to VM bytecode at runtime
- **No Classes/Objects**: Purely procedural (as per v0.1 design)
- **First-Class Functions (Future)**: To be added in v0.2+

---

**FAMT v0.1 Grammar Specification - Complete**
