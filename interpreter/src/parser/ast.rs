
use crate::parser::lexer::Token;
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // left + right, x > y
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    
    // (5 + 5)
    Grouping {
        expression: Box<Expr>,
    },
    
    // 5.0, "hello", true
    Literal {
        value: Token,
    },
    
    // -5, !true
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    
    // x, temperature
    Variable {
        name: Token,
    },

    // SK Specific: [0..1]
    Interval {
        min: Box<Expr>,
        max: Box<Expr>,
    },

    // TODO: add more stuff later!
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // let x = 5
    Let {
        name: Token,
        initializer: Expr,
    },
    
    // x = 10 (reassignment)
    Assign {
        name: Token,
        value: Expr,
    },

    // print x
    Print {
        expression: Expr,
    },

    // A simple expression like "5 + 5;" appearing as a statement
    Expression {
        expression: Expr,
    },
    
    // TODO: add more stuff later!
}