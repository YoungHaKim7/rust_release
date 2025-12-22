trait UsesAssocErrorType {
    type Error;
    fn method(&self) -> Result<(), Self::Error>;
}

struct CannotFail;
impl UsesAssocErrorType for CannotFail {
    type Error = core::convert::Infallible;
    fn method(&self) -> Result<(), Self::Error> {
        Ok(())
    }
}

struct CanFail;
impl UsesAssocErrorType for CanFail {
    type Error = std::io::Error;
    fn method(&self) -> Result<(), Self::Error> {
        Err(std::io::Error::other("something went wrong"))
    }
}

fn main() {
    CannotFail.method(); // No warning
    let _ = CanFail.method(); // Warning: unused `Result` that must be used
}
