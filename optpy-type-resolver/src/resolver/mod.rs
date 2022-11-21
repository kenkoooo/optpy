use std::collections::{BTreeMap, BTreeSet};

use crate::types::{Edge, Type, Vertex};

use self::unionfind::UnionFind;

mod unionfind;

fn hints() -> Vec<Edge> {
    use Type::*;
    fn list(ty: Type) -> Type {
        Type::List(Box::new(ty))
    }
    fn edge<T: Into<Vertex>, U: Into<Vertex>>(t: T, u: U) -> Edge {
        Edge::equal(t.into(), u.into())
    }
    fn func<S: AsRef<str>>(name: S, args: &[Type]) -> Vertex {
        Vertex::ReturnType {
            function: name.as_ref().into(),
            args: args.iter().map(|arg| arg.clone().into()).collect(),
        }
    }
    fn method<T: Into<Vertex>, S: AsRef<str>>(value: T, name: S, args: &[Type]) -> Vertex {
        Vertex::MethodReturnType {
            value: Box::new(value.into()),
            name: name.as_ref().into(),
            args: args.iter().map(|arg| arg.clone().into()).collect(),
        }
    }

    vec![
        edge(func("input", &[]), String),
        edge(func("map_int", &[list(String)]), list(Number)),
        edge(func("range__macro__", &[Number]), list(Number)),
        edge(func("list", &[list(Number)]), list(Number)),
        edge(method(list(Number), "pop", &[]), Number),
        edge(method(String, "split", &[]), list(String)),
    ]
}

pub fn try_resolve(edges: &[Edge]) {
    let mut edges = edges.to_vec();

    // add hints
    edges.extend(hints());

    let mut fixed = BTreeMap::new();
    let mut uf = UnionFind::new();

    for _ in 0..20 {
        for edge in edges.iter() {
            if edge.u.has_unknown() || edge.v.has_unknown() {
                continue;
            }
            uf.unite(&edge.u, &edge.v);
        }

        let mut groups = BTreeMap::new();
        for edge in edges.iter() {
            if edge.u.has_unknown() || edge.v.has_unknown() {
                continue;
            }

            let pu = uf.find(&edge.u);
            let pv = uf.find(&edge.v);
            groups
                .entry(pu)
                .or_insert_with(BTreeSet::new)
                .insert(edge.u.clone());
            groups
                .entry(pv)
                .or_insert_with(BTreeSet::new)
                .insert(edge.v.clone());
        }

        let types = [
            Type::String,
            Type::Number,
            Type::Bool,
            Type::None,
            Type::List(Box::new(Type::String)),
            Type::List(Box::new(Type::Number)),
        ];
        for t in types {
            let parent = uf.find(&Vertex::Fixed(t.clone()));
            let group = match groups.get(&parent) {
                Some(g) => g,
                None => continue,
            };
            for v in group {
                if fixed.insert(v.clone(), t.clone()).is_some() {
                    continue;
                }

                edges = replace(edges, v, &Vertex::Fixed(t.clone()));
            }
        }

        edges.sort();
        edges.dedup();
    }
    for (v, t) in fixed {
        eprintln!("{} {}", v.to_string(), t.to_string());
    }
    for e in edges {
        eprintln!("{}", e.to_string());
    }
}

fn replace(edges: Vec<Edge>, from: &Vertex, to: &Vertex) -> Vec<Edge> {
    edges
        .into_iter()
        .filter_map(|e| {
            let u = replace_vertex(e.u, &from, &to);
            let v = replace_vertex(e.v, &from, &to);
            if u == v {
                None
            } else {
                Some(Edge::equal(u, v))
            }
        })
        .collect()
}

fn replace_vertex(target: Vertex, from: &Vertex, to: &Vertex) -> Vertex {
    if &target == from {
        return to.clone();
    }
    match target {
        Vertex::Variable(_) | Vertex::Fixed(_) | Vertex::Unknown => target,
        Vertex::ReturnType { function, args } => {
            let args = args
                .into_iter()
                .map(|arg| replace_vertex(arg, from, to))
                .collect();
            Vertex::ReturnType { function, args }
        }
        Vertex::Index { value, key } => {
            let value = Box::new(replace_vertex(*value, from, to));
            let key = Box::new(replace_vertex(*key, from, to));
            Vertex::Index { value, key }
        }
        Vertex::MethodReturnType { value, name, args } => {
            let value = Box::new(replace_vertex(*value, from, to));
            let args = args
                .into_iter()
                .map(|arg| replace_vertex(arg, from, to))
                .collect();
            Vertex::MethodReturnType { value, name, args }
        }
        Vertex::List(list) => {
            let list = Box::new(replace_vertex(*list, from, to));
            Vertex::List(list)
        }
        Vertex::Map { key, value } => {
            let value = Box::new(replace_vertex(*value, from, to));
            let key = Box::new(replace_vertex(*key, from, to));
            Vertex::Map { key, value }
        }
    }
}
