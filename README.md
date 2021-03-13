# VSL Programming Language
## Project Goal
The purpose of this project is to create a new programming language that focuses on security to protect against mistakes that may lead to the release of confidential information. This new language will introduce a feature called “variable security.” The idea behind variable security is that there are certain data sets within a program that may be more sensitive than others. Some languages such as C++ or Java allow for the programmer to declare private and public variables within a class, however, variable security takes this a step further and allows the programmer to introduce multiple levels of security priorities for variables. This means that variables could potentially have hierarchies of security within the same program and within the same function. These hierarchies will define how certain variables can interact within one another. The new programming language will be developed by first creating a stack based virtual machine and then writing a compiler that takes a high-level readable language and translates it to the byte-code of the virtual machine. The virtual machine will be developed as a 64 bit machine to allow the language to take full advantage of modern hardware and operating systems. Other than the variable security feature, the language will have a small feature set including functions, loops, if / if else statements, input / output capabilities, integers, floats, strings, and arrays. The intent of this language is to provide companies a cross platform programming language that makes handling sensitive or confidential information intuitive and easy for developers to limit data leaks.

## How to Install and Use
### Windows
1. Download the Windows binaries from releases.
2. Download Windows terminal from the Microsoft Store (The default cmd and powershell applications are notoriously awful).
3. Create a vsl file such as 'main.vsl'
4. Enter either some example code or code for a new program.
5. In the directory of the binary files and source file type './vsl_compiler main.vsl' in your terminal to compile program.
6. In the directory of the binary files and source file type './cmvm program' in your terminal to run the program.

### Linux
1. Download the Linux binaries from releases.
3. Create a vsl file such as 'main.vsl'
4. Enter either some example code or code for a new program.
5. In the directory of the binary files and source file type './vsl_compiler main.vsl' in your terminal to compile program.
6. In the directory of the binary files and source file type './cmvm program' in your terminal to run the program.

### Mac
Same as Linux.

### Example
[Example Video](https://youtu.be/I007YlARifE)

## Sample Code
### Hello World
```typescript
fn void main() {
    print("Hello, World!\n");
    return;
}
```
### Factorial
```typescript
fn void main() {
    let fact int:0 = 14;
    print("Factorial of 14: ", factorial(fact), "\n");
    return;
}

fn int:0 factorial(fact int:0) {
    if fact <= 1 {
        return 1;
    }
    return fact * factorial(fact - 1);
}
```

## Language Rules
1. All code must be contained within a function.
2. All functions must return within the root scope of the function (even void functions with 'return;').
3. All variables passed to functions will be passed by value (execpt for arrays).
4. All statements except if and while loops must end with a semicolon.
5. The syntax for defining new variables is as follows:
    let_keyword identifier variable_type:security_value = expression.


## To Do List
1. Implement the 'read();' function to get input from user.
2. Finish string operations.
3. Implement arrays for the int, float, and string data types. (And char indexing for string type)
4. Implement garbage collection in the VM.