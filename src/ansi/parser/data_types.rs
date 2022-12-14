use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::u32;
use nom::combinator::{map, opt};
use nom::sequence::{pair, preceded, separated_pair, tuple};
use nom::IResult;

use crate::ansi::ast::data_types::{
    CharLengthUnits, CharacterLargeObjectLength, CharacterLength, DataType, ExactNumberInfo,
    LargeObjectLength, Multiplier, WithOrWithoutTimeZone,
};
use crate::common::parsers::{
    delimited_ws0, paren_delimited, preceded_ws0, preceded_ws1, terminated_ws0,
};
use crate::common::tokens::comma;

/// Parses `ANSI` data type [(1)].
///
/// # Errors
/// This function returns an error if the data type is not supported or not
/// exists in the current dialect.
///
/// [(1)]: crate::ansi::DataType
pub fn data_type(input: &[u8]) -> IResult<&[u8], DataType> {
    // OBS: the order matters to parse data types. Do not change it.
    alt((
        character_large_object_types,
        character_string,
        binary_string_types,
        decimal_floating_point_type,
        exact_numeric_type,
        approximate_numeric_type,
        boolean_type,
        datetime_type,
    ))(input)
}

fn character_string(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(
            preceded(
                terminated_ws0(tag_no_case("CHARACTER VARYING")),
                opt_character_length,
            ),
            DataType::CharacterVarying,
        ),
        map(
            preceded(
                terminated_ws0(tag_no_case("CHAR VARYING")),
                opt_character_length,
            ),
            DataType::CharVarying,
        ),
        map(
            preceded(
                terminated_ws0(tag_no_case("CHARACTER")),
                opt_character_length,
            ),
            DataType::Character,
        ),
        map(
            preceded(terminated_ws0(tag_no_case("VARCHAR")), opt_character_length),
            DataType::Varchar,
        ),
        map(
            preceded(terminated_ws0(tag_no_case("CHAR")), opt_character_length),
            DataType::Char,
        ),
    ))(input)
}

fn character_large_object_types(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(
            preceded(
                tag_no_case("CHARACTER LARGE OBJECT"),
                opt(paren_delimited(character_large_object_length)),
            ),
            DataType::CharacterLargeObject,
        ),
        map(
            preceded(
                tag_no_case("CHAR LARGE OBJECT"),
                opt(paren_delimited(character_large_object_length)),
            ),
            DataType::CharLargeObject,
        ),
        map(
            preceded(
                tag_no_case("CLOB"),
                opt(paren_delimited(character_large_object_length)),
            ),
            DataType::Clob,
        ),
    ))(input)
}

fn binary_string_types(input: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(
            preceded(
                tag_no_case("BINARY LARGE OBJECT"),
                opt(preceded_ws0(paren_delimited(large_object_length))),
            ),
            DataType::BinaryLargeObject,
        ),
        map(
            preceded(
                tag_no_case("BLOB"),
                opt(preceded_ws0(paren_delimited(large_object_length))),
            ),
            DataType::Blob,
        ),
        map(
            preceded(
                tag_no_case("VARBINARY"),
                opt(preceded_ws0(paren_delimited(u32))),
            ),
            DataType::Varbinary,
        ),
        map(
            preceded(
                tag_no_case("BINARY VARYING"),
                opt(preceded_ws0(paren_delimited(u32))),
            ),
            DataType::BinaryVarying,
        ),
        map(
            preceded(
                tag_no_case("BINARY"),
                opt(preceded_ws0(paren_delimited(u32))),
            ),
            DataType::Binary,
        ),
    ))(input)
}

fn exact_numeric_type(i: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(
            preceded(tag_no_case("DECIMAL"), exact_number_info),
            DataType::Decimal,
        ),
        map(
            preceded(tag_no_case("NUMERIC"), exact_number_info),
            DataType::Numeric,
        ),
        map(
            preceded(tag_no_case("DEC"), exact_number_info),
            DataType::Dec,
        ),
        map(tag_no_case("SMALLINT"), |_| DataType::Smallint),
        map(tag_no_case("INTEGER"), |_| DataType::Integer),
        map(tag_no_case("BIGINT"), |_| DataType::Bigint),
        map(tag_no_case("INT"), |_| DataType::Int),
    ))(i)
}

fn approximate_numeric_type(i: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(tag_no_case("FLOAT"), |_| DataType::Float),
        map(tag_no_case("REAL"), |_| DataType::Real),
        map(tag_no_case("DOUBLE PRECISION"), |_| {
            DataType::DoublePrecision
        }),
    ))(i)
}

fn decimal_floating_point_type(i: &[u8]) -> IResult<&[u8], DataType> {
    map(
        preceded(
            tag_no_case("DECFLOAT"),
            opt(preceded_ws0(paren_delimited(u32))),
        ),
        DataType::DecFloat,
    )(i)
}

fn boolean_type(i: &[u8]) -> IResult<&[u8], DataType> {
    map(tag_no_case("BOOLEAN"), |_| DataType::Boolean)(i)
}

fn datetime_type(i: &[u8]) -> IResult<&[u8], DataType> {
    alt((
        map(tag_no_case("DATE"), |_| DataType::Date),
        map(
            preceded(
                tag_no_case("TIMESTAMP"),
                tuple((opt(paren_delimited(u32)), with_or_without_timezone)),
            ),
            |(precision, tz_info)| DataType::Timestamp(precision, tz_info),
        ),
        map(
            preceded(
                tag_no_case("TIME"),
                tuple((opt(paren_delimited(u32)), with_or_without_timezone)),
            ),
            |(precision, tz_info)| DataType::Time(precision, tz_info),
        ),
    ))(i)
}

fn opt_character_length(i: &[u8]) -> IResult<&[u8], Option<CharacterLength>> {
    map(
        opt(paren_delimited(pair(
            u32,
            opt(preceded_ws1(char_length_units)),
        ))),
        |opt_character_length| {
            if let Some((length, opt_units)) = opt_character_length {
                Some(*CharacterLength::new(length).with_opt_units(opt_units))
            } else {
                None
            }
        },
    )(i)
}

fn character_large_object_length(i: &[u8]) -> IResult<&[u8], CharacterLargeObjectLength> {
    let (i, (length, opt_units)) =
        tuple((large_object_length, opt(preceded_ws1(char_length_units))))(i)?;

    let mut character_length = CharacterLargeObjectLength::new(length);
    if let Some(units) = opt_units {
        character_length.with_units(units);
    }

    Ok((i, character_length))
}

fn large_object_length(i: &[u8]) -> IResult<&[u8], LargeObjectLength> {
    let (i, (length, opt_multiplier)) = pair(u32, opt(multiplier))(i)?;

    let mut large_object_length = LargeObjectLength::new(length);
    if let Some(multiplier) = opt_multiplier {
        large_object_length.with_multiplier(multiplier);
    }

    Ok((i, large_object_length))
}

fn multiplier(i: &[u8]) -> IResult<&[u8], Multiplier> {
    alt((
        map(tag_no_case("K"), |_| Multiplier::K),
        map(tag_no_case("M"), |_| Multiplier::M),
        map(tag_no_case("G"), |_| Multiplier::G),
        map(tag_no_case("T"), |_| Multiplier::T),
        map(tag_no_case("P"), |_| Multiplier::P),
    ))(i)
}

fn char_length_units(i: &[u8]) -> IResult<&[u8], CharLengthUnits> {
    alt((
        map(tag_no_case("OCTETS"), |_| CharLengthUnits::Octets),
        map(tag_no_case("CHARACTERS"), |_| CharLengthUnits::Characters),
    ))(i)
}

fn exact_number_info(i: &[u8]) -> IResult<&[u8], ExactNumberInfo> {
    alt((
        map(
            paren_delimited(separated_pair(u32, delimited_ws0(comma), u32)),
            |(precision, scale)| ExactNumberInfo::PrecisionAndScale(precision, scale),
        ),
        map(paren_delimited(u32), ExactNumberInfo::Precision),
        map(tag(""), |_| ExactNumberInfo::None),
    ))(i)
}

fn with_or_without_timezone(i: &[u8]) -> IResult<&[u8], WithOrWithoutTimeZone> {
    alt((
        map(preceded_ws1(tag_no_case("WITHOUT TIME ZONE")), |_| {
            WithOrWithoutTimeZone::WithoutTimeZone
        }),
        map(preceded_ws1(tag_no_case("WITH TIME ZONE")), |_| {
            WithOrWithoutTimeZone::WithTimeZone
        }),
        map(tag(""), |_| WithOrWithoutTimeZone::None),
    ))(i)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_str_eq;

    use super::*;

    macro_rules! assert_expected_data_type {
        ($input:expr, $expected:expr) => {{
            let (remaining, parsed) = data_type($input.as_ref()).unwrap();
            assert_eq!($expected, parsed);
            assert_str_eq!($input, parsed.to_string());
            assert!(remaining.is_empty());
        }};
    }

    #[test]
    fn parse_character_varying() {
        assert_expected_data_type!("CHARACTER VARYING", DataType::CharacterVarying(None));

        assert_expected_data_type!(
            "CHARACTER VARYING(20)",
            DataType::CharacterVarying(Some(CharacterLength::new(20)))
        );

        assert_expected_data_type!(
            "CHARACTER VARYING(20 OCTETS)",
            DataType::CharacterVarying(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Octets)
            ))
        );

        assert_expected_data_type!(
            "CHARACTER VARYING(20 CHARACTERS)",
            DataType::CharacterVarying(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_char_varying() {
        assert_expected_data_type!("CHAR VARYING", DataType::CharVarying(None));

        assert_expected_data_type!(
            "CHAR VARYING(20)",
            DataType::CharVarying(Some(CharacterLength::new(20)))
        );

        assert_expected_data_type!(
            "CHAR VARYING(20 OCTETS)",
            DataType::CharVarying(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Octets)
            ))
        );

        assert_expected_data_type!(
            "CHAR VARYING(20 CHARACTERS)",
            DataType::CharVarying(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_character() {
        assert_expected_data_type!("CHARACTER", DataType::Character(None));

        assert_expected_data_type!(
            "CHARACTER(20)",
            DataType::Character(Some(CharacterLength::new(20)))
        );

        assert_expected_data_type!(
            "CHARACTER(20 OCTETS)",
            DataType::Character(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Octets)
            ))
        );

        assert_expected_data_type!(
            "CHARACTER(20 CHARACTERS)",
            DataType::Character(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_varchar() {
        assert_expected_data_type!("VARCHAR", DataType::Varchar(None));

        assert_expected_data_type!(
            "VARCHAR(20)",
            DataType::Varchar(Some(CharacterLength::new(20)))
        );

        assert_expected_data_type!(
            "VARCHAR(20 OCTETS)",
            DataType::Varchar(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Octets)
            ))
        );

        assert_expected_data_type!(
            "VARCHAR(20 CHARACTERS)",
            DataType::Varchar(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_char() {
        assert_expected_data_type!("CHAR", DataType::Char(None));

        assert_expected_data_type!("CHAR(20)", DataType::Char(Some(CharacterLength::new(20))));

        assert_expected_data_type!(
            "CHAR(20 OCTETS)",
            DataType::Char(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Octets)
            ))
        );

        assert_expected_data_type!(
            "CHAR(20 CHARACTERS)",
            DataType::Char(Some(
                *CharacterLength::new(20).with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_character_large_object() {
        assert_expected_data_type!(
            "CHARACTER LARGE OBJECT",
            DataType::CharacterLargeObject(None)
        );

        assert_expected_data_type!(
            "CHARACTER LARGE OBJECT(20)",
            DataType::CharacterLargeObject(Some(CharacterLargeObjectLength::new(
                LargeObjectLength::new(20)
            )))
        );

        assert_expected_data_type!(
            "CHARACTER LARGE OBJECT(20 CHARACTERS)",
            DataType::CharacterLargeObject(Some(
                *CharacterLargeObjectLength::new(LargeObjectLength::new(20))
                    .with_units(CharLengthUnits::Characters)
            ))
        );

        assert_expected_data_type!(
            "CHARACTER LARGE OBJECT(20K)",
            DataType::CharacterLargeObject(Some(CharacterLargeObjectLength::new(
                *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
            )))
        );

        assert_expected_data_type!(
            "CHARACTER LARGE OBJECT(20K CHARACTERS)",
            DataType::CharacterLargeObject(Some(
                *CharacterLargeObjectLength::new(
                    *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
                )
                .with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_character_large_object_types_char_large_object() {
        assert_expected_data_type!("CHAR LARGE OBJECT", DataType::CharLargeObject(None));

        assert_expected_data_type!(
            "CHAR LARGE OBJECT(20)",
            DataType::CharLargeObject(Some(CharacterLargeObjectLength::new(
                LargeObjectLength::new(20)
            )))
        );

        assert_expected_data_type!(
            "CHAR LARGE OBJECT(20 CHARACTERS)",
            DataType::CharLargeObject(Some(
                *CharacterLargeObjectLength::new(LargeObjectLength::new(20))
                    .with_units(CharLengthUnits::Characters)
            ))
        );

        assert_expected_data_type!(
            "CHAR LARGE OBJECT(20K)",
            DataType::CharLargeObject(Some(CharacterLargeObjectLength::new(
                *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
            )))
        );

        assert_expected_data_type!(
            "CHAR LARGE OBJECT(20K CHARACTERS)",
            DataType::CharLargeObject(Some(
                *CharacterLargeObjectLength::new(
                    *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
                )
                .with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_character_large_object_types_clob() {
        assert_expected_data_type!("CLOB", DataType::Clob(None));

        assert_expected_data_type!(
            "CLOB(20)",
            DataType::Clob(Some(CharacterLargeObjectLength::new(
                LargeObjectLength::new(20)
            )))
        );

        assert_expected_data_type!(
            "CLOB(20 CHARACTERS)",
            DataType::Clob(Some(
                *CharacterLargeObjectLength::new(LargeObjectLength::new(20))
                    .with_units(CharLengthUnits::Characters)
            ))
        );

        assert_expected_data_type!(
            "CLOB(20K)",
            DataType::Clob(Some(CharacterLargeObjectLength::new(
                *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
            )))
        );

        assert_expected_data_type!(
            "CLOB(20K CHARACTERS)",
            DataType::Clob(Some(
                *CharacterLargeObjectLength::new(
                    *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
                )
                .with_units(CharLengthUnits::Characters)
            ))
        );
    }

    #[test]
    fn parse_binary() {
        assert_expected_data_type!("BINARY", DataType::Binary(None));
        assert_expected_data_type!("BINARY(20)", DataType::Binary(Some(20)));
    }

    #[test]
    fn parse_binary_varying() {
        assert_expected_data_type!("BINARY VARYING", DataType::BinaryVarying(None));
        assert_expected_data_type!("BINARY VARYING(20)", DataType::BinaryVarying(Some(20)));
    }

    #[test]
    fn parse_varbinary() {
        assert_expected_data_type!("VARBINARY", DataType::Varbinary(None));
        assert_expected_data_type!("VARBINARY(20)", DataType::Varbinary(Some(20)));
    }

    #[test]
    fn parse_binary_large_object() {
        assert_expected_data_type!("BINARY LARGE OBJECT", DataType::BinaryLargeObject(None));
        assert_expected_data_type!(
            "BINARY LARGE OBJECT(20)",
            DataType::BinaryLargeObject(Some(LargeObjectLength::new(20)))
        );
        assert_expected_data_type!(
            "BINARY LARGE OBJECT(20K)",
            DataType::BinaryLargeObject(Some(
                *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
            ))
        );
    }

    #[test]
    fn parse_blob() {
        assert_expected_data_type!("BLOB", DataType::Blob(None));
        assert_expected_data_type!("BLOB(20)", DataType::Blob(Some(LargeObjectLength::new(20))));
        assert_expected_data_type!(
            "BLOB(20K)",
            DataType::Blob(Some(
                *LargeObjectLength::new(20).with_multiplier(Multiplier::K)
            ))
        );
    }

    #[test]
    fn parse_numeric() {
        assert_expected_data_type!("NUMERIC", DataType::Numeric(ExactNumberInfo::None));
        assert_expected_data_type!(
            "NUMERIC(20)",
            DataType::Numeric(ExactNumberInfo::Precision(20))
        );
        assert_expected_data_type!(
            "NUMERIC(30, 2)",
            DataType::Numeric(ExactNumberInfo::PrecisionAndScale(30, 2))
        );
    }

    #[test]
    fn parse_decimal() {
        assert_expected_data_type!("DECIMAL", DataType::Decimal(ExactNumberInfo::None));
        assert_expected_data_type!(
            "DECIMAL(20)",
            DataType::Decimal(ExactNumberInfo::Precision(20))
        );
        assert_expected_data_type!(
            "DECIMAL(30, 2)",
            DataType::Decimal(ExactNumberInfo::PrecisionAndScale(30, 2))
        );
    }

    #[test]
    fn parse_dec() {
        assert_expected_data_type!("DEC", DataType::Dec(ExactNumberInfo::None));
        assert_expected_data_type!("DEC(20)", DataType::Dec(ExactNumberInfo::Precision(20)));
        assert_expected_data_type!(
            "DEC(30, 2)",
            DataType::Dec(ExactNumberInfo::PrecisionAndScale(30, 2))
        );
        assert_expected_data_type!("SMALLINT", DataType::Smallint);
        assert_expected_data_type!("INTEGER", DataType::Integer);
        assert_expected_data_type!("INT", DataType::Int);
        assert_expected_data_type!("BIGINT", DataType::Bigint);
    }

    #[test]
    fn parse_smallint() {
        assert_expected_data_type!("SMALLINT", DataType::Smallint);
    }

    #[test]
    fn parse_integer() {
        assert_expected_data_type!("INTEGER", DataType::Integer);
        assert_expected_data_type!("INT", DataType::Int);
        assert_expected_data_type!("BIGINT", DataType::Bigint);
    }

    #[test]
    fn parse_int() {
        assert_expected_data_type!("INT", DataType::Int);
        assert_expected_data_type!("BIGINT", DataType::Bigint);
    }

    #[test]
    fn parse_bigint() {
        assert_expected_data_type!("BIGINT", DataType::Bigint);
    }

    #[test]
    fn parse_float() {
        assert_expected_data_type!("FLOAT", DataType::Float);
    }

    #[test]
    fn parse_real() {
        assert_expected_data_type!("REAL", DataType::Real);
    }

    #[test]
    fn parse_double_precision() {
        assert_expected_data_type!("DOUBLE PRECISION", DataType::DoublePrecision);
    }

    #[test]
    fn parse_decimal_floating_point_type() {
        assert_expected_data_type!("DECFLOAT", DataType::DecFloat(None));
        assert_expected_data_type!("DECFLOAT(120)", DataType::DecFloat(Some(120)));
    }

    #[test]
    fn parse_boolean_type() {
        assert_expected_data_type!("BOOLEAN", DataType::Boolean);
    }

    #[test]
    fn parse_date() {
        assert_expected_data_type!("DATE", DataType::Date);
    }

    #[test]
    fn parse_time() {
        assert_expected_data_type!("TIME", DataType::Time(None, WithOrWithoutTimeZone::None));

        assert_expected_data_type!(
            "TIME WITH TIME ZONE",
            DataType::Time(None, WithOrWithoutTimeZone::WithTimeZone)
        );

        assert_expected_data_type!(
            "TIME WITHOUT TIME ZONE",
            DataType::Time(None, WithOrWithoutTimeZone::WithoutTimeZone)
        );

        assert_expected_data_type!(
            "TIME(20)",
            DataType::Time(Some(20), WithOrWithoutTimeZone::None)
        );

        assert_expected_data_type!(
            "TIME(20) WITH TIME ZONE",
            DataType::Time(Some(20), WithOrWithoutTimeZone::WithTimeZone)
        );

        assert_expected_data_type!(
            "TIME(20) WITHOUT TIME ZONE",
            DataType::Time(Some(20), WithOrWithoutTimeZone::WithoutTimeZone)
        );
    }

    #[test]
    fn parse_timestamp() {
        assert_expected_data_type!(
            "TIMESTAMP",
            DataType::Timestamp(None, WithOrWithoutTimeZone::None)
        );

        assert_expected_data_type!(
            "TIMESTAMP WITH TIME ZONE",
            DataType::Timestamp(None, WithOrWithoutTimeZone::WithTimeZone)
        );

        assert_expected_data_type!(
            "TIMESTAMP",
            DataType::Timestamp(None, WithOrWithoutTimeZone::None)
        );

        assert_expected_data_type!(
            "TIMESTAMP(20)",
            DataType::Timestamp(Some(20), WithOrWithoutTimeZone::None)
        );

        assert_expected_data_type!(
            "TIMESTAMP(20) WITH TIME ZONE",
            DataType::Timestamp(Some(20), WithOrWithoutTimeZone::WithTimeZone)
        );

        assert_expected_data_type!(
            "TIMESTAMP(20) WITHOUT TIME ZONE",
            DataType::Timestamp(Some(20), WithOrWithoutTimeZone::WithoutTimeZone)
        );
    }
}
