use crate::compiler::*;

pub fn lexing(input: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut begin_idx = 0;
    let mut condition = LexerCondition::CondCompletion;
    for (i, s) in input.chars().enumerate() {
        match return_letter_kind(s) {
            LetterKind::LtNum => match condition {
                LexerCondition::CondMiddleOfNumber => {}
                LexerCondition::CondMiddleOfComparisonOperator => {
                    let new_token = Token {
                        kind: TokenKind::TkComparisonOperator,
                        val: check_valid_token(&input[begin_idx..i]).to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfNumber;
                }
                LexerCondition::CondMiddleOfVariable => {
                    //tokens.push(create_token_of_variable(&input[begin_idx..i]));
                    //begin_idx = i;
                    //condition = LexerCondition::CondMiddleOfNumber;
                    panic!("can insert numbers in a variable")
                }
                
                LexerCondition::CondCompletion => {
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfNumber;
                }
            },
            LetterKind::LtSpace => match condition {
                LexerCondition::CondMiddleOfNumber => {
                    let new_token = Token {
                        kind: TokenKind::TkNum,
                        val: input[begin_idx..i].to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i + 1;
                    condition = LexerCondition::CondCompletion;
                }
                LexerCondition::CondMiddleOfComparisonOperator => {
                    let new_token = Token {
                        kind: TokenKind::TkComparisonOperator,
                        val: input[begin_idx..i].to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i + 1;
                    condition = LexerCondition::CondCompletion;
                }
                LexerCondition::CondMiddleOfVariable => {
                    tokens.push(create_token_of_variable(&input[begin_idx..i]));
                    begin_idx = i + 1;
                    condition = LexerCondition::CondCompletion;
                }
                LexerCondition::CondCompletion => {
                    begin_idx = i + 1;
                }
            },
            LetterKind::LtOperator | LetterKind::LtParenthesis 
            | LetterKind::LtSymbol | LetterKind::LtBraces => {
                let new_tokenkind;
                match return_letter_kind(s) {
                    LetterKind::LtParenthesis => {
                        new_tokenkind = TokenKind::TkParenthesis;
                    }
                    LetterKind::LtSymbol => {
                        new_tokenkind = TokenKind::TkSymbol;
                    }
                    LetterKind::LtBraces => {
                        new_tokenkind = TokenKind::TkBraces;
                    }
                    _ => {
                        new_tokenkind = TokenKind::TkOperator;
                    }
                }
                match condition {
                    LexerCondition::CondMiddleOfNumber => {
                        let new_token = Token {
                            kind: TokenKind::TkNum,
                            val: input[begin_idx..i].to_string(),
                        };
                        tokens.push(new_token);
                        let new_token = Token {
                            kind: new_tokenkind,
                            val: String::from(s.to_string()),
                        };
                        tokens.push(new_token);
                        begin_idx = i + 1;
                        condition = LexerCondition::CondCompletion;
                    }
                    LexerCondition::CondMiddleOfComparisonOperator => {
                        let new_token = Token {
                            kind: TokenKind::TkComparisonOperator,
                            val: check_valid_token(&input[begin_idx..i]).to_string(),
                        };
                        tokens.push(new_token);
                        let new_token = Token {
                            kind: new_tokenkind,
                            val: String::from(s.to_string()),
                        };
                        tokens.push(new_token);
                        begin_idx = i + 1;
                        condition = LexerCondition::CondCompletion;
                    }
                    LexerCondition::CondMiddleOfVariable => {
                        tokens.push(create_token_of_variable(&input[begin_idx..i]));
                        let new_token = Token {
                            kind: new_tokenkind,
                            val: String::from(s.to_string()),
                        };
                        tokens.push(new_token);
                        begin_idx = i + 1;
                        condition = LexerCondition::CondCompletion;
                    }
                    LexerCondition::CondCompletion => {
                        let new_token = Token {
                            kind: new_tokenkind,
                            val: String::from(s.to_string()),
                        };
                        tokens.push(new_token);
                        begin_idx = i + 1;
                    }
                }
            }
            LetterKind::LtComparisonOperator => match condition {
                LexerCondition::CondMiddleOfNumber => {
                    let new_token = Token {
                        kind: TokenKind::TkNum,
                        val: input[begin_idx..i].to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfComparisonOperator;
                }
                LexerCondition::CondMiddleOfComparisonOperator => {
                    let new_token = Token {
                        kind: TokenKind::TkComparisonOperator,
                        val: check_valid_token(&input[begin_idx..i + 1]).to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i;
                    condition = LexerCondition::CondCompletion;
                }
                LexerCondition::CondMiddleOfVariable => {
                    tokens.push(create_token_of_variable(&input[begin_idx..i]));
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfComparisonOperator;
                }
                _ => {
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfComparisonOperator;
                }
            },
            LetterKind::LtAlphabet => match condition {
                LexerCondition::CondMiddleOfNumber => {
                    let new_token = Token {
                        kind: TokenKind::TkNum,
                        val: input[begin_idx..i].to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfVariable;
                }
                LexerCondition::CondMiddleOfComparisonOperator => {
                    let new_token = Token {
                        kind: TokenKind::TkComparisonOperator,
                        val: check_valid_token(&input[begin_idx..i]).to_string(),
                    };
                    tokens.push(new_token);
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfVariable;
                }
                LexerCondition::CondMiddleOfVariable => {
                    condition = LexerCondition::CondMiddleOfVariable;
                }
                _ => {
                    begin_idx = i;
                    condition = LexerCondition::CondMiddleOfVariable;
                }
            },
        }
    }
    tokens
}



fn return_letter_kind(s: char) -> LetterKind {
    match s {
        '0'..='9' => LetterKind::LtNum,
        'a'..='z' => LetterKind::LtAlphabet,
        ' ' | '\n' => LetterKind::LtSpace,
        '+' | '-' | '*' | '/' => LetterKind::LtOperator,
        '(' | ')' => LetterKind::LtParenthesis,
        ';' => LetterKind::LtSymbol,
        '<' | '>' | '=' | '!' => LetterKind::LtComparisonOperator,
        '{' | '}' => LetterKind::LtBraces,
        _ => {
            panic!("Cannot recognize {}", s);
        }
    }
}

fn check_valid_token(s: &str) -> &str {
    match s {
        "<" | ">" | "<=" | ">=" | "==" | "!=" | "=" => s,
        _ => {
            panic!("Not a valid token : {}", s);
        }
    }
}

fn create_token_of_variable(s: &str) -> Token {
    match s {
        "return" => {
            return Token {
                kind: TokenKind::TkReturn,
                val: String::from("return"),
            };
        }
        "int" => {
            return Token {
                kind: TokenKind::TkInt,
                val: String::from("int"),
            };
        }
        "main" => {
            return Token {
                kind: TokenKind::TkMain,
                val: String::from("main"),
            };
        }
        _ => {
            return Token {
                kind: TokenKind::TkVariable,
                val: String::from(s),
            };
        }
    }
}