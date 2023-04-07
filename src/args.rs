pub struct Args;

pub enum ArgsError {
    CannotGetArgument(usize, String),
}

impl Into<String> for ArgsError {
    fn into(self) -> String {
        match self {
            ArgsError::CannotGetArgument(n, x) => format!("We expect argument {n} to be {x}, but we didn't get this argument from command line. "),
        }
    }
}

impl Args {
    pub fn new() -> Self {
        Args
    }
    pub fn get(&self, n: usize, x: String) -> Result<String, ArgsError> {
        let s = std::env::args().nth(n);
        match s {
            Some(s) => Ok(s),
            None => Err(ArgsError::CannotGetArgument(n, x)),
        }
    }
}
