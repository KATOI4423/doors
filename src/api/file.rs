//! # file.rs
//!
//! Defines File Entry API

use std::fs::metadata;
use std::path::{
    Path,
    PathBuf,
};

use anyhow::Result;

// TODO: シンボリックリンク、ショートカットを追加
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileEntryKind {
    File { size: u64 },
    Directory,
}

impl TryFrom<&Path> for FileEntryKind {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self> {
        let metadata = match metadata(path) {
            Ok(md) => md,
            Err(e) => {
                eprintln!("Failed to get metadata for {}: {e}", path.display());
                anyhow::bail!(e)
            }
        };
        if metadata.is_dir() {
            return Ok(Self::Directory);
        }
        if metadata.is_file() {
            let size = metadata.len();
            return Ok(Self::File { size });
        }

        anyhow::bail!("Unknown Entry")
    }
}

impl FileEntryKind {
    pub fn size(&self) -> String {
        const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
        const K: f64 = 1024.0;

        let mut size = match self {
            Self::File { size } => size.clone(),
            Self::Directory => return "".into(), // ディレクトリの場合はファイルサイズを表示しない
        } as f64;
        let mut unit = 0;

        while size >= K && unit < UNITS.len() {
            size /= K;
            unit += 1;
        }

        if unit == 0 {
            format!("{size:.0} {}", UNITS[unit])
        } else {
            format!("{size:.1} {}", UNITS[unit])
        }
    }

    pub fn icon_path(&self) -> PathBuf {
        match self {
            Self::Directory => PathBuf::from("icons/folder.svg"),
            Self::File { .. } => PathBuf::from("icons/file.svg"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileEntry {
    path: PathBuf,
    name: String,
    kind: FileEntryKind,
}

impl FileEntry {
    pub fn new(
        path: impl Into<PathBuf>,
        name: impl Into<String>,
        kind: FileEntryKind,
    ) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            kind,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> FileEntryKind {
        self.kind
    }

    pub fn read_directory(path: &Path) -> Result<Vec<Self>> {
        let mut entries = Vec::new();

        for entry in std::fs::read_dir(path)? {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Failed to parse DirEntry: {e}");
                    continue;
                }
            };
            let path = entry.path();
            let kind = {
                let path: &Path = &path;
                match FileEntryKind::try_from(path) {
                    Ok(k) => k,
                    Err(e) => {
                        eprintln!("Failed to get FileEntryKind for {}: {e}", path.display());
                        continue;
                    }
                }
            };

            entries.push(Self::new(
                path,
                entry.file_name().to_string_lossy().into_owned(),
                kind,
            ))
        }

        dbg!("All directory are OK");
        Ok(entries)
    }
}
