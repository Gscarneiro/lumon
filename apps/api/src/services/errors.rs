#[derive(Debug)]
pub enum AuthErrors {
    EmailAlreadyExists,
    InvalidCredentials,
    HashingFailed,
    DatabaseError,
}

#[derive(Debug)]
pub enum FileErrors {
    NameAlreadyExists,
    DatabaseError,
}