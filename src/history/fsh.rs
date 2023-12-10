use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    fs::File,
    path::PathBuf,
};

use super::{version::INIT, Version};

pub struct FSHistory {
    pub dir: PathBuf,
    pub next_map: HashMap<String, String>,
    pub current_version: String,
}

impl FSHistory {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            next_map: HashMap::new(),
            current_version: INIT.to_owned(),
        }
    }

    pub fn from_dir(dir: &PathBuf) -> Result<Self, FSHistoryError> {
        let mut history = FSHistory {
            dir: dir.clone(),
            next_map: HashMap::new(),
            current_version: INIT.to_owned(),
        };

        let dir_entries = std::fs::read_dir(&dir)?;
        for entry in dir_entries {
            let entry = entry?;
            let filetype = entry.file_type()?;
            let filename = entry.file_name().to_str().unwrap().to_owned();

            if filetype.is_dir() || !filename.ends_with(".yaml") {
                continue;
            }
            let version_name = filename.trim_end_matches(".yaml").to_owned();

            let v = history.get_version(&version_name)?;

            if let Some(previous) = v.previous {
                if let Some(existing) = history.next_map.get(&previous) {
                    return Err(FSHistoryError::Branched {
                        parent: previous.clone(),
                        child_a: existing.clone(),
                        child_b: v.name.clone(),
                    });
                }
                history.next_map.insert(previous, v.name.clone());
            }
        }

        let mut curr_version = "init".to_string();
        while let Some(next) = history.next_map.get(&curr_version) {
            history.current_version = curr_version.clone();
            curr_version = next.clone();
        }

        Ok(history)
    }

    pub fn get_current_version(&self) -> Result<Version, FSHistoryError> {
        self.get_version(&self.current_version)
    }

    pub fn get_version(&self, version_name: &str) -> Result<Version, FSHistoryError> {
        let version_file_path = self.get_version_file_path(version_name);
        let version_file = File::open(version_file_path)?;
        let version: Version = serde_yaml::from_reader(version_file)?;

        Ok(version)
    }

    pub fn get_upward_range(&self, from: &str, to: &str) -> Result<Vec<String>, FSHistoryError> {
        let mut versions = vec![];
        let mut curr = to.to_owned();

        loop {
            let v = self.get_version(&curr)?;

            if curr == from {
                break;
            } else if let Some(previous) = v.previous.as_ref() {
                curr = previous.clone();
                versions.push(v.name);
            } else {
                return Err(FSHistoryError::Unreachable {
                    high: to.to_owned(),
                    low: from.to_owned(),
                });
            }
        }
        versions.reverse();
        return Ok(versions);
    }

    pub fn save_version(&mut self, version: Version) -> Result<(), FSHistoryError> {
        // TODO: 인 메모리 변경 안해도 되나?...
        let version_file_path = self.get_version_file_path(&version.name);
        let version_file = File::create(version_file_path)?;
        serde_yaml::to_writer(version_file, &version)?;

        Ok(())
    }

    fn get_version_file_path(&self, version_name: &str) -> PathBuf {
        self.dir.join(format!("{}.yaml", version_name))
    }
}

#[derive(Debug)]
pub enum FSHistoryError {
    Branched {
        parent: String,
        child_a: String,
        child_b: String,
    },
    Unreachable {
        high: String,
        low: String,
    },
    FileSystemError(std::io::Error),
    SerdeYAMLError(serde_yaml::Error),
}

impl std::error::Error for FSHistoryError {}

impl From<std::io::Error> for FSHistoryError {
    fn from(err: std::io::Error) -> Self {
        Self::FileSystemError(err)
    }
}

impl From<serde_yaml::Error> for FSHistoryError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerdeYAMLError(err)
    }
}

impl Display for FSHistoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FSHistoryError::Branched {
                parent,
                child_a,
                child_b,
            } => write!(
                f,
                "version \"{}\" has two next versions: \"{}\" and \"{}\"",
                parent, child_a, child_b
            ),
            FSHistoryError::Unreachable { high, low } => write!(
                f,
                "version \"{}\" is not reachable from version \"{}\"",
                high, low
            ),
            FSHistoryError::FileSystemError(err) => write!(f, "{}", err),
            FSHistoryError::SerdeYAMLError(err) => write!(f, "{}", err),
        }
    }
}
