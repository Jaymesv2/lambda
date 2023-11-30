//#![feature()]

pub mod parser;
//pub mod ast;
pub mod rename;
pub mod scoped_map;
pub mod types;

/*
    binding an existing variable
*/

// compiler stages:
//
// 1. Parsing
//     a. Lexer
//         Takes raw source and turns it into tokens
//     b. Layout
//         Inserts virtual braces and semicolons based on the layout of the program
//     c. Parsing
// 2. Renaming
//     Give each variable in the
//
