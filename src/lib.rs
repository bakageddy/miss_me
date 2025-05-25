use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Index {
    pub identifier: String,
    pub entries: HashMap<String, String>,
}

#[derive(Debug)]
pub enum IndexError {
    IdentifierExtractionErr,
    ExpectedDirectory,
    ReadDirectoryErr,
    ReadFileErr(String)
}

impl Index {
    pub async fn get_identifier(path: &PathBuf) -> Option<String> {
        if let Some(last_component) = path.components().last() {
            last_component
                .as_os_str()
                .to_str()
                .and_then(|s| Some(s.to_string()))
        } else {
            None
        }
    }

    // Expect the path to contain
    // INDEX file which is formatted as
    // 'ID'<SPACE>'URL'
    pub async fn get_entries(
        path: &PathBuf
    ) -> Result<HashMap<String, String>, IndexError> {
        if !path.is_dir() {
            return Err(IndexError::ExpectedDirectory)
        }

        let mut dir_handle = tokio::fs::read_dir(path).await.map_err(|e| {
            eprintln!("Failed to read directory: {e}");
            IndexError::ReadDirectoryErr
        })?;

        let mut entries = HashMap::new();
        while let Ok(entry) = dir_handle.next_entry().await {
            match entry {
                Some(path) => {
                    if !path.file_name().eq("INDEX") {
                        continue;
                    }

                    let body = tokio::fs::read_to_string(path.path()).await.map_err(|e| {
                        eprintln!("{e:?}");
                        IndexError::ReadFileErr(path.file_name().to_string_lossy().into_owned())
                    })?;


                    for line in body.split('\n') {
                        if let Some((key, url)) = line.split_once(' ') {
                            entries.insert(key.to_string(), url.to_string());
                        }
                    }
                },
                None => break,
            };
        }

        return Ok(entries)
    }

    pub async fn extract_index(path: PathBuf) -> Result<Index, IndexError> {
        let identifier = match Index::get_identifier(&path).await {
            Some(id) => id,
            None => return Err(IndexError::IdentifierExtractionErr),
        };

        let entries = Index::get_entries(&path).await?;

        Ok(Self {
            identifier,
            entries,
        })
    }
}
