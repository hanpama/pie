use postgres::Transaction;

use crate::{
    error::AnyError,
    snapshot::{
        Check, Column, Constraint, Database, ForeignKey, Function, Index, PrimaryKey, Relation,
        Schema, Sequence, Table, Unique,
    },
};

mod check;
mod column;
mod foreignkey;
mod function;
mod index;
mod primarykey;
mod schema;
mod sequence;
mod table;
mod unique;
mod view;

pub fn introspect(tx: &mut Transaction, schemas: &[&str]) -> Result<Database, AnyError> {
    let mut database = Database::new();

    load_schemas(tx, &mut database, schemas)?;
    load_sequences(tx, &mut database, schemas)?;
    load_tables(tx, &mut database, schemas)?;
    load_columns(tx, &mut database, schemas)?;
    load_primary_keys(tx, &mut database, schemas)?;
    load_foreign_keys(tx, &mut database, schemas)?;
    load_functions(tx, &mut database, schemas)?;
    load_unique(tx, &mut database, schemas)?;
    load_checks(tx, &mut database, schemas)?;
    load_indexes(tx, &mut database, schemas)?;

    Ok(database)
}

fn load_schemas(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for ischema in schema::introspect_schemas(tx, schemas)? {
        let schema = Schema::new(&ischema.name);
        database.add_schema(schema)?;
    }
    Ok(())
}

fn load_sequences(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for iseq in sequence::introspect_sequences(tx, schemas)? {
        let sequence = Sequence {
            name: iseq.name,
            schema_name: iseq.schema,
            data_type: iseq.data_type,
            start: iseq.start,
            min_value: iseq.min_value,
            max_value: iseq.max_value,
            increment: iseq.increment,
            cycle: iseq.cycle,
            cache: iseq.cache,
            owned_by_column: iseq.owned_by_column_name,
            owned_by_table: iseq.owned_by_table_name,
        };
        database
            .get_schema_mut(&sequence.schema_name)?
            .add_relation(Relation::Sequence(sequence))?;
    }
    Ok(())
}

fn load_tables(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for itable in table::introspect_tables(tx, schemas)? {
        let table = Table::new(&itable.schema, &itable.name);
        database
            .get_schema_mut(&itable.schema)?
            .add_relation(Relation::Table(table))?;
    }
    Ok(())
}

fn load_columns(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for icolumn in column::introspect_columns(tx, schemas)? {
        let column = Column {
            name: icolumn.name,
            schema_name: icolumn.schema,
            table_name: icolumn.table,
            data_type: icolumn.data_type,
            default: if icolumn.default.is_empty() {
                None
            } else {
                Some(icolumn.default)
            },
            not_null: icolumn.not_null,
        };
        database
            .get_schema_mut(&column.schema_name)?
            .get_relation_mut(&column.table_name)?
            .as_table_mut()?
            .add_column(column)?;
    }
    Ok(())
}

fn load_primary_keys(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for ipk in primarykey::introspect_primary_keys(tx, schemas)? {
        let primary_key = PrimaryKey {
            name: ipk.name,
            schema_name: ipk.schema,
            table_name: ipk.table_name,
            columns: ipk.table_columns,
            deferrable: ipk.deferrable,
            initially_deferred: ipk.initially_deferred,
        };

        database
            .get_schema_mut(&primary_key.schema_name)?
            .get_relation_mut(&primary_key.table_name)?
            .as_table_mut()?
            .add_constraint(Constraint::PrimaryKey(primary_key))?;
    }
    Ok(())
}

fn load_unique(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for iunique in unique::introspect_uniques(tx, schemas)? {
        let unique = Unique {
            name: iunique.name,
            schema_name: iunique.schema,
            table_name: iunique.table_name,
            columns: iunique.table_columns,
            deferrable: iunique.deferrable,
            initially_deferred: iunique.initially_deferred,
        };

        database
            .get_schema_mut(&unique.schema_name)?
            .get_relation_mut(&unique.table_name)?
            .as_table_mut()?
            .add_constraint(Constraint::Unique(unique))?;
    }
    Ok(())
}

fn load_foreign_keys(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for ifk in foreignkey::introspect_foreign_keys(tx, schemas)? {
        let foreign_key = ForeignKey {
            name: ifk.constraint_name,
            schema_name: ifk.constraint_schema,
            table_name: ifk.constraint_table_name,
            columns: ifk.columns,
            target_schema: ifk.target_table_schema,
            target_table: ifk.target_table_name,
            target_columns: ifk.target_table_columns,
            deferrable: ifk.deferrable,
            initially_deferred: ifk.initially_deferred,
            match_option: ifk.match_option,
            update_rule: ifk.update_rule,
            delete_rule: ifk.delete_rule,
        };

        database
            .get_schema_mut(&foreign_key.schema_name)?
            .get_relation_mut(&foreign_key.table_name)?
            .as_table_mut()?
            .add_constraint(Constraint::ForeignKey(foreign_key))?;
    }
    Ok(())
}

fn load_checks(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for icheck in check::introspect_checks(tx, schemas)? {
        let check = Check {
            name: icheck.name,
            schema_name: icheck.schema,
            table_name: icheck.table,
            expression: icheck.check_clause,
            deferrable: icheck.is_deferrable,
            initially_deferred: icheck.initially_deferred,
        };
        database
            .get_schema_mut(&check.schema_name)?
            .get_relation_mut(&check.table_name)?
            .as_table_mut()?
            .add_constraint(Constraint::Check(check))?;
    }
    Ok(())
}

fn load_indexes(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for iindex in index::introspect_indexes(tx, schemas)? {
        let key_expressions = if let Some(expressions) = iindex.expressions {
            vec![expressions] //
        } else {
            iindex.key_columns.unwrap()
        };

        let index = Index {
            name: iindex.index_name,
            schema_name: iindex.schema_name,
            table_name: iindex.table_name,
            key_expressions: key_expressions,
            unique: iindex.unique,
            method: iindex.method,
        };
        database
            .get_schema_mut(&index.schema_name)?
            .add_relation(Relation::Index(index))?
    }
    Ok(())
}

fn load_functions(
    tx: &mut Transaction,
    database: &mut Database,
    schemas: &[&str],
) -> Result<(), AnyError> {
    for ifunction in function::introspect_functions(tx, schemas)? {
        let function = Function {
            name: ifunction.name,
            schema_name: ifunction.schema,
            language: ifunction.language,
            returns: ifunction.returns,
            volatility: ifunction.volatility,
            body: ifunction.body,
        };
        database
            .get_schema_mut(&function.schema_name)?
            .add_function(function)?;
    }
    Ok(())
}
