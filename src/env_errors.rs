#[derive(PartialEq, Debug)]
pub enum EnvEnumResult<T: Clone> {
    Ok(T),
    Absent(String),
    IncorrectCast(String)
}

impl<T: Clone> EnvEnumResult<T> {
    pub fn panic_if_absent(&self) -> T {
        match self {
            EnvEnumResult::Ok(val) => val.clone(),
            EnvEnumResult::Absent(message) => panic!("{}", message),
            EnvEnumResult::IncorrectCast(message) => panic!("{}", message),
        }
    }
}