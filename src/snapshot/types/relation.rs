use crate::snapshot::SnapshotError;

use super::{Index, Sequence, Table, View};

#[derive(PartialEq, Debug)]
pub enum Relation {
    Table(Table),
    View(View),
    Index(Index),
    Sequence(Sequence),
}

impl Relation {
    pub fn get_schema_name(&self) -> &str {
        match self {
            Relation::Table(table) => &table.schema_name,
            Relation::View(view) => &view.schema_name,
            Relation::Index(index) => &index.schema_name,
            Relation::Sequence(sequence) => &sequence.schema_name,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Relation::Table(table) => &table.name,
            Relation::View(view) => &view.name,
            Relation::Index(index) => &index.name,
            Relation::Sequence(sequence) => &sequence.name,
        }
    }

    pub fn as_table(&self) -> Result<&Table, SnapshotError> {
        if let Relation::Table(table) = self {
            return Ok(table);
        }
        Err(self.format_unexpected_type_error("table"))
    }
    pub fn as_table_mut(&mut self) -> Result<&mut Table, SnapshotError> {
        if let Relation::Table(table) = self {
            return Ok(table);
        }
        Err(self.format_unexpected_type_error("table"))
    }
    pub fn as_view(&self) -> Result<&View, SnapshotError> {
        if let Relation::View(view) = self {
            return Ok(view);
        }
        Err(self.format_unexpected_type_error("view"))
    }
    pub fn as_view_mut(&mut self) -> Result<&mut View, SnapshotError> {
        if let Relation::View(view) = self {
            return Ok(view);
        }
        Err(self.format_unexpected_type_error("view"))
    }
    pub fn as_index(&self) -> Result<&Index, SnapshotError> {
        if let Relation::Index(index) = self {
            return Ok(index);
        }
        Err(self.format_unexpected_type_error("index"))
    }
    pub fn as_index_mut(&mut self) -> Result<&mut Index, SnapshotError> {
        if let Relation::Index(index) = self {
            return Ok(index);
        }
        Err(self.format_unexpected_type_error("index"))
    }
    pub fn as_sequence(&self) -> Result<&Sequence, SnapshotError> {
        if let Relation::Sequence(sequence) = self {
            return Ok(sequence);
        }
        Err(self.format_unexpected_type_error("sequence"))
    }
    pub fn as_sequence_mut(&mut self) -> Result<&mut Sequence, SnapshotError> {
        if let Relation::Sequence(sequence) = self {
            return Ok(sequence);
        }
        Err(self.format_unexpected_type_error("sequence"))
    }

    fn get_type(&self) -> &'static str {
        match self {
            Relation::Table(_) => "table",
            Relation::View(_) => "view",
            Relation::Index(_) => "index",
            Relation::Sequence(_) => "sequence",
        }
    }

    fn format_unexpected_type_error(&self, expected: &'static str) -> SnapshotError {
        SnapshotError::relation_has_unexpected_type(
            self.get_schema_name(),
            self.get_name(),
            expected,
            self.get_type(),
        )
    }
}

impl Into<Relation> for Table {
    fn into(self) -> Relation {
        Relation::Table(self)
    }
}
impl Into<Relation> for View {
    fn into(self) -> Relation {
        Relation::View(self)
    }
}
impl Into<Relation> for Index {
    fn into(self) -> Relation {
        Relation::Index(self)
    }
}
impl Into<Relation> for Sequence {
    fn into(self) -> Relation {
        Relation::Sequence(self)
    }
}
