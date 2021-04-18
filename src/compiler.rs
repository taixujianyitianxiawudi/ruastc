pub mod intermediate_code_generator;
pub mod lexer;
pub mod native_code_generator;
pub mod parser;

use std::fmt;
pub use intermediate_code_generator::generate_intermediate_code;
pub use lexer::lexing;
pub use native_code_generator::generate_native_code;
pub use parser::parsing;

pub enum LetterKind {
    LtNum,//0-9
    LtSpace,//空格
    LtOperator,//+-*/
    LtComparisonOperator,//<>=!
    LtParenthesis,//()
    LtSymbol,//;
    LtAlphabet,//a-z
    LtBraces,//{}
}

pub enum LexerCondition {
    CondCompletion,
    CondMiddleOfNumber,
    CondMiddleOfComparisonOperator,
    CondMiddleOfVariable,
}
#[derive(Debug)]
pub enum TokenKind {
    TkNum,
    TkOperator,
    TkComparisonOperator,
    TkParenthesis,
    TkSymbol,
    TkVariable,
    TkReturn,
    TkMain,
    TkInt,
    TkBraces,
}
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub val: String,
}
impl fmt::Display for Token {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}
pub enum NodeKind {
    NdNum,
    NdOperator,
    NdComparisonOperator,
    NdAssignOperator,
    NdVariable,
    NdReturn,
}

pub struct Node {
    pub val: String,
    pub kind: NodeKind,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
