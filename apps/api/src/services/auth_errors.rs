#[derive(Debug)]
pub enum SignupError {
    EmailAlreadyExists,
    HashingFailed,
    DatabaseError,
}