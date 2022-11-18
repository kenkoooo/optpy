use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub(super) struct FunctionTree {
    parents: BTreeMap<String, String>,
}

impl FunctionTree {
    pub(super) fn add_edge(&mut self, parent: &str, child: &str) {
        self.parents.insert(child.to_string(), parent.to_string());
    }

    pub(super) fn path(&self, node: &str) -> Vec<String> {
        let mut cur = node;
        let mut path = vec![cur.to_string()];
        while let Some(parent) = self.parents.get(cur) {
            path.push(parent.to_string());
            cur = parent.as_str();
        }
        path
    }
}
