use std::fmt;

#[derive(Copy,Clone,Debug)]
pub enum Error {
    ConvertString,
    CreateFile,
    DeleteFile,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", &self)
    }
}

#[derive(Copy,Clone,Debug)]
pub enum CreateMode {
    CreateOnly,
    OpenOrCreate,
    OpenOnly,
}

#[derive(Copy,Clone,Debug)]
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

#[derive(Copy,Clone,Debug)]
pub enum Permissions {
    User,
    Group,
    Everybody
}

#[cfg(test)]
mod tests {
    #[test]
    fn breath_test_error() {
        use super::*;
        println!("{}", Error::CreateFile);
        println!("{:?}", Error::CreateFile);
        println!("{:#?}", Error::CreateFile);
    }
}
