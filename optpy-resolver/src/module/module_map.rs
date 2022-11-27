use std::collections::BTreeMap;

pub(crate) struct ModuleMap {
    modules: BTreeMap<String, String>,
}

impl Default for ModuleMap {
    fn default() -> Self {
        Self {
            modules: BTreeMap::from([("math.gcd".into(), "__math__gcd".into())]),
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
