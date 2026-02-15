use crate::parser::lexer::TokenSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: TokenSpan,
        right: Box<Expr>,
    },

    Grouping {
        expression: Box<Expr>,
    },

    Literal {
        value: TokenSpan,
    },
    
    Unary {
        operator: TokenSpan,
        right: Box<Expr>,
    },

    Variable {
        name: TokenSpan,
    },

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
    },

    Get {
        object: Box<Expr>,
        name: TokenSpan,
    },

    Postfix {
        name: TokenSpan,
        operator: TokenSpan,
    },

    Quantity {
        value: Box<Expr>,
        unit: UnitExpr,
    },

    Array {
        elements: Vec<Expr>,
        bracket: TokenSpan,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
        bracket: TokenSpan,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitExpr {
    Name(TokenSpan),
    Mul(Box<UnitExpr>, Box<UnitExpr>),
    Div(Box<UnitExpr>, Box<UnitExpr>),
    Pow(Box<UnitExpr>, i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Import {
        path: TokenSpan,
        alias: Option<TokenSpan>,
    },

    Let {
        name: TokenSpan,
        initializer: Expr,
    },

    Assign {
        name: TokenSpan,
        value: Expr,
    },

    Symbolic {
        name: TokenSpan,
        initializer: Expr,
        is_quiet: bool,
    },

    Print {
        expression: Expr,
    },

    Panic,
    
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
        params: Vec<Parameter>,
        body: Vec<Stmt>,
        is_public: bool
    },

    Loop {
        body: Vec<Stmt>,
    },
    For {
        variable: TokenSpan,
        iterable: Expr,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IfPolicy {
    Strict,
    Merge,
    Panic,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: TokenSpan,
    pub default: Option<Expr>,
}