use crate::{
    fileposition::FilePosition,
    tokens::{Operator, Unary},
    ttype::TType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Arg {
    pub identifier: String,
    pub ttype: TType,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub identifier: String,
    pub ttype: TType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub input: Vec<Arg>,
    pub output: TType,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    GenericFunction,
    Variable,
    Constructor,
    Parameter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub id: String,
    pub ttype: TType,
    pub pos: Option<FilePosition>,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ast {
    pub program: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Continue,
    Break,
    Pass,
    // type id value
    Let {
        ttype: TType,
        identifier: String,
        expr: Expr,
        global: bool,
    },
    // type id input output
    Function(TType, String, Vec<Arg>, Vec<Statement>),
    // type id fields
    Struct(TType, String, Vec<Field>),
    // type exression
    Return(TType, Expr, usize, usize),
    Expression(TType, Expr),
    // type test body {else}
    If(TType, Expr, Vec<Statement>, Option<Vec<Statement>>),
    While(Expr, Vec<Statement>),
    For(Expr, Expr, Expr, Vec<Statement>),
    Block(Vec<Statement>, String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    None,
    Char(char),
    Bool(bool),
    Id(String),
    Float(f64),
    String(String),
    Integer(i64),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Closure(TType, Vec<Arg>, Vec<Statement>, Vec<String>),
    ListConstructor(TType, Vec<Expr>),
    Field(TType, String, usize, Box<Expr>, FilePosition),
    Indexed(TType, String, Box<Expr>, Box<Expr>, FilePosition),
    Call(TType, String, Box<Expr>, Vec<Expr>),
    Unary(TType, Unary, Box<Expr>),
    Binop(TType, Operator, Box<Expr>, Box<Expr>),
    Literal(TType, Atom),
    None,
}

impl Expr {
    pub fn get_type(&self) -> TType {
        match self {
            Expr::Unary(t, _, _) => t.clone(),
            Expr::Binop(t, _, _, _) => t.clone(),
            Expr::Literal(t, _) => t.clone(),
            Expr::Field(t, _, _, _, _) => t.clone(),
            Expr::ListConstructor(t, _) => t.clone(),
            Expr::Indexed(t, _, _, _, _) => t.clone(),
            Expr::None => TType::None,
            Expr::Call(t, _, _, _) => t.clone(),
            Expr::Closure(t, _, _, _) => t.clone(),
        }
    }

    pub fn cast(&mut self, cast: TType) {
        match self {
            Expr::Closure(_, env, params, body) => {
                // Rebuild expression with the new type
                *self = Expr::Closure(cast, env.clone(), params.clone(), body.clone());
            }
            Expr::ListConstructor(_, elements) => {
                // Rebuild expression with the new type
                *self = Expr::ListConstructor(cast, elements.clone());
            }
            Expr::Field(_, obj, field_name, index, in_expr) => {
                // Rebuild expression with the new type
                *self = Expr::Field(
                    cast,
                    obj.clone(),
                    field_name.clone(),
                    index.clone(),
                    in_expr.clone(),
                );
            }
            Expr::Indexed(_, container, index_expr, index, in_expr) => {
                // Rebuild expression with the new type
                *self = Expr::Indexed(
                    cast,
                    container.clone(),
                    index_expr.clone(),
                    index.clone(),
                    in_expr.clone(),
                );
            }
            Expr::Call(_, func, args, in_expr) => {
                // Rebuild expression with the new type
                *self = Expr::Call(cast, func.clone(), args.clone(), in_expr.clone());
            }
            Expr::Unary(_, op, operand) => {
                // Rebuild expression with the new type
                *self = Expr::Unary(cast, op.clone(), operand.clone());
            }
            Expr::Binop(_, op, lhs, rhs) => {
                // Rebuild expression with the new type
                *self = Expr::Binop(cast, op.clone(), lhs.clone(), rhs.clone());
            }
            Expr::Literal(_, a) => {
                // Rebuild expression with the new type
                *self = Expr::Literal(cast, a.clone());
            }
            Expr::None => {
                // No need to change anything for Expr::None
            }
        }
    }
}
