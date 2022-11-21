use optpy_parser::{
    Assign, BinaryOperation, BoolOperation, CallFunction, CallMethod, Compare, Dict, Expr, Func,
    If, Index, Statement, UnaryOperation, While,
};

use crate::types::{Edge, Type, Vertex};

pub fn read_types(statements: &[Statement]) -> Vec<Edge> {
    let mut edges = vec![];
    for statement in statements {
        read_statement(statement, &mut edges);
    }
    edges
}

fn read_statement(statement: &Statement, edges: &mut Vec<Edge>) -> Vec<Vertex> {
    match statement {
        Statement::Assign(Assign { target, value }) => {
            let target = read_expr(target, edges);
            let value = read_expr(value, edges);
            edges.push(Edge::equal(target, value));
            vec![]
        }
        Statement::Expression(expr) => {
            read_expr(expr, edges);
            vec![]
        }
        Statement::If(If { test, body, orelse }) => {
            let test = read_expr(test, edges);
            edges.push(Edge::equal(test, Vertex::Fixed(Type::Bool)));

            let mut function_return_value = vec![];
            for statement in body {
                function_return_value.extend(read_statement(statement, edges));
            }
            for statement in orelse {
                function_return_value.extend(read_statement(statement, edges));
            }
            function_return_value
        }
        Statement::Func(Func { name, args, body }) => {
            let mut function_return_value = vec![];
            for statement in body {
                function_return_value.extend(read_statement(statement, edges));
            }
            let args = args
                .iter()
                .map(|arg| Vertex::Variable(arg.into()))
                .collect();
            let return_type = Vertex::ReturnType {
                function: name.into(),
                args,
            };

            complete(&function_return_value, edges);
            for r in function_return_value {
                edges.push(Edge::equal(r, return_type.clone()));
            }

            vec![]
        }
        Statement::Return(r) => match r {
            Some(e) => {
                let e = read_expr(e, edges);
                vec![e]
            }
            None => vec![Vertex::Fixed(Type::None)],
        },
        Statement::While(While { test, body }) => {
            let test = read_expr(test, edges);
            edges.push(Edge::equal(test, Vertex::Fixed(Type::Bool)));

            let mut function_return_value = vec![];
            for statement in body {
                function_return_value.extend(read_statement(statement, edges));
            }
            function_return_value
        }
        Statement::Break => vec![],
        Statement::Continue => vec![],
    }
}

fn read_expr(expr: &Expr, edges: &mut Vec<Edge>) -> Vertex {
    match expr {
        Expr::CallFunction(CallFunction { name, args }) => {
            let args = args.iter().map(|arg| read_expr(arg, edges)).collect();
            Vertex::ReturnType {
                function: name.into(),
                args,
            }
        }
        Expr::CallMethod(CallMethod { value, name, args }) => {
            let value = read_expr(value, edges);
            let args = args.iter().map(|arg| read_expr(arg, edges)).collect();
            Vertex::MethodReturnType {
                value: Box::new(value),
                name: name.into(),
                args,
            }
        }
        Expr::Tuple(_) => todo!(),
        Expr::VariableName(name) => Vertex::Variable(name.into()),
        Expr::BoolOperation(BoolOperation { op: _, conditions }) => {
            for condition in conditions {
                let condition = read_expr(condition, edges);
                edges.push(Edge::equal(Vertex::Fixed(Type::Bool), condition))
            }
            Vertex::Fixed(Type::Bool)
        }
        Expr::Compare(Compare { left, right, op: _ }) => {
            let left = read_expr(left, edges);
            let right = read_expr(right, edges);
            edges.push(Edge::equal(left, right));
            Vertex::Fixed(Type::Bool)
        }
        Expr::UnaryOperation(UnaryOperation { value, op: _ }) => read_expr(value, edges),
        Expr::BinaryOperation(BinaryOperation { left, right, op }) => {
            let left = read_expr(left, edges);
            let right = read_expr(right, edges);
            use optpy_parser::BinaryOperator::*;
            match op {
                Add | Sub | Div | Mod | BitAnd => {
                    edges.push(Edge::equal(left.clone(), right.clone()));
                }
                FloorDiv => {
                    edges.push(Edge::equal(left.clone(), right.clone()));
                    edges.push(Edge::equal(left.clone(), Type::Number.into()))
                }
                Mul => {}
                Pow => {}
            }
            left
        }
        Expr::Index(Index { value, index }) => {
            let value = read_expr(value, edges);
            let index = read_expr(index, edges);
            Vertex::Index {
                value: Box::new(value),
                key: Box::new(index),
            }
        }
        Expr::ConstantNumber(_) => Vertex::Fixed(Type::Number),
        Expr::ConstantString(_) => Vertex::Fixed(Type::String),
        Expr::ConstantBoolean(_) => Vertex::Fixed(Type::Bool),
        Expr::List(elements) => {
            let mut list = vec![];
            for element in elements {
                list.push(read_expr(element, edges));
            }

            complete(&list, edges);

            if let Some(a) = list.pop() {
                Vertex::List(Box::new(a))
            } else {
                Vertex::List(Box::new(Vertex::Unknown))
            }
        }
        Expr::Dict(Dict { pairs }) => {
            let mut keys = vec![];
            let mut values = vec![];
            for (key, value) in pairs {
                let key = read_expr(key, edges);
                let value = read_expr(value, edges);
                keys.push(key);
                values.push(value);
            }

            complete(&keys, edges);
            complete(&values, edges);

            if let (Some(key), Some(value)) = (keys.pop(), values.pop()) {
                Vertex::Map {
                    key: Box::new(key),
                    value: Box::new(value),
                }
            } else {
                Vertex::Map {
                    key: Box::new(Vertex::Unknown),
                    value: Box::new(Vertex::Unknown),
                }
            }
        }
        Expr::None => Vertex::Fixed(Type::None),
    }
}

fn complete(list: &[Vertex], edges: &mut Vec<Edge>) {
    let n = list.len();
    for i in 0..n {
        for j in 0..i {
            edges.push(Edge::equal(list[i].clone(), list[j].clone()));
        }
    }
}
