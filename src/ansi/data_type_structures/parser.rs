use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{multispace0, multispace1, u32};
use nom::combinator::{map, opt};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

use crate::ansi::data_type_structures::ast::{
    CharacterLength, CharacterLengthUnits, DataType, ExactNumberInfo, WithOrWithoutTimeZone,
};
use crate::common::parsers::delimited_u32;

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
        character_string,
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
            tuple((tag_no_case("CHARACTER VARYING"), character_length)),
            |(_, opt_len)| DataType::CharacterVarying(opt_len),
        ),
        map(
            tuple((tag_no_case("CHAR VARYING"), character_length)),
            |(_, opt_len)| DataType::CharVarying(opt_len),
        ),
        map(
            tuple((tag_no_case("CHARACTER"), character_length)),
            |(_, opt_len)| DataType::Character(opt_len),
        ),
        map(
            tuple((tag_no_case("VARCHAR"), character_length)),
            |(_, opt_len)| DataType::Varchar(opt_len),
        ),
        map(
            tuple((tag_no_case("CHAR"), character_length)),
            |(_, opt_len)| DataType::Char(opt_len),
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
            opt(preceded(multispace0, delimited_u32)),
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
                tuple((opt(delimited_u32), with_or_without_timezone)),
            ),
            |(precision, tz_info)| DataType::Timestamp(precision, tz_info),
        ),
        map(
            preceded(
                tag_no_case("TIME"),
                tuple((opt(delimited_u32), with_or_without_timezone)),
            ),
            |(precision, tz_info)| DataType::Time(precision, tz_info),
        ),
    ))(i)
}

fn character_length(i: &[u8]) -> IResult<&[u8], Option<CharacterLength>> {
    let characters_mapping = alt((
        map(tag_no_case("CHARACTERS"), |_| {
            CharacterLengthUnits::Characters
        }),
        map(tag_no_case("OCTETS"), |_| CharacterLengthUnits::Octets),
    ));

    let interior = map(
        tuple((
            u32,
            opt(map(
                tuple((multispace1, characters_mapping)),
                |(_, units)| units,
            )),
        )),
        |(length, opt_units)| {
            let mut character_length = CharacterLength::new(length);
            character_length.with_units(opt_units);
            character_length
        },
    );

    opt(delimited(
        tuple((multispace0, tag("("))),
        interior,
        tuple((tag(")"), multispace0)),
    ))(i)
}

fn exact_number_info(i: &[u8]) -> IResult<&[u8], ExactNumberInfo> {
    alt((
        delimited(
            tuple((multispace0, tag("("))),
            map(
                tuple((
                    u32,
                    preceded(tuple((multispace0, tag(","), multispace0)), u32),
                )),
                |(precision, scale)| ExactNumberInfo::PrecisionAndScale(precision, scale),
            ),
            tuple((multispace0, tag(")"))),
        ),
        delimited(
            tuple((multispace0, tag("("))),
            map(u32, ExactNumberInfo::Precision),
            tuple((multispace0, tag(")"))),
        ),
        map(tag(""), |_| ExactNumberInfo::None),
    ))(i)
}

fn with_or_without_timezone(i: &[u8]) -> IResult<&[u8], WithOrWithoutTimeZone> {
    alt((
        map(
            tuple((multispace1, tag_no_case("WITHOUT TIME ZONE"))),
            |_| WithOrWithoutTimeZone::WithoutTimeZone,
        ),
        map(tuple((multispace1, tag_no_case("WITH TIME ZONE"))), |_| {
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
    fn parse_character_string() {
        assert_expected_data_type!("CHARACTER VARYING", DataType::CharacterVarying(None));

        assert_expected_data_type!(
            "CHARACTER VARYING(20)",
            DataType::CharacterVarying(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected_data_type!(
            "CHARACTER VARYING(20 OCTETS)",
            DataType::CharacterVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected_data_type!(
            "CHARACTER VARYING(20 CHARACTERS)",
            DataType::CharacterVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected_data_type!("CHAR VARYING", DataType::CharVarying(None));

        assert_expected_data_type!(
            "CHAR VARYING(20)",
            DataType::CharVarying(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected_data_type!(
            "CHAR VARYING(20 OCTETS)",
            DataType::CharVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected_data_type!(
            "CHAR VARYING(20 CHARACTERS)",
            DataType::CharVarying(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected_data_type!("CHARACTER", DataType::Character(None));

        assert_expected_data_type!(
            "CHARACTER(20)",
            DataType::Character(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected_data_type!(
            "CHARACTER(20 OCTETS)",
            DataType::Character(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected_data_type!(
            "CHARACTER(20 CHARACTERS)",
            DataType::Character(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected_data_type!("VARCHAR", DataType::Varchar(None));

        assert_expected_data_type!(
            "VARCHAR(20)",
            DataType::Varchar(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected_data_type!(
            "VARCHAR(20 OCTETS)",
            DataType::Varchar(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected_data_type!(
            "VARCHAR(20 CHARACTERS)",
            DataType::Varchar(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );

        assert_expected_data_type!("CHAR", DataType::Char(None));

        assert_expected_data_type!(
            "CHAR(20)",
            DataType::Char(Some(*CharacterLength::new(20).with_units(None)))
        );

        assert_expected_data_type!(
            "CHAR(20 OCTETS)",
            DataType::Char(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Octets))
            ))
        );

        assert_expected_data_type!(
            "CHAR(20 CHARACTERS)",
            DataType::Char(Some(
                *CharacterLength::new(20).with_units(Some(CharacterLengthUnits::Characters))
            ))
        );
    }

    #[test]
    fn parse_exact_numeric_type() {
        assert_expected_data_type!("NUMERIC", DataType::Numeric(ExactNumberInfo::None));
        assert_expected_data_type!(
            "NUMERIC(20)",
            DataType::Numeric(ExactNumberInfo::Precision(20))
        );
        assert_expected_data_type!(
            "NUMERIC(30, 2)",
            DataType::Numeric(ExactNumberInfo::PrecisionAndScale(30, 2))
        );
        assert_expected_data_type!("DECIMAL", DataType::Decimal(ExactNumberInfo::None));
        assert_expected_data_type!(
            "DECIMAL(20)",
            DataType::Decimal(ExactNumberInfo::Precision(20))
        );
        assert_expected_data_type!(
            "DECIMAL(30, 2)",
            DataType::Decimal(ExactNumberInfo::PrecisionAndScale(30, 2))
        );
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
    fn parse_approximate_numeric_type() {
        assert_expected_data_type!("FLOAT", DataType::Float);
        assert_expected_data_type!("REAL", DataType::Real);
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
    fn parse_datetime() {
        assert_expected_data_type!("DATE", DataType::Date);

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
