use sqlx::Error;
#[derive(Debug, Clone, PartialEq)]
pub enum QueryError {
    NoResultFound,
    WrongQueryFormat,
    PoolProblem,
    DataBaseError,
    DecodingProblem,
    ConnectionProblem,
    WrongType(String),
    ColumnIndexOutOfBounds(usize, usize),
    OtherProblem,
}

impl QueryError {
    pub fn match_sqlx_error(err: sqlx::Error) -> Self {
        match err {
            Error::RowNotFound | Error::ColumnNotFound(_) => QueryError::NoResultFound,
            Error::PoolClosed | Error::PoolTimedOut => QueryError::PoolProblem,
            Error::TypeNotFound {type_name} => QueryError::WrongType(type_name),
            Error::ColumnIndexOutOfBounds {index, len} => QueryError::ColumnIndexOutOfBounds(index, len),
            Error::Decode(_) => QueryError::DecodingProblem,
            Error::Tls(_) => QueryError::ConnectionProblem,
            Error::Database(_) | Error::Io(_) | Error::Protocol(_) => QueryError::DataBaseError,
            _ => QueryError::OtherProblem
        }
    }
}