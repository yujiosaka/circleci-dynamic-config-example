use std::{error, result};

pub type Error = Box<dyn error::Error + Send + Sync + 'static>;
pub type Result<T> = result::Result<T, Error>;

pub fn add(left: f64, right: f64) -> f64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2., 2.);
        assert_eq!(result, 4.);
    }
}
