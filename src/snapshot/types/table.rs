use std::collections::HashMap;

use crate::snapshot::error::SnapshotError;

use super::{Column, Constraint};

#[derive(PartialEq, Debug)]
pub struct Table {
    pub schema_name: String,
    pub name: String,

    pub columns: Vec<Column>,
    pub constraints: HashMap<String, Constraint>,
}

impl Table {
    pub fn new(schema_name: &str, name: &str) -> Self {
        Self {
            schema_name: schema_name.to_string(),
            name: name.to_string(),
            columns: Vec::new(),
            constraints: HashMap::new(),
        }
    }

    // column
    pub fn iter_columns(&self) -> impl Iterator<Item = &Column> {
        self.columns.iter()
    }
    pub fn get_column(&self, column: &str) -> Result<&Column, SnapshotError> {
        self.columns
            .iter()
            .find(|c| c.get_name() == column)
            .ok_or(SnapshotError::column_not_found(
                &self.schema_name,
                &self.name,
                column,
            ))
    }
    pub fn get_column_mut(&mut self, column: &str) -> Result<&mut Column, SnapshotError> {
        self.columns
            .iter_mut()
            .find(|c| c.get_name() == column)
            .ok_or(SnapshotError::column_not_found(
                &self.schema_name,
                &self.name,
                column,
            ))
    }
    pub fn has_column(&self, column: &str) -> bool {
        self.columns.iter().any(|c| c.get_name() == column)
    }
    pub fn add_column(&mut self, column: Column) -> Result<(), SnapshotError> {
        let column_name = column.get_name().to_owned();
        if self.columns.iter().any(|c| c.get_name() == column_name) {
            return Err(SnapshotError::column_already_exists(
                &self.schema_name,
                &self.name,
                &column_name,
            ));
        }
        self.columns.push(column);
        return Ok(());
    }
    pub fn remove_column(&mut self, column: &str) -> Result<Column, SnapshotError> {
        let index = self
            .columns
            .iter()
            .position(|c| c.get_name() == column)
            .ok_or(SnapshotError::column_not_found(
                &self.schema_name,
                &self.name,
                column,
            ))?;
        Ok(self.columns.remove(index))
    }

    // constraint
    pub fn iter_constraints(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.values()
    }
    pub fn get_constraint(&self, constraint: &str) -> Result<&Constraint, SnapshotError> {
        self.constraints
            .get(constraint)
            .ok_or(SnapshotError::constraint_not_found(
                &self.schema_name,
                &self.name,
                constraint,
            ))
    }
    pub fn get_constraint_mut(
        &mut self,
        constraint: &str,
    ) -> Result<&mut Constraint, SnapshotError> {
        self.constraints
            .get_mut(constraint)
            .ok_or(SnapshotError::constraint_not_found(
                &self.schema_name,
                &self.name,
                constraint,
            ))
    }
    pub fn has_constraint(&self, constraint: &str) -> bool {
        self.constraints.contains_key(constraint)
    }
    pub fn add_constraint(&mut self, constraint: Constraint) -> Result<(), SnapshotError> {
        let constraint_name = constraint.get_name().to_owned();
        if self.constraints.contains_key(&constraint_name) {
            return Err(SnapshotError::constraint_already_exists(
                &self.schema_name,
                &self.name,
                &constraint_name,
            ));
        }
        self.constraints.insert(constraint_name, constraint);
        return Ok(());
    }
    pub fn remove_constraint(&mut self, constraint: &str) -> Result<Constraint, SnapshotError> {
        self.constraints
            .remove(constraint)
            .ok_or(SnapshotError::constraint_not_found(
                &self.schema_name,
                &self.name,
                constraint,
            ))
    }
}
