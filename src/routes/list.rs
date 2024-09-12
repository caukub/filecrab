use anyhow::Context;
use serde::Serialize;
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    success: bool,
    message: String,
    files: Option<Vec<File>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct File {
    name: String,
    size: Option<u64>,
    modified: u64,
    editable: bool,
    directory: bool,
}

async fn _get_files(path: PathBuf) -> Result<Vec<File>, anyhow::Error> {
    let mut files = Vec::new();

    let mut read_dir = tokio::fs::read_dir(path).await.context("")?;

    while let Some(file) = read_dir.next_entry().await.context("")? {
        let metadata = file.metadata().await.context("")?;

        let name = file.file_name().into_string().unwrap();
        let modified = metadata
            .modified()
            .context("")?
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_secs();

        let directory = metadata.is_dir();
        let editable = true;

        let size = match directory {
            true => None,
            false => Some(metadata.len()),
        };

        files.push(File {
            name,
            size,
            modified,
            editable,
            directory,
        });
    }

    files.sort_by(_sort);

    Ok(files)
}

fn _sort(a: &File, b: &File) -> std::cmp::Ordering {
    if a.directory && !b.directory {
        std::cmp::Ordering::Less
    } else if !a.directory && b.directory {
        std::cmp::Ordering::Greater
    } else {
        a.name.to_lowercase().cmp(&b.name.to_lowercase())
    }
}
