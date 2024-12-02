use std::path::{Path, PathBuf};

use axum::extract::multipart::Field;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::error::AppError;

pub struct FileService {
    base_dir: PathBuf,
}

impl FileService {
    pub fn new<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            base_dir: dir.as_ref().to_owned(),
        }
    }

    pub async fn save(&self, ws_id: u64, mut field: Field<'_>) -> Result<Option<String>, AppError> {
        let filename = match field.file_name() {
            Some(filename) => filename.to_owned(),
            None => return Ok(None),
        };
        let dst = self.base_dir.join(&filename);
        if !dst.exists() {
            fs::create_dir_all(dst.parent().expect("file path parent should exists")).await?;
            let mut f = File::create(dst).await?;
            while let Some(chunk) = field
                .chunk()
                .await
                .map_err(|_| AppError::GeneralError("multipart error".to_owned()))?
            {
                f.write_all(&chunk).await?;
            }
        }
        Ok(Some(format!("/{}/{}", ws_id, filename)))
    }
}
