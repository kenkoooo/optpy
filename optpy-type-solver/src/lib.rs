use std::collections::{BTreeMap, BTreeSet};

use optpy_parser::{
    Assign, BinaryOperation, BinaryOperator, BoolOperation, CallFunction, CallMethod, Compare,
    Dict, Expr, Func, If, Index, Number, Statement, UnaryOperation, While,
};

pub fn resolve_types(statements: &[Statement]) {
    let mut resolver = Resolver::default();
    for statement in statements {
        resolver.load_statement(statement);
    }

    let mut definitions = resolver.definitions;
    let mut fixed = BTreeMap::new();
    loop {
        let mut new_definitions = BTreeMap::new();
        for (name, values) in definitions.iter() {
            for value in values {
                let value = value.resolve(&fixed);
                if value.fixed() {
                    fixed.insert(name.clone(), value);
                } else {
                    new_definitions
                        .entry(name.clone())
                        .or_insert_with(BTreeSet::new)
                        .insert(value);
                }
            }
        }

        if definitions == new_definitions {
            break;
        }

        definitions = new_definitions;
    }

    for (name, values) in definitions {
        eprintln!("{}", name.0);
        for value in values {
            eprintln!("= {:?}", value);
        }
        eprintln!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    Param(usize),
    Typing(Var),
    Number,
    String,
    Bool,
    None,
    Fun(Vec<Type>, Box<Type>),
    Map(Box<Type>, Box<Type>),
    Apply(Var, Vec<Type>),
}

impl Type {
    fn map(key: Type, value: Type) -> Self {
        Self::Map(Box::new(key), Box::new(value))
    }
    fn list(t: Type) -> Self {
        Self::Map(Box::new(Type::Number), Box::new(t))
    }
    fn fun(args: Vec<Type>, result: Type) -> Self {
        Self::Fun(args, result.into())
    }

    fn fixed(&self) -> bool {
        match self {
            Type::Param(_) | Type::Number | Type::String | Type::Bool | Type::None => true,
            Type::Fun(args, result) => args.iter().all(|arg| arg.fixed()) && result.fixed(),
            Type::Map(k, v) => k.fixed() && v.fixed(),
            Type::Apply(_, _) | Type::Typing(_) => false,
        }
    }
    fn resolve(&self, fixed: &BTreeMap<Var, Type>) -> Type {
        match self {
            Type::Typing(var) => match fixed.get(var) {
                Some(fixed) => fixed.clone(),
                None => self.clone(),
            },
            Type::Param(_) | Type::Number | Type::String | Type::Bool | Type::None => self.clone(),
            Type::Fun(args, result) => {
                let args = args
                    .iter()
                    .map(|arg| arg.resolve(fixed))
                    .collect::<Vec<_>>();
                let result = result.resolve(fixed);
                Type::Fun(args, Box::new(result))
            }
            Type::Map(k, v) => Type::Map(Box::new(k.resolve(fixed)), Box::new(v.resolve(fixed))),
            Type::Apply(f, args) => {
                let args = args
                    .iter()
                    .map(|arg| arg.resolve(fixed))
                    .collect::<Vec<_>>();
                match fixed.get(f) {
                    Some(Type::Fun(params, result)) => {
                        assert_eq!(args.len(), params.len());
                        let mut param_map = BTreeMap::new();
                        for (arg, param) in args.iter().zip(params) {
                            map_params(arg, param, &mut param_map);
                        }
                        resolve_param(result, &param_map)
                    }
                    _ => Type::Apply(f.clone(), args),
                }
            }
        }
    }
}

fn resolve_param(result: &Type, map: &BTreeMap<usize, Type>) -> Type {
    match result {
        Type::Param(p) => match map.get(p) {
            Some(t) => t.clone(),
            None => result.clone(),
        },
        Type::Typing(_) => unreachable!("param is var"),
        Type::Apply(_, _) => unreachable!("param is apply"),
        Type::Number | Type::String | Type::Bool | Type::None => result.clone(),
        Type::Map(k, v) => {
            let k = resolve_param(k, map);
            let v = resolve_param(v, map);
            Type::map(k, v)
        }
        Type::Fun(_, _) => todo!(),
    }
}

fn map_params(arg: &Type, param: &Type, map: &mut BTreeMap<usize, Type>) {
    match (arg, param) {
        (Type::Param(_), _) => unreachable!("arg is param"),
        (_, Type::Typing(_)) => unreachable!("param is var"),
        (_, Type::Apply(_, _)) => unreachable!("param is apply"),
        (arg, Type::Param(p)) => {
            map.insert(*p, arg.clone());
        }
        (Type::Fun(a1, b1), Type::Fun(a2, b2)) => {
            assert_eq!(a1.len(), a2.len());
            for (a1, a2) in a1.iter().zip(a2) {
                map_params(a1, a2, map);
            }
            map_params(b1, b2, map);
        }
        (Type::Map(a1, b1), Type::Map(a2, b2)) => {
            map_params(a1, a2, map);
            map_params(b1, b2, map);
        }
        _ => {}
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Var(String);

struct Resolver {
    tmp_counter: usize,
    definitions: BTreeMap<Var, BTreeSet<Type>>,
}

impl Default for Resolver {
    fn default() -> Self {
        let mut definitions = BTreeMap::new();
        definitions
            .entry(Var("iter_1".into()))
            .or_insert_with(BTreeSet::new)
            .insert(Type::fun(vec![Type::Param(0)], Type::Param(0)));
        definitions
            .entry(Var("range__macro___1".into()))
            .or_insert_with(BTreeSet::new)
            .insert(Type::fun(
                vec![Type::Number],
                Type::map(Type::Number, Type::Number),
            ));
        Self {
            tmp_counter: 0,
            definitions,
        }
    }
}

impl Resolver {
    fn load_statement(&mut self, statement: &Statement) -> Vec<Type> {
        match statement {
            Statement::Assign(Assign { target, value }) => {
                let rhs = self.parse_expr(value);
                self.assign(target, rhs);
                vec![]
            }
            Statement::Expression(e) => {
                self.parse_expr(e);
                vec![]
            }
            Statement::If(If { test, body, orelse }) => {
                let tmp = self.new_tmp_var();
                self.define(tmp.clone(), Type::Bool);

                let test = self.parse_expr(test);
                self.define(tmp.clone(), test);

                let mut results = vec![];
                for statement in body {
                    results.extend(self.load_statement(statement));
                }
                for statement in orelse {
                    results.extend(self.load_statement(statement));
                }
                results
            }
            Statement::Func(Func { name, args, body }) => {
                let name = Var(format!("{}_{}", name, args.len()));
                let args = args
                    .iter()
                    .map(|arg| Type::Typing(Var(arg.to_string())))
                    .collect::<Vec<_>>();
                let mut results = vec![];
                for statement in body {
                    results.extend(self.load_statement(statement));
                }
                for result in results {
                    self.define(name.clone(), Type::Fun(args.clone(), Box::new(result)));
                }

                vec![]
            }
            Statement::While(While { test, body }) => {
                let tmp = self.new_tmp_var();
                let test = self.parse_expr(test);
                self.define(tmp.clone(), test);
                self.define(tmp.clone(), Type::Bool);
                let mut results = vec![];
                for statement in body {
                    results.extend(self.load_statement(statement));
                }
                results
            }
            Statement::Break
            | Statement::Continue
            | Statement::Import(_)
            | Statement::FromImport(_) => vec![],
            Statement::Return(r) => match r {
                Some(r) => vec![self.parse_expr(r)],
                None => vec![Type::None],
            },
        }
    }
    fn assign(&mut self, lhs: &Expr, rhs: Type) {
        match lhs {
            Expr::Index(Index { value, index }) => {
                let key = self.parse_expr(index);
                self.assign(value, Type::map(key, rhs))
            }
            Expr::VariableName(name) => self.define(Var(name.into()), rhs),
            lhs => {
                let tmp = self.new_tmp_var();
                let lhs = self.parse_expr(lhs);
                self.define(tmp.clone(), lhs);
                self.define(tmp.clone(), rhs);
            }
        }
    }
    fn parse_expr(&mut self, expr: &Expr) -> Type {
        match expr {
            Expr::CallFunction(CallFunction { name, args }) => {
                let args = args
                    .iter()
                    .map(|arg| self.parse_expr(arg))
                    .collect::<Vec<_>>();
                let name = format!("{}_{}", name, args.len());
                Type::Apply(Var(name), args)
            }
            Expr::CallMethod(CallMethod { value, name, args }) => {
                let mut method_args = vec![self.parse_expr(value)];
                for arg in args {
                    let arg = self.parse_expr(arg);
                    method_args.push(arg);
                }
                Type::Apply(
                    Var(format!("__method__{}_{}", name, method_args.len())),
                    method_args,
                )
            }
            Expr::Tuple(list) | Expr::List(list) => {
                let element_type = self.new_tmp_var();
                for e in list {
                    let e = self.parse_expr(e);
                    self.define(element_type.clone(), e);
                }
                Type::list(Type::Typing(element_type))
            }
            Expr::VariableName(name) => Type::Typing(Var(name.to_string())),
            Expr::BoolOperation(BoolOperation { op: _, conditions }) => {
                let tmp = self.new_tmp_var();
                for condition in conditions {
                    let condition = self.parse_expr(condition);
                    self.define(tmp.clone(), condition);
                }
                self.define(tmp, Type::Bool);
                Type::Bool
            }
            Expr::Compare(Compare { left, right, op: _ }) => {
                let tmp = self.new_tmp_var();
                let left = self.parse_expr(left);
                let right = self.parse_expr(right);
                self.define(tmp.clone(), left);
                self.define(tmp.clone(), right);
                Type::Bool
            }
            Expr::UnaryOperation(UnaryOperation { value, op: _ }) => self.parse_expr(value),
            Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
                let left = self.parse_expr(left);
                let right = self.parse_expr(right);
                match op {
                    BinaryOperator::Mul => {
                        let tmp = self.new_tmp_var();
                        self.define(tmp.clone(), right);
                        self.define(tmp.clone(), Type::Number);
                        left
                    }
                    _ => {
                        let tmp = self.new_tmp_var();
                        self.define(tmp.clone(), left);
                        self.define(tmp.clone(), right);
                        Type::Typing(tmp)
                    }
                }
            }
            Expr::Index(Index { value, index }) => {
                let element_type = Type::Typing(self.new_tmp_var());
                let tmp_target = self.new_tmp_var();
                let value = self.parse_expr(value);
                let key = self.parse_expr(index);
                self.define(tmp_target.clone(), value);
                self.define(
                    tmp_target.clone(),
                    Type::Map(Box::new(key), Box::new(element_type)),
                );
                Type::Typing(tmp_target)
            }
            Expr::ConstantNumber(_) => Type::Number,
            Expr::ConstantString(_) => Type::String,
            Expr::ConstantBoolean(_) => Type::Bool,
            Expr::None => Type::None,
            Expr::Dict(Dict { pairs }) => {
                let key_type = self.new_tmp_var();
                let value_type = self.new_tmp_var();
                for (k, v) in pairs {
                    let k = self.parse_expr(k);
                    self.define(key_type.clone(), k);
                    let v = self.parse_expr(v);
                    self.define(value_type.clone(), v);
                }
                Type::map(Type::Typing(key_type), Type::Typing(value_type))
            }
        }
    }

    fn define(&mut self, var: Var, t: Type) {
        self.definitions.entry(var).or_default().insert(t);
    }

    fn new_tmp_var(&mut self) -> Var {
        let name = format!("__t{}", self.tmp_counter);
        self.tmp_counter += 1;
        Var(name)
    }
}
