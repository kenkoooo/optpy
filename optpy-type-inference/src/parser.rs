use std::collections::BTreeMap;

use optpy_parser::{
    Assign, BinaryOperation, CallFunction, CallMethod, Dict, Expr, Func, If, Index,
    Statement as Stmt, While,
};

pub(super) struct Parser {
    level: u64,
    map: BTreeMap<String, Variable>,
    env: BTreeMap<Variable, Type>,
    tmp_counter: u64,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            level: 0,
            map: Default::default(),
            env: Default::default(),
            tmp_counter: 0,
        }
    }
}

impl Parser {
    pub(super) fn add_env(&mut self, name: &str, t: Type) {
        let var = self.variable(name);
        self.env.insert(var, t);
    }
    pub(super) fn parse_statements(&mut self, statements: &[Stmt]) -> Vec<Statement> {
        statements
            .iter()
            .flat_map(|s| self.parse_statement(s))
            .collect()
    }

    fn variable(&mut self, name: &str) -> Variable {
        let v = self
            .map
            .entry(name.to_string())
            .or_insert_with(|| Variable {
                name: name.to_string(),
                level: self.level,
            });
        v.clone()
    }
    fn tmp_variable(&mut self) -> Variable {
        let name = format!("__tmp_{}", self.tmp_counter);
        self.tmp_counter += 1;
        self.variable(&name)
    }

    fn parse_statement(&mut self, statement: &Stmt) -> Vec<Statement> {
        match statement {
            Stmt::Assign(Assign { target, value }) => {
                let mut result = vec![];
                let value = self.parse_expression(value, &mut result);
                let target = self.parse_expression(target, &mut result);

                let tmp = self.tmp_variable();
                result.push(Statement::Let(tmp.clone(), value));
                result.push(Statement::Let(tmp, target));
                result
            }
            Stmt::Func(Func { name, args, body }) => {
                let f = self.variable(name);
                self.level += 1;
                let args = args
                    .iter()
                    .map(|arg| self.variable(arg))
                    .collect::<Vec<_>>();
                let body = self.parse_statements(body);

                eprintln!("name={}", name);
                let result = self.resolve(&args, &body);
                self.level -= 1;
                vec![Statement::Let(f, result)]
            }
            Stmt::If(If { test, body, orelse }) => {
                let mut result = vec![];
                let test = self.parse_expression(test, &mut result);

                let tmp = self.tmp_variable();
                result.push(Statement::Let(tmp.clone(), test));
                result.push(Statement::Let(tmp.clone(), Type::Bool));
                result.extend(self.parse_statements(body));
                result.extend(self.parse_statements(orelse));
                result
            }
            Stmt::Return(r) => match r {
                Some(r) => {
                    let mut result = vec![];
                    let r = self.parse_expression(r, &mut result);
                    result.push(Statement::Return(r));
                    result
                }
                None => vec![Statement::Return(Type::None)],
            },
            Stmt::While(While { test, body }) => {
                let mut result = vec![];
                let test = self.parse_expression(test, &mut result);
                result.extend(self.parse_statements(body));

                let tmp = self.tmp_variable();
                result.push(Statement::Let(tmp.clone(), test));
                result.push(Statement::Let(tmp.clone(), Type::Bool));
                result
            }
            Stmt::Break
            | Stmt::Continue
            | Stmt::Import(_)
            | Stmt::Expression(_)
            | Stmt::FromImport(_) => vec![],
        }
    }

    fn parse_expression(&mut self, expr: &Expr, statements: &mut Vec<Statement>) -> Type {
        match expr {
            Expr::CallFunction(CallFunction { name, args }) => {
                let f = self.variable(name);
                let args = args
                    .iter()
                    .map(|arg| self.parse_expression(arg, statements))
                    .collect::<Vec<_>>();
                Type::Apply(f, args)
            }
            Expr::CallMethod(CallMethod { value, name, args }) => {
                let name = format!("__method__{}", name);
                let mut args = args
                    .iter()
                    .map(|arg| self.parse_expression(arg, statements))
                    .collect::<Vec<_>>();
                let f = self.variable(&name);
                args.insert(0, self.parse_expression(value, statements));
                Type::Apply(f, args)
            }
            Expr::Tuple(tuple) => {
                if tuple.is_empty() {
                    let tmp = self.tmp_variable();
                    Type::Fun(vec![Type::Int], Box::new(Type::Typing(tmp)))
                } else {
                    let v = self.parse_expression(&tuple[0], statements);
                    Type::Fun(vec![Type::Int], Box::new(v))
                }
            }
            Expr::VariableName(name) => {
                let v = self.variable(name);
                Type::Typing(v)
            }
            Expr::Compare(_) | Expr::BoolOperation(_) => Type::Bool,
            Expr::UnaryOperation(_) => todo!(),
            Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
                let left = self.parse_expression(left, statements);
                let right = self.parse_expression(right, statements);
                match op {
                    optpy_parser::BinaryOperator::Mul => {
                        let f = self.variable("__mul");
                        Type::Apply(f, vec![left, right])
                    }
                    _ => left,
                }
            }
            Expr::Index(Index { value, index }) => {
                let value = self.parse_expression(value, statements);
                let index = self.parse_expression(index, statements);

                let f = self.tmp_variable();
                statements.push(Statement::Let(f.clone(), value));
                Type::Apply(f, vec![index])
            }
            Expr::ConstantNumber(n) => match n {
                optpy_parser::Number::Int(_) => Type::Int,
                optpy_parser::Number::Float(_) => Type::Float,
            },
            Expr::ConstantString(_) => Type::String,
            Expr::ConstantBoolean(_) => Type::Bool,
            Expr::None => Type::None,
            Expr::List(list) => {
                if list.is_empty() {
                    let tmp = self.tmp_variable();
                    Type::Fun(vec![Type::Int], Box::new(Type::Typing(tmp)))
                } else {
                    let v = self.parse_expression(&list[0], statements);
                    Type::Fun(vec![Type::Int], Box::new(v))
                }
            }
            Expr::Dict(Dict { pairs }) => {
                if pairs.is_empty() {
                    let tmp_key = self.tmp_variable();
                    let tmp_value = self.tmp_variable();
                    Type::Fun(
                        vec![Type::Typing(tmp_key)],
                        Box::new(Type::Typing(tmp_value)),
                    )
                } else {
                    let key = self.parse_expression(&pairs[0].0, statements);
                    let value = self.parse_expression(&pairs[0].1, statements);
                    Type::Fun(vec![key], Box::new(value))
                }
            }
        }
    }

    fn resolve(&mut self, args: &[Variable], statements: &[Statement]) -> Type {
        let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
        println!("args={}", args.join(", "));
        for s in statements {
            println!("{}", s.to_string());
        }
        todo!()
    }
}

pub(super) enum Statement {
    Let(Variable, Type),
    Return(Type),
}
impl ToString for Statement {
    fn to_string(&self) -> String {
        match self {
            Statement::Let(a, b) => format!("let {} = {}", a.to_string(), b.to_string()),
            Statement::Return(r) => format!("return {}", r.to_string()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Variable {
    name: String,
    level: u64,
}
impl ToString for Variable {
    fn to_string(&self) -> String {
        format!("?{}({})", self.name, self.level)
    }
}

#[derive(Clone)]
pub(super) enum Type {
    String,
    Int,
    Float,
    Bool,
    None,
    Fun(Vec<Type>, Box<Type>),
    Apply(Variable, Vec<Type>),
    Typing(Variable),
    TypeParam(usize),
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::String => "String".into(),
            Type::Int => "Int".into(),
            Type::Float => "Float".into(),
            Type::Bool => "Bool".into(),
            Type::Fun(args, value) => {
                let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
                format!("({}) -> {}", args.join(", "), value.to_string())
            }
            Type::Typing(v) => v.to_string(),
            Type::TypeParam(name) => format!("'{}", name),
            Type::None => "None".to_string(),
            Type::Apply(f, args) => {
                let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<_>>();
                format!("{}({})", f.to_string(), args.join(", "))
            }
        }
    }
}
