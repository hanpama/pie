use crate::snapshot::SnapshotError;

use super::{Check, ForeignKey, PrimaryKey, Unique};

#[derive(PartialEq, Debug)]
pub enum Constraint {
    PrimaryKey(PrimaryKey),
    ForeignKey(ForeignKey),
    Unique(Unique),
    Check(Check),
}

impl Constraint {
    pub fn get_schema_name(&self) -> &str {
        match self {
            Constraint::PrimaryKey(pk) => &pk.schema_name,
            Constraint::ForeignKey(fk) => &fk.schema_name,
            Constraint::Unique(u) => &u.schema_name,
            Constraint::Check(c) => &c.schema_name,
        }
    }

    pub fn get_table_name(&self) -> &str {
        match self {
            Constraint::PrimaryKey(pk) => &pk.table_name,
            Constraint::ForeignKey(fk) => &fk.table_name,
            Constraint::Unique(u) => &u.table_name,
            Constraint::Check(c) => &c.table_name,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Constraint::PrimaryKey(pk) => &pk.name,
            Constraint::ForeignKey(fk) => &fk.name,
            Constraint::Unique(u) => &u.name,
            Constraint::Check(c) => &c.name,
        }
    }

    pub fn as_primary_key(&self) -> Result<&PrimaryKey, SnapshotError> {
        match self {
            Constraint::PrimaryKey(pk) => Ok(pk),
            _ => Err(self.format_unexpected_type_error("primary key")),
        }
    }
    pub fn as_primary_key_mut(&mut self) -> Result<&mut PrimaryKey, SnapshotError> {
        match self {
            Constraint::PrimaryKey(pk) => Ok(pk),
            _ => Err(self.format_unexpected_type_error("primary key")),
        }
    }

    pub fn as_foreign_key(&self) -> Result<&ForeignKey, SnapshotError> {
        match self {
            Constraint::ForeignKey(fk) => Ok(fk),
            _ => Err(self.format_unexpected_type_error("foreign key")),
        }
    }
    pub fn as_foreign_key_mut(&mut self) -> Result<&mut ForeignKey, SnapshotError> {
        match self {
            Constraint::ForeignKey(fk) => Ok(fk),
            _ => Err(self.format_unexpected_type_error("foreign key")),
        }
    }

    pub fn as_unique(&self) -> Result<&Unique, SnapshotError> {
        match self {
            Constraint::Unique(u) => Ok(u),
            _ => Err(self.format_unexpected_type_error("unique")),
        }
    }
    pub fn as_unique_mut(&mut self) -> Result<&mut Unique, SnapshotError> {
        match self {
            Constraint::Unique(u) => Ok(u),
            _ => Err(self.format_unexpected_type_error("unique")),
        }
    }

    pub fn as_check(&self) -> Result<&Check, SnapshotError> {
        match self {
            Constraint::Check(c) => Ok(c),
            _ => Err(self.format_unexpected_type_error("check")),
        }
    }
    pub fn as_check_mut(&mut self) -> Result<&mut Check, SnapshotError> {
        match self {
            Constraint::Check(c) => Ok(c),
            _ => Err(self.format_unexpected_type_error("check")),
        }
    }

    fn get_type(&self) -> &'static str {
        match self {
            Constraint::PrimaryKey(_) => "primary key",
            Constraint::ForeignKey(_) => "foreign key",
            Constraint::Unique(_) => "unique",
            Constraint::Check(_) => "check",
        }
    }

    fn format_unexpected_type_error(&self, expected: &'static str) -> SnapshotError {
        SnapshotError::constraint_has_unexpected_type(
            self.get_schema_name(),
            self.get_table_name(),
            self.get_name(),
            expected,
            self.get_type(),
        )
    }
}

impl Into<Constraint> for PrimaryKey {
    fn into(self) -> Constraint {
        Constraint::PrimaryKey(self)
    }
}
impl Into<Constraint> for ForeignKey {
    fn into(self) -> Constraint {
        Constraint::ForeignKey(self)
    }
}
impl Into<Constraint> for Unique {
    fn into(self) -> Constraint {
        Constraint::Unique(self)
    }
}
impl Into<Constraint> for Check {
    fn into(self) -> Constraint {
        Constraint::Check(self)
    }
}
