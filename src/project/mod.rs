use crate::util::expand::expand_envvar;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display, fs, path::PathBuf};

// mod yaml;

/// Project
///
/// A project is a directory with a .podo.yaml file in it.
///
/// ```text
/// .
/// ├── .podo.yaml
/// └── database
///     ├── definitions
///     │   └── public.yaml
///     └── history
///         ├── init.yaml
///         └── stage.yaml
/// ```
///
/// The .podo.yaml file contains the following:
///
/// ```yaml
/// database:
///   url: postgresql://localhost:5432/postgres
///   definition: database/definitions
///   history: database/history
/// ```
///
/// Environment variables can be used in the profile fields:
///
/// ```yaml
/// database:
///   url: $DATABASE_URL
///   definition: database/definitions
///   history: database/history
/// ```
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    base_dir: PathBuf,
    profiles: Vec<Profile>,
}

impl Project {
    fn get_profile(&self, profile_name: &str) -> Result<&Profile, ProjectError> {
        for profile in self.profiles.iter() {
            if profile.name == profile_name {
                return Ok(profile);
            }
        }
        Err(ProjectError::ProfileNotFound {
            profile_name: profile_name.to_owned(),
        })
    }

    pub fn resolve_database_url(&self, profile_name: &str) -> Result<String, ProjectError> {
        let profile = self.get_profile(profile_name)?;
        Ok(profile.database_url.clone())
    }
    pub fn resolve_history_dir(&self, profile_name: &str) -> Result<PathBuf, ProjectError> {
        let profile = self.get_profile(profile_name)?;
        Ok(profile.history.clone())
    }
    pub fn resolve_definitions_dir(&self, profile_name: &str) -> Result<PathBuf, ProjectError> {
        let profile = self.get_profile(profile_name)?;
        Ok(profile.definitions.clone())
    }
    pub fn resolve_default_history_dir(&self) -> Result<PathBuf, ProjectError> {
        self.resolve_history_dir(DEFAULT_PROFILE)
    }
    pub fn resolve_default_definitions_dir(&self) -> Result<PathBuf, ProjectError> {
        self.resolve_definitions_dir(DEFAULT_PROFILE)
    }
    pub fn resolve_metadata_schema(&self, profile_name: &str) -> Result<String, ProjectError> {
        let profile = self.get_profile(profile_name)?;
        Ok(profile.metadata_schema.clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    name: String,
    database_url: String,
    history: PathBuf,
    definitions: PathBuf,
    metadata_schema: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YAMLProject {
    pub profiles: HashMap<String, YAMLProjectProfile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YAMLProjectProfile {
    pub database_url: String,
    pub history: String,
    pub definitions: String,
    pub metadata_schema: String,
}

const CONFIG_FILE: &str = ".podo.yaml";
const DEFAULT_PROFILE: &str = "database";

pub fn discover_project(current_dir: PathBuf) -> Result<Project, ProjectError> {
    let mut current_dir = current_dir;
    loop {
        let config_file = current_dir.join(CONFIG_FILE);
        if config_file.exists() {
            let project = load_project_from_file(current_dir, config_file)?;
            return Ok(project);
        }

        if !current_dir.pop() {
            return Err(ProjectError::ProjectNotFound {
                current_dir: current_dir.clone(),
            }
            .into());
        }
    }
}

pub fn initialize_project(base_dir: PathBuf) -> Result<Project, ProjectError> {
    let yaml_project = create_base_yaml_project();
    let s = serde_yaml::to_string(&yaml_project).unwrap();
    let config_file = base_dir.join(CONFIG_FILE);
    fs::write(config_file, &s).unwrap();
    load_project_from_str(base_dir, &s)
}

fn load_project_from_file(
    base_dir: PathBuf,
    config_file: PathBuf,
) -> Result<Project, ProjectError> {
    let s = fs::read_to_string(config_file)?;
    load_project_from_str(base_dir, &s)
}

fn load_project_from_str(base_dir: PathBuf, s: &str) -> Result<Project, ProjectError> {
    let yaml_project: YAMLProject = serde_yaml::from_str(s)?;
    let mut profiles: Vec<Profile> = Vec::new();

    for (name, profile) in yaml_project.profiles.iter() {
        profiles.push(Profile {
            name: name.clone(),
            database_url: expand_envvar(&profile.database_url),
            history: base_dir.join(expand_envvar(&profile.history)),
            definitions: base_dir.join(expand_envvar(&profile.definitions)),
            metadata_schema: profile.metadata_schema.clone(),
        });
    }

    Ok(Project { base_dir, profiles })
}

pub fn create_base_yaml_project() -> YAMLProject {
    let mut project = YAMLProject {
        profiles: HashMap::new(),
    };
    project.profiles.insert(
        DEFAULT_PROFILE.to_owned(),
        YAMLProjectProfile {
            database_url: "postgresql://localhost:5432/postgres".to_string(),
            history: "database/history".to_string(),
            definitions: "database/definitions".to_string(),
            metadata_schema: "podo_meta".to_string(),
        },
    );
    project
}

#[derive(Debug)]
pub enum ProjectError {
    ProjectNotFound { current_dir: PathBuf },
    ProfileNotFound { profile_name: String },
    IOError { error: std::io::Error },
    SerdeYAMLError { error: serde_yaml::Error },
}

impl From<std::io::Error> for ProjectError {
    fn from(error: std::io::Error) -> Self {
        ProjectError::IOError { error }
    }
}

impl From<serde_yaml::Error> for ProjectError {
    fn from(error: serde_yaml::Error) -> Self {
        ProjectError::SerdeYAMLError { error }
    }
}

impl Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::ProjectNotFound { current_dir } => {
                write!(
                    f,
                    "project not found in directory tree: {}",
                    current_dir.display()
                )
            }
            ProjectError::ProfileNotFound { profile_name } => {
                write!(f, "profile not found: {}", profile_name)
            }
            ProjectError::IOError { error } => {
                write!(f, "error reading project configuration: {}", error)
            }
            ProjectError::SerdeYAMLError { error } => {
                write!(f, "error parsing project configuration: {}", error)
            }
        }
    }
}

impl std::error::Error for ProjectError {}
