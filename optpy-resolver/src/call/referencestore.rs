use std::collections::{BTreeMap, BTreeSet};

#[derive(Default, Debug)]
pub(super) struct ReferenceStore {
    variable_functions: BTreeMap<String, BTreeSet<String>>,
    function_variables: BTreeMap<String, BTreeSet<String>>,
}

impl ReferenceStore {
    pub(super) fn record(&mut self, variable_name: &str, function_name: &str) {
        self.variable_functions
            .entry(variable_name.to_string())
            .or_default()
            .insert(function_name.to_string());
        self.function_variables
            .entry(function_name.to_string())
            .or_default()
            .insert(variable_name.to_string());
    }

    pub(super) fn list_variables(&self) -> Vec<String> {
        self.variable_functions
            .keys()
            .map(|key| key.to_string())
            .collect()
    }

    pub(super) fn list_by_function(&self, function_name: &str) -> BTreeSet<String> {
        self.function_variables
            .get(function_name)
            .cloned()
            .unwrap_or_default()
    }

    pub(super) fn list_by_variable(&self, variable_name: &str) -> BTreeSet<String> {
        self.variable_functions
            .get(variable_name)
            .cloned()
            .unwrap_or_default()
    }

    pub(super) fn remove_function(&mut self, function_name: &str) {
        let variables = self
            .function_variables
            .remove(function_name)
            .unwrap_or_default();
        for variable in variables {
            assert!(self
                .variable_functions
                .get_mut(&variable)
                .expect("invalid")
                .remove(function_name));
        }
    }
}
