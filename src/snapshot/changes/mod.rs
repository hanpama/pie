use super::Database;
use super::SnapshotError;
use serde::{Deserialize, Serialize};

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

pub use check::*;
pub use column::*;
pub use foreignkey::*;
pub use function::*;
pub use index::*;
pub use primarykey::*;
pub use schema::*;
pub use sequence::*;
pub use table::*;
pub use unique::*;
pub use view::*;

macro_rules! define_change_impl {
    ($enum_name:ident, $($variant:ident),*) => {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        #[serde(tag="type", content="change")]
        pub enum $enum_name {
            $( $variant($variant), )*
        }

        impl $enum_name {
            pub fn apply(&self, source: &mut Database) -> Result<(), SnapshotError> {
                match self {
                    $( $enum_name::$variant(change) => change.apply(source), )*
                }
            }
            pub fn render_sql(&self) -> String {
                match self {
                    $( $enum_name::$variant(change) => change.render_sql(), )*
                }
            }
            pub fn revert(&self, target: &Database) -> Result<Change, SnapshotError> {
                match self {
                    $( $enum_name::$variant(change) => change.revert(target), )*
                }
            }
            pub fn display_name(&self) -> &str {
                match self {
                    $( $enum_name::$variant(_) => stringify!($variant), )*
                }
            }
        }
        $(
            impl From<$variant> for $enum_name {
                fn from(change: $variant) -> Self {
                    $enum_name::$variant(change)
                }
            }
        )*
    };
}

define_change_impl!(
    Change,
    AddCheckChange,
    AddColumnChange,
    AddForeignKeyChange,
    AddPrimaryKeyChange,
    AddUniqueChange,
    AlterCheckChange,
    AlterColumnSetDataTypeChange,
    AlterColumnSetDefaultChange,
    AlterColumnSetNotNullChange,
    AlterForeignKeyChange,
    AlterPrimaryKeyChange,
    AlterUniqueChange,
    CreateFunctionChange,
    CreateIndexChange,
    CreateSchemaChange,
    CreateSequenceChange,
    CreateTableChange,
    CreateViewChange,
    DropCheckChange,
    DropColumnChange,
    DropForeignKeyChange,
    DropFunctionChange,
    DropIndexChange,
    DropPrimaryKeyChange,
    DropSchemaChange,
    DropSequenceChange,
    DropTableChange,
    DropUniqueChange,
    DropViewChange,
    RenameColumnChange
);
