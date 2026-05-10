pub enum AuthenticationError {
    // --- Token
    TokenExpired,

    // --- Users
    UserAlreadyExists,
    InvalidCredentials,

    // --- General
    DatabaseError(String),
}
