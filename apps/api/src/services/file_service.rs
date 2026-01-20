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
}