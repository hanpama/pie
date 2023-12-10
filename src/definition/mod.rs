use std::path::{Path, PathBuf};

use crate::{
    definition::{
        base::{yaml::render_nodes_to_mapping, Node},
        ddl::render_schema_definition,
    },
    error::AnyError,
    snapshot::Database,
};

use self::{base::yaml::parse_mapping_to_nodes, ddl::parse_schema_definition};

mod base;
mod ddl;
mod defaults;

pub fn save_snapshot(file_path: &Path, snapshot: &Database) -> Result<(), AnyError> {
    let mut nodes: Vec<Node> = Vec::new();

    for schema in snapshot.iter_schemas() {
        nodes.push(render_schema_definition(schema));
    }

    let mapping = render_nodes_to_mapping(&nodes);
    let got = serde_yaml::to_string(&mapping).unwrap();

    std::fs::write(file_path, got)?;

    return Ok(());
}

pub fn load_snpashot(dir_path: &PathBuf) -> Result<Database, AnyError> {
    let mut database = Database::new();

    let yaml_paths = collect_yaml_file(dir_path)?;

    for yaml_path in yaml_paths {
        load_yaml_file_into(&mut database, &yaml_path)?;
    }

    Ok(database)
}

fn load_yaml_file_into(database: &mut Database, file_path: &PathBuf) -> Result<(), AnyError> {
    let s = std::fs::read_to_string(file_path)?;
    let snapshot = load_yaml_string(&s)?;
    database.merge(snapshot)?;

    Ok(())
}

pub fn load_yaml_string(s: &str) -> Result<Database, AnyError> {
    let mapping: serde_yaml::Mapping = serde_yaml::from_str(s)?;
    let nodes = parse_mapping_to_nodes(&mapping)?;

    let mut database = Database::new();

    for node in nodes {
        let parsed_schema = parse_schema_definition(&node)?;
        if database.has_schema(&parsed_schema.name) {
            let target_schema = database.get_schema_mut(&parsed_schema.name)?;
            target_schema.merge_schema(parsed_schema)?;
        } else {
            database.add_schema(parsed_schema)?;
        }
    }

    Ok(database)
}

fn collect_yaml_file(dir_path: &PathBuf) -> Result<Vec<PathBuf>, AnyError> {
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in std::fs::read_dir(dir_path)? {
        let path = entry?.path();
        if path.is_dir() {
            let mut sub_paths = collect_yaml_file(&path)?;
            paths.append(&mut sub_paths);
        } else if path.extension().is_some_and(|ext| ext == "yaml") {
            paths.push(path);
        }
    }
    Ok(paths)
}
