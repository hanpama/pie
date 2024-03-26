#[cfg(test)]
use crate::snapshot::{Database, changes::Change, compare_diff};


#[cfg(test)]
pub fn run_snapshot_diffing_test(mut source: Database, target: Database, expected: Vec<Change>) {
    let changes = compare_diff(&source, &target);

    assert_eq!(changes, expected);

    let mut reverts: Vec<Change> = vec![];
    changes.iter().for_each(|change| {
        reverts.push(change.revert(&source).unwrap());
        change.apply(&mut source).unwrap();
    });
    reverts.reverse();
    
    assert_eq!(compare_diff(&source, &target), vec![]);
    
    reverts.iter().for_each(|change| {
        change.apply(&mut source).unwrap();
    });

    assert_eq!(compare_diff(&source, &target), expected);
}
