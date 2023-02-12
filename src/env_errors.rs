/// # Environment Enum Result
/// Anything that isn't in the Ok status is an error.
#[derive(PartialEq, Debug)]
pub enum EnvEnumResult<T: Clone> {
    Ok(T),
    Absent(String),
    IncorrectCast(String)
}

impl<T: Clone> EnvEnumResult<T> {
    /// Will panic for everything that isn't Ok
    /// ```
    /// use dotenv_enum::env_errors::EnvEnumResult;
    ///
    /// assert_eq!(EnvEnumResult::Ok(4).panic_if_absent(), 4)
    /// ```
    pub fn panic_if_absent(&self) -> T {
        match self {
            EnvEnumResult::Ok(val) => val.clone(),
            EnvEnumResult::Absent(message) => panic!("{}", message),
            EnvEnumResult::IncorrectCast(message) => panic!("{}", message),
        }
    }
}