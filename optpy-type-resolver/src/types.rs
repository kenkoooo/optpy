#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum Type {
    String,
    Number,
    Bool,
    None,
    List(Box<Type>),
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::String => "String".to_string(),
            Type::Number => "Number".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::None => "None".to_string(),
            Type::List(t) => format!("List<{}>", t.to_string()),
        }
    }
}
impl Into<Vertex> for Type {
    fn into(self) -> Vertex {
        Vertex::Fixed(self)
    }
}

#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub(crate) enum Vertex {
    Variable(String),
    ReturnType {
        function: String,
        args: Vec<Vertex>,
    },
    Fixed(Type),
    Index {
        value: Box<Vertex>,
        key: Box<Vertex>,
    },
    MethodReturnType {
        value: Box<Vertex>,
        name: String,
        args: Vec<Vertex>,
    },
    List(Box<Vertex>),
    Map {
        key: Box<Vertex>,
        value: Box<Vertex>,
    },
    Unknown,
}

impl Vertex {
    pub(crate) fn has_unknown(&self) -> bool {
        match self {
            Vertex::Variable(_) => false,
            Vertex::ReturnType { function: _, args } => args.iter().any(|arg| arg.has_unknown()),
            Vertex::Fixed(_) => false,
            Vertex::Index { value, key } => value.has_unknown() || key.has_unknown(),
            Vertex::MethodReturnType {
                value,
                name: _,
                args,
            } => value.has_unknown() || args.iter().any(|arg| arg.has_unknown()),
            Vertex::List(v) => v.has_unknown(),
            Vertex::Map { key, value } => key.has_unknown() || value.has_unknown(),
            Vertex::Unknown => true,
        }
    }
}

impl ToString for Vertex {
    fn to_string(&self) -> String {
        match self {
            Self::Variable(name) => format!("{}", name),
            Self::ReturnType { function, args } => {
                let args = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{function}({args})")
            }
            Self::Fixed(t) => format!("Type({})", t.to_string()),
            Self::Index { value, key } => format!("{}[{}]", value.to_string(), key.to_string()),
            Self::MethodReturnType { value, name, args } => {
                let args = args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}.{name}({args})", value.to_string())
            }
            Self::List(arg0) => format!("List<{}>", arg0.to_string()),
            Self::Map { key, value } => format!("Map<{}, {}>", value.to_string(), key.to_string()),
            Self::Unknown => format!("Unknown"),
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Edge {
    pub(crate) v: Vertex,
    pub(crate) u: Vertex,
}

impl ToString for Edge {
    fn to_string(&self) -> String {
        format!("{} == {}", self.v.to_string(), self.u.to_string())
    }
}

impl Edge {
    pub(crate) fn equal(v: Vertex, u: Vertex) -> Self {
        let (u, v) = if v > u { (v, u) } else { (u, v) };
        Self { v, u }
    }
}
