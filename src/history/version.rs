use crate::snapshot::changes::{Change, CreateSchemaChange};
use serde::{Deserialize, Serialize};

pub const STAGE: &str = "stage";
pub const INIT: &str = "init";

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub previous: Option<String>,
    pub changes: Vec<Change>,
    pub up: Vec<String>,
    pub down: Vec<String>,
}

impl Version {
    pub fn new_init() -> Self {
        Self {
            name: INIT.to_string(),
            previous: None,
            changes: vec![CreateSchemaChange {
                schema_name: "public".to_string(),
            }
            .into()],
            up: vec!["CREATE SCHEMA public;".to_owned()],
            down: vec!["DROP SCHEMA public;".to_owned()],
        }
    }

    pub fn new_stage(previous: &str) -> Self {
        Self {
            name: STAGE.to_string(),
            previous: Some(previous.to_owned()),
            changes: vec![],
            up: vec![],
            down: vec![],
        }
    }

    pub fn add_change(&mut self, change: Change) {
        self.changes.push(change);
    }
    pub fn add_up(&mut self, up: &str) {
        self.up.push(up.to_owned());
    }
    pub fn add_down(&mut self, down: &str) {
        let previous = self.down.clone();
        self.down = vec![down.to_owned()];
        self.down.extend(previous)
    }
    pub fn reset(&mut self) {
        self.changes = vec![];
        self.up = vec![];
        self.down = vec![];
    }

    pub fn is_empty(&self) -> bool {
        self.changes.len() == 0 && self.up.len() == 0 && self.down.len() == 0
    }
}
