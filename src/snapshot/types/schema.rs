use std::collections::HashMap;

use crate::snapshot::error::SnapshotError;

use super::{Function, Relation};

#[derive(PartialEq, Debug)]
pub struct Schema {
    pub name: String,
    pub relations: HashMap<String, Relation>,
    pub functions: HashMap<String, Function>,
}

impl Schema {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            relations: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    // relation
    pub fn get_relation(&self, relation: &str) -> Result<&Relation, SnapshotError> {
        self.relations
            .get(relation)
            .ok_or(SnapshotError::relation_not_found(&self.name, relation))
    }
    pub fn iter_relations(&self) -> impl Iterator<Item = &Relation> {
        self.relations.values()
    }
    pub fn get_relation_mut(&mut self, rel_name: &str) -> Result<&mut Relation, SnapshotError> {
        self.relations
            .get_mut(rel_name)
            .ok_or(SnapshotError::relation_not_found(&self.name, rel_name))
    }
    pub fn has_relation(&self, rel_name: &str) -> bool {
        self.relations.contains_key(rel_name)
    }
    pub fn add_relation(&mut self, rel: Relation) -> Result<(), SnapshotError> {
        let rel_name = rel.get_name();
        if self.relations.contains_key(rel_name) {
            return Err(SnapshotError::relation_already_exists(&self.name, rel_name));
        }
        self.relations.insert(rel_name.to_owned(), rel);
        return Ok(());
    }
    pub fn remove_relation(&mut self, rel_name: &str) -> Result<Relation, SnapshotError> {
        self.relations
            .remove(rel_name)
            .ok_or(SnapshotError::relation_not_found(&self.name, rel_name))
    }
    // function
    pub fn get_function(&self, function: &str) -> Result<&Function, SnapshotError> {
        self.functions
            .get(function)
            .ok_or(SnapshotError::function_not_found(&self.name, function))
    }
    pub fn iter_functions(&self) -> impl Iterator<Item = &Function> {
        self.functions.values()
    }
    pub fn get_function_mut(&mut self, function: &str) -> Result<&mut Function, SnapshotError> {
        self.functions
            .get_mut(function)
            .ok_or(SnapshotError::function_not_found(&self.name, function))
    }
    pub fn has_function(&self, function: &str) -> bool {
        self.functions.contains_key(function)
    }
    pub fn add_function(&mut self, function: Function) -> Result<(), SnapshotError> {
        let function_name = function.get_name();
        if self.functions.contains_key(function_name) {
            return Err(SnapshotError::function_already_exists(
                &self.name,
                function_name,
            ));
        }
        self.functions.insert(function_name.to_owned(), function);
        return Ok(());
    }
    pub fn remove_function(&mut self, function: &str) -> Result<Function, SnapshotError> {
        self.functions
            .remove(function)
            .ok_or(SnapshotError::function_not_found(&self.name, function))
    }

    pub fn merge_schema(&mut self, source: Schema) -> Result<(), SnapshotError> {
        for (_, v) in source.relations {
            self.add_relation(v)?;
        }
        for (_, v) in source.functions {
            self.add_function(v)?;
        }
        Ok(())
    }
}
