use serde_json::Value;

use crate::db::models::File;
use crate::services::errors::FileErrors;
use crate::db::repositories::{files_repo::FileRepository, bins_repo::BinRepository};

#[derive(Clone)]
pub struct FileService {
    file_repo: FileRepository,
    bin_repo: BinRepository,
}

impl FileService {
    pub fn new(file_repo: FileRepository, bin_repo: BinRepository) -> Self {
        Self {
            file_repo,
            bin_repo
        }
    }

    pub async fn create_file(&self, name: &str, seed: i64, min_fill: f64, tolerance: f64, dominance_gap: Option<f64>, target_profile: Value) -> Result<File, FileErrors> {

        let file = self.file_repo.create_file(name, seed, min_fill, tolerance, dominance_gap, target_profile).await.map_err(|_| FileErrors::DatabaseError)?;

        self.bin_repo.create_bins(&file.id, None).await.map_err(|_| FileErrors::DatabaseError)?;

        Ok(file)
    }
}