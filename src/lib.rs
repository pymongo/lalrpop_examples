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

## lalrpop 全局 match
可以用来跳过 comment 行的解析
还能用于解决 ambiguities 问题，例如全局的优先将 "from" 解析成 keyword 而非 ident

## <lpos:@L> 表示什么意思

use for custom lexer Location tracking

## match case-insensitive keyword

多个方法:
1. 迭代 Vec<String> keywords 然后 str::eq_ignore_ascii_case
2. phf_map, input.to_upper_case
3. (?i)select
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
#[cfg(test)]
mod json;
