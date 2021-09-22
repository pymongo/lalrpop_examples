/*!
## 和类型 和 积类型
和类型:
```
enum A {
    A1(bool),
    A2(bool),
    A3(bool),
}
```
A 一共有 2+2+2 种可能的状态

积类型:
```
struct A {
    a1: bool,
    a2: bool,
    a3: bool,
}
```
A 一共有 2*2*2 种可能的状态
*/
#[cfg(test)]
#[macro_use]
extern crate lalrpop_util;

#[cfg(test)]
mod calculator;
#[cfg(test)]
mod calculator_ast;
#[cfg(test)]
mod parsing_parenthesized_numbers;
#[cfg(test)]
mod test_regex;
