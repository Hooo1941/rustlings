#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // if value < 0 {
        //     Err(CreationError::Negative)
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Ok(PositiveNonzeroInteger(value as u64))
        // }
        match value {
            v if v < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            v => Ok(PositiveNonzeroInteger(v as u64)),
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        assert_eq!(
            PositiveNonzeroInteger::new(10),
            Ok(PositiveNonzeroInteger(10)),
        );
        assert_eq!(
            PositiveNonzeroInteger::new(-10),
            Err(CreationError::Negative),
        );
        assert_eq!(PositiveNonzeroInteger::new(0), Err(CreationError::Zero));
    }
}