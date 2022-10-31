use std::fmt::{Display, Formatter};

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::map;
use nom::IResult;

/// Data type for ANSI dialect [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_6_1_data_type
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum Ansi {
    /// Boolean data type [(1)].
    ///
    /// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#boolean-type
    Boolean,
}

impl Display for Ansi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean => {
                write!(f, "BOOLEAN")
            }
        }
    }
}

/// Parses the Ansi data type [(1)], returning it and the remaining string.
///
/// # Errors
/// This method will fail if there's no matching data type to the input.
///
/// [(1)]: Ansi
pub fn data_type(input: &str) -> IResult<&str, Ansi> {
    alt((map(tag_no_case("boolean"), |_| Ansi::Boolean),))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_parse_data_type {
        ( $input:expr, $expected:expr ) => {{
            let (remaining, parsed) = data_type($input).unwrap();
            assert!(remaining.is_empty());
            assert_eq!($expected, parsed);
            assert_eq!($input, parsed.to_string());
        }};
    }

    #[test]
    fn test_parse_boolean() {
        test_parse_data_type!("BOOLEAN", Ansi::Boolean);
    }
}
