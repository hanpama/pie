use super::changes::*;
use super::types::*;

pub struct Changeset {
    changes: Vec<Change>,
}

impl Changeset {
    pub fn new() -> Self {
        Changeset { changes: vec![] }
    }
    pub fn push<T: Into<Change>>(&mut self, change: T) {
        self.changes.push(change.into());
    }
    pub fn append(&mut self, other: &mut Changeset) {
        self.changes.append(&mut other.changes);
    }
    pub fn len(&self) -> usize {
        self.changes.len()
    }
}

impl IntoIterator for Changeset {
    type Item = Change;
    type IntoIter = std::vec::IntoIter<Change>;

    fn into_iter(self) -> Self::IntoIter {
        self.changes.into_iter()
    }
}

pub fn compare_diff(s: &Database, t: &Database) -> Vec<Change> {
    return diff_database_update(s, t).changes;
}

fn diff_database_update(s: &Database, t: &Database) -> Changeset {
    let mut changes = Changeset::new();

    for ti in t.iter_schemas() {
        if !s.has_schema(ti.get_name()) {
            changes.append(&mut diff_schema_create(ti));
        }
    }
    for si in s.iter_schemas() {
        if t.has_schema(si.get_name()) {
            let ti = t.get_schema(si.get_name()).unwrap();
            changes.append(&mut diff_schema_update(si, ti));
        } else {
            changes.append(&mut diff_schema_drop(si));
        }
    }

    return changes;
}

fn diff_schema_create(t: &Schema) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(CreateSchemaChange::new(t));

    for ti in t.iter_relations() {
        changes.append(&mut diff_relation_create(ti));
    }
    for ti in t.iter_functions() {
        changes.append(&mut diff_function_create(ti));
    }

    return changes;
}

fn diff_schema_update(s: &Schema, t: &Schema) -> Changeset {
    let mut changes = Changeset::new();

    for ti in t.iter_relations() {
        if !s.has_relation(ti.get_name()) {
            changes.append(&mut diff_relation_create(ti));
        }
    }
    for si in s.iter_relations() {
        if t.has_relation(si.get_name()) {
            let ti = t.get_relation(si.get_name()).unwrap();
            changes.append(&mut diff_relation_update(si, ti));
        } else {
            changes.append(&mut diff_relation_drop(si));
        }
    }

    for ti in t.iter_functions() {
        if !s.has_function(ti.get_name()) {
            changes.append(&mut diff_function_create(ti));
        }
    }
    for si in s.iter_functions() {
        if t.has_function(si.get_name()) {
            let ti = t.get_function(si.get_name()).unwrap();
            changes.append(&mut diff_function_update(si, ti));
        } else {
            changes.append(&mut diff_function_drop(si));
        }
    }

    return changes;
}

fn diff_schema_drop(s: &Schema) -> Changeset {
    let mut changes = Changeset::new();

    for si in s.iter_relations() {
        changes.append(&mut diff_relation_drop(si));
    }
    for si in s.iter_functions() {
        changes.append(&mut diff_function_drop(si));
    }

    return changes;
}

fn diff_relation_create(t: &Relation) -> Changeset {
    match t {
        Relation::Table(t) => diff_table_create(t),
        Relation::View(t) => diff_view_create(t),
        Relation::Sequence(t) => diff_sequence_create(t),
        Relation::Index(t) => diff_index_create(t),
    }
}

fn diff_relation_update(s: &Relation, t: &Relation) -> Changeset {
    let mut changes = Changeset::new();

    match (s, t) {
        (Relation::Table(s), Relation::Table(t)) => {
            changes.append(&mut diff_table_update(s, t));
        }
        (Relation::View(s), Relation::View(t)) => {
            changes.append(&mut diff_view_update(s, t));
        }
        (Relation::Sequence(s), Relation::Sequence(t)) => {
            changes.append(&mut diff_sequence_update(s, t));
        }
        (Relation::Index(s), Relation::Index(t)) => {
            changes.append(&mut diff_index_update(s, t));
        }
        _ => {
            changes.append(&mut diff_relation_drop(s));
            changes.append(&mut diff_relation_create(t));
        }
    }

    return changes;
}

fn diff_relation_drop(s: &Relation) -> Changeset {
    match s {
        Relation::Table(t) => diff_table_drop(t),
        Relation::View(t) => diff_view_drop(t),
        Relation::Sequence(t) => diff_sequence_drop(t),
        Relation::Index(t) => diff_index_drop(t),
    }
}

fn diff_table_create(t: &Table) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(CreateTableChange::new(t));

    // constraints
    for ti in t.iter_constraints() {
        changes.append(&mut diff_constraint_create(ti));
    }

    return changes;
}

fn diff_table_update(s: &Table, t: &Table) -> Changeset {
    let mut changes = Changeset::new();

    // columns
    for ti in t.iter_columns() {
        if s.has_column(ti.get_name()) {
            let si = s.get_column(ti.get_name()).unwrap();
            changes.append(&mut diff_column_update(si, ti));
        } else {
            changes.append(&mut diff_column_create(ti));
        }
    }
    for si in s.iter_columns() {
        if t.has_column(si.get_name()) {
            let ti = t.get_column(si.get_name()).unwrap();
            changes.append(&mut diff_column_update(si, ti));
        } else {
            changes.append(&mut diff_column_drop(si));
        }
    }

    // constraints
    for ti in t.iter_constraints() {
        if s.has_constraint(ti.get_name()) {
            let si = s.get_constraint(ti.get_name()).unwrap();
            changes.append(&mut diff_constraint_update(si, ti));
        } else {
            changes.append(&mut diff_constraint_create(ti));
        }
    }

    return changes;
}

fn diff_table_drop(s: &Table) -> Changeset {
    let mut changes = Changeset::new();

    // constraints
    for si in s.iter_constraints() {
        changes.append(&mut diff_constraint_drop(si));
    }
    // columns
    for si in s.iter_columns() {
        changes.append(&mut diff_column_drop(si));
    }

    changes.push(DropTableChange::new(s));

    return changes;
}

fn diff_column_create(t: &Column) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(AddColumnChange::new(t));

    return changes;
}

fn diff_column_update(s: &Column, t: &Column) -> Changeset {
    let mut changes = Changeset::new();

    if s.data_type != t.data_type {
        changes.push(AlterColumnSetDataTypeChange::new(t));
    }
    if s.default != t.default {
        changes.push(AlterColumnSetDefaultChange::new(t));
    }
    if s.not_null != t.not_null {
        changes.push(AlterColumnSetNotNullChange::new(t));
    }

    return changes;
}

fn diff_column_drop(s: &Column) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(DropColumnChange::new(s));

    return changes;
}

fn diff_constraint_create(t: &Constraint) -> Changeset {
    match t {
        Constraint::PrimaryKey(t) => diff_primary_key_create(t),
        Constraint::Unique(t) => diff_unique_create(t),
        Constraint::ForeignKey(t) => diff_foreign_key_create(t),
        Constraint::Check(t) => diff_check_create(t),
    }
}
fn diff_constraint_update(s: &Constraint, t: &Constraint) -> Changeset {
    match (s, t) {
        (Constraint::PrimaryKey(s), Constraint::PrimaryKey(t)) => diff_primary_key_update(s, t),
        (Constraint::Unique(s), Constraint::Unique(t)) => diff_unique_update(s, t),
        (Constraint::ForeignKey(s), Constraint::ForeignKey(t)) => diff_foreign_key_update(s, t),
        (Constraint::Check(s), Constraint::Check(t)) => diff_check_update(s, t),
        _ => {
            let mut changes = Changeset::new();
            changes.append(&mut diff_constraint_drop(s));
            changes.append(&mut diff_constraint_create(t));
            changes
        }
    }
}
fn diff_constraint_drop(s: &Constraint) -> Changeset {
    match s {
        Constraint::PrimaryKey(s) => diff_primary_key_drop(s),
        Constraint::Unique(s) => diff_unique_drop(s),
        Constraint::ForeignKey(s) => diff_foreign_key_drop(s),
        Constraint::Check(s) => diff_check_drop(s),
    }
}

fn diff_primary_key_create(t: &PrimaryKey) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(AddPrimaryKeyChange::new(t));
    return changes;
}

fn diff_primary_key_update(s: &PrimaryKey, t: &PrimaryKey) -> Changeset {
    let mut changes = Changeset::new();

    if s.columns != t.columns {
        changes.push(DropPrimaryKeyChange::new(s));
        changes.push(AddPrimaryKeyChange::new(t));
    } else if s.deferrable != t.deferrable || s.initially_deferred != t.initially_deferred {
        changes.push(AlterPrimaryKeyChange::new(t));
    }

    return changes;
}

fn diff_primary_key_drop(s: &PrimaryKey) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(DropPrimaryKeyChange {
        schema: s.schema_name.clone(),
        table: s.table_name.clone(),
        constraint: s.name.clone(),
    });

    return changes;
}

fn diff_unique_create(t: &Unique) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(AddUniqueChange::new(t));

    return changes;
}

fn diff_unique_update(s: &Unique, t: &Unique) -> Changeset {
    let mut changes = Changeset::new();

    if s.columns != t.columns {
        changes.push(DropUniqueChange::new(s));
        changes.push(AddUniqueChange::new(t));
    } else if s.deferrable != t.deferrable || s.initially_deferred != t.initially_deferred {
        changes.push(AlterUniqueChange::new(t));
    }

    return changes;
}

fn diff_unique_drop(s: &Unique) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(DropUniqueChange {
        schema: s.schema_name.clone(),
        table: s.table_name.clone(),
        constraint: s.name.clone(),
    });

    return changes;
}

fn diff_foreign_key_create(t: &ForeignKey) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(AddForeignKeyChange::new(t));
    return changes;
}

fn diff_foreign_key_update(s: &ForeignKey, t: &ForeignKey) -> Changeset {
    let mut changes = Changeset::new();

    if s.columns != t.columns
        || s.target_schema != t.target_schema
        || s.target_table != t.target_table
        || s.target_columns != t.target_columns
        || s.match_option != t.match_option
        || s.update_rule != t.update_rule
        || s.delete_rule != t.delete_rule
    {
        changes.push(DropForeignKeyChange::new(s));
        changes.push(AddForeignKeyChange::new(t));
    } else if s.deferrable != t.deferrable || s.initially_deferred != t.initially_deferred {
        changes.push(AlterForeignKeyChange::new(t));
    }

    return changes;
}

fn diff_foreign_key_drop(s: &ForeignKey) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(DropForeignKeyChange::new(s));
    return changes;
}

fn diff_check_create(t: &Check) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(AddCheckChange::new(t));
    return changes;
}

fn diff_check_update(s: &Check, t: &Check) -> Changeset {
    let mut changes = Changeset::new();

    if s.expression != t.expression {
        changes.push(DropCheckChange::new(s));
        changes.push(AddCheckChange::new(t));
    } else if s.deferrable != t.deferrable || s.initially_deferred != t.initially_deferred {
        changes.push(AlterCheckChange::new(t));
    }

    return changes;
}

fn diff_check_drop(s: &Check) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(DropCheckChange::new(s));
    return changes;
}

fn diff_view_create(t: &View) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(CreateViewChange::new(t));
    return changes;
}

fn diff_view_update(s: &View, t: &View) -> Changeset {
    let mut changes = Changeset::new();

    if s.query != t.query {
        changes.push(DropViewChange::new(s));
        changes.push(CreateViewChange::new(t));
    }

    return changes;
}

fn diff_view_drop(s: &View) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(DropViewChange::new(s));
    return changes;
}

fn diff_function_create(t: &Function) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(CreateFunctionChange::new(t));
    return changes;
}

fn diff_function_update(s: &Function, t: &Function) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(DropFunctionChange::new(s));
    changes.push(CreateFunctionChange::new(t));

    return changes;
}

fn diff_function_drop(s: &Function) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(DropFunctionChange::new(s));
    return changes;
}

fn diff_index_create(t: &Index) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(CreateIndexChange::new(t));
    return changes;
}

fn diff_index_update(s: &Index, t: &Index) -> Changeset {
    let mut changes = Changeset::new();

    if s.key_expressions != t.key_expressions {
        changes.push(DropIndexChange::new(s));
        changes.push(CreateIndexChange::new(t));
    }

    return changes;
}

fn diff_index_drop(s: &Index) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(DropIndexChange::new(s));
    return changes;
}

fn diff_sequence_create(t: &Sequence) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(CreateSequenceChange::new(t));
    return changes;
}

fn diff_sequence_update(s: &Sequence, t: &Sequence) -> Changeset {
    let mut changes = Changeset::new();

    changes.push(DropSequenceChange::new(s));
    changes.push(CreateSequenceChange::new(t));

    return changes;
}

fn diff_sequence_drop(s: &Sequence) -> Changeset {
    let mut changes = Changeset::new();
    changes.push(DropSequenceChange::new(s));
    return changes;
}
