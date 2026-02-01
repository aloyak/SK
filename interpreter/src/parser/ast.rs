use crate::parser::lexer::TokenSpan;
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // left + right, x > y
    Binary {
        left: Box<Expr>,
        operator: TokenSpan,
        right: Box<Expr>,
    },

    // (5 + 5)
    Grouping {
        expression: Box<Expr>,
    },

    // 5.0, "hello", true
    Literal {
        value: TokenSpan,
    },
    
    // -5, !true
    Unary {
        operator: TokenSpan,
        right: Box<Expr>,
    },

    // x, temperature
    Variable {
        name: TokenSpan,
    },

    // Intervals: [0..1]
    Interval {
        min: Box<Expr>,
        max: Box<Expr>,
        bracket: TokenSpan, 
    },

    Block { 
        statements: Vec<Stmt> 
    },

    Call {
        callee: Box<Expr>,
        paren: TokenSpan, 
        arguments: Vec<Expr>,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfPolicy {
    Strict,
    Merge,
    Panic,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // let x = 5
    Let {
        name: TokenSpan,
        initializer: Expr,
    },

    // x = 10 (reassignment)
    Assign {
        name: TokenSpan,
        value: Expr,
    },

    Symbolic {
        name: TokenSpan,
        initializer: Expr,
        is_quiet: bool,
    },

    // print(x)
    Print {
        expression: Expr,
    },

    Panic,
    
    // A simple expression like "5 + 5;" appearing as a statement
    Expression {
        expression: Expr,
    },

    Block {
        statements: Vec<Stmt>,
    },

    If {
        condition: Expr,
        policy: IfPolicy,
        then_branch: Box<Stmt>,
        elif_branch: Vec<(Expr, Stmt)>, // List of (condition, body)
        else_branch: Option<Box<Stmt>>,
    },

    Function {
        name: TokenSpan,
        params: Vec<TokenSpan>,
        body: Vec<Stmt>
    }
}