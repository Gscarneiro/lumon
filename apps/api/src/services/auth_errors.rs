#[derive(Debug)]
pub enum AuthErrors {
    EmailAlreadyExists,
    InvalidCredentials,
    HashingFailed,
    DatabaseError,
}