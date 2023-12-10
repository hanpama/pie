use crate::error::AnyError;

/// sync calculates the changes between the definition snapshot and the database snapshot.
/// and sets the database version it to the given version
/// When the database status is different from the definition status, the user is prompted to
/// confirm the changes.
pub fn sync() -> Result<(), AnyError> {
    todo!()
}
