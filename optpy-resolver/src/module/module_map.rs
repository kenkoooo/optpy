use std::collections::BTreeMap;

pub(crate) struct ModuleMap {
    modules: BTreeMap<String, String>,
}

impl Default for ModuleMap {
    fn default() -> Self {
        Self {
            modules: BTreeMap::from([
                ("math.gcd".into(), "__math__gcd".into()),
                ("math.log".into(), "__math__log".into()),
                ("math.exp".into(), "__math__exp".into()),
                (
                    "sys.setrecursionlimit".into(),
                    "__sys__setrecursionlimit".into(),
                ),
                (
                    "collections.deque".into(),
                    "__collections__deque__macro__".into(),
                ),
            ]),
        }
    }
}

impl ModuleMap {
    pub(crate) fn find_children(&self, module: &str) -> Vec<(&str, &str)> {
        let mut result = vec![];
        for (key, value) in self.modules.iter() {
            if let Some(child) = key.strip_prefix(&format!("{module}.")) {
                if !child.contains(".") {
                    result.push((child, value.as_str()));
                }
            }
        }
        result
    }

    pub(crate) fn find_match(&self, module_function: &str) -> Option<&str> {
        self.modules.get(module_function).map(|s| s.as_str())
    }
}
