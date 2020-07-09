use crate::token::TokenType;

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    // literal values
    Number(f64),
    String(&'a str),
    Boolean(bool),
    Nil,
    // compound expressions
    Binary {
        left: Box<Expr<'a>>,
        token_type: TokenType<'a>,
        right: Box<Expr<'a>>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Unary {
        token_type: TokenType<'a>,
        right: Box<Expr<'a>>,
    },
    //
    Variable {
        name: &'a str,
    },
}

#[derive(Debug)]
pub enum Stmt<'a> {
    Expression {
        expression: Expr<'a>,
    },
    Print {
        expression: Expr<'a>,
    },
    Var {
        name: &'a str,
        initializer: Option<Expr<'a>>,
    },
}