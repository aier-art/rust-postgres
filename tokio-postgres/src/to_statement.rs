use crate::to_statement::private::{Sealed, ToStatementType};
use crate::Statement;

pub(crate) mod private {
    use crate::{Client, Error, Statement};

    pub trait Sealed {}

    /// enum Statement or Query
    pub enum ToStatementType<'a> {
        /// statements
        Statement(&'a Statement),
        /// query
        Query(&'a str),
    }

    impl<'a> ToStatementType<'a> {
        /// into_statement
        pub async fn into_statement(self, client: &Client) -> Result<Statement, Error> {
            match self {
                ToStatementType::Statement(s) => Ok(s.clone()),
                ToStatementType::Query(s) => client.prepare(s).await,
            }
        }
    }
}

/// A trait abstracting over prepared and unprepared statements.
///
/// Many methods are generic over this bound, so that they support both a raw query string as well as a statement which
/// was prepared previously.
///
/// This trait is "sealed" and cannot be implemented by anything outside this crate.
pub trait ToStatement {
    /// A trait abstracting over prepared and unprepared statements.
    fn __convert(&self) -> ToStatementType<'_>;
}

impl ToStatement for Statement {
    fn __convert(&self) -> ToStatementType<'_> {
        ToStatementType::Statement(self)
    }
}

impl Sealed for Statement {}

impl<T: AsRef<str>> ToStatement for T {
    fn __convert(&self) -> ToStatementType<'_> {
        ToStatementType::Query(self.as_ref())
    }
}

impl Sealed for str {}

impl Sealed for String {}
