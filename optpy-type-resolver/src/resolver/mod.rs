use std::collections::{BTreeMap, BTreeSet};

use crate::types::{Edge, Type, Vertex};

use self::unionfind::UnionFind;

mod unionfind;

macro_rules! t {
    ($x:ident) => {
        Type::$x
    };
    (List<$x:ident>) => {
        Type::List(Box::new(t!($x)))
    };
}

pub fn try_resolve(edges: &[Edge]) {
    let mut edges = edges.to_vec();

    // add hints
    edges.push(Edge::equal(
        t!(String).into(),
        Vertex::ReturnType {
            function: "input".into(),
            args: vec![],
        },
    ));
    edges.push(Edge::equal(
        t!(List<String>).into(),
        Vertex::MethodReturnType {
            value: Box::new(Type::String.into()),
            name: "split".into(),
            args: vec![],
        },
    ));
    edges.push(Edge::equal(
        t!(Number).into(),
        Vertex::MethodReturnType {
            value: Box::new((t!(List<Number>)).into()),
            name: "pop".into(),
            args: vec![],
        },
    ));
    edges.push(Edge::equal(
        t!(List<Number>).into(),
        Vertex::ReturnType {
            function: "map_int".into(),
            args: vec![t!(List<String>).into()],
        },
    ));
    edges.push(Edge::equal(
        t!(List<Number>).into(),
        Vertex::ReturnType {
            function: "range__macro__".into(),
            args: vec![t!(Number).into()],
        },
    ));
    edges.push(Edge::equal(
        t!(List<Number>).into(),
        Vertex::ReturnType {
            function: "list".into(),
            args: vec![t!(List<Number>).into()],
        },
    ));

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
