use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map, opt, peek};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

use crate::ansi::ast::common::{
    ColumnDefinition, ColumnNameList, DeleteRule, DropBehavior, LocalOrSchemaQualifier,
    LocalQualifier, MatchType, ReferencedPeriodSpecification, ReferentialAction,
    ReferentialTriggeredAction, SchemaName, TableName, UpdateRule,
};
use crate::ansi::parser::data_types::data_type;
use crate::common::parsers::ident;
use crate::common::tokens::{comma, period};

/// Parses a schema name [(1)](SchemaName).
///
/// # Errors
/// If the schema name has too many qualifications or invalid structure, this
/// function call will fail.
pub fn schema_name(i: &[u8]) -> IResult<&[u8], SchemaName> {
    alt((
        map(
            tuple((terminated(ident, period), ident)),
            |(catalog, schema)| SchemaName::new(Some(&catalog), &schema),
        ),
        map(ident, |schema| SchemaName::new(None, &schema)),
    ))(i)
}

/// Parses a table name [(1)](TableName).
///
/// # Errors
/// If the table name has too many qualifications or invalid structure, this
/// function call will fail.
pub fn table_name(i: &[u8]) -> IResult<&[u8], TableName> {
    let (i, (opt_local_or_schema, name)) =
        tuple((opt(terminated(local_or_schema_qualifier, period)), ident))(i)?;

    let mut table_name = TableName::new(&name);
    if let Some(local_or_schema) = opt_local_or_schema {
        table_name.with_local_or_schema(local_or_schema);
    }

    Ok((i, table_name))
}

/// Parses a local or schema qualifier [(1)](LocalOrSchemaQualifier).
///
/// # Errors
/// If the received input is malformed, this function call will fail.
pub fn local_or_schema_qualifier(i: &[u8]) -> IResult<&[u8], LocalOrSchemaQualifier> {
    alt((
        map(local_qualifier, LocalOrSchemaQualifier::LocalQualifier),
        map(
            schema_for_qualified_table_name,
            LocalOrSchemaQualifier::Schema,
        ),
    ))(i)
}

/// Parses a schema name considering that the current identifier must have a
/// table name [(1)](SchemaName).
///
/// This function considers that, if 4 identifiers are parsed (e.g.,
/// `name_1.name_2.name_3.name_2`), 3 of them are going to be returned, and the
/// last one (".`name_4`") is going to be returned unchanged.
///
/// # Errors
/// If the received input is malformed, this function call will fail.
pub fn schema_for_qualified_table_name(i: &[u8]) -> IResult<&[u8], SchemaName> {
    alt((
        map(
            terminated(
                tuple((terminated(ident, period), ident)),
                peek(tuple((period, ident))),
            ),
            |(catalog, schema)| SchemaName::new(Some(&catalog), &schema),
        ),
        map(terminated(ident, peek(tuple((period, ident)))), |schema| {
            SchemaName::new(None, &schema)
        }),
    ))(i)
}

/// Parses a local qualifier [(1)](LocalQualifier).
///
/// # Errors
/// If the input does not match a case-insensitive `MODULE` word, this function
/// call will fail.
pub fn local_qualifier(i: &[u8]) -> IResult<&[u8], LocalQualifier> {
    map(tag_no_case("MODULE"), |_| LocalQualifier::Module)(i)
}

/// Parses a column definition [(1)](ColumnDefinition).
///
/// # Errors
/// If the column definition has unsupported syntax or invalid, this function
/// call will fail. Check the described syntax on column definition structure to
/// understand the supported syntax.
pub fn column_definition(i: &[u8]) -> IResult<&[u8], ColumnDefinition> {
    let (i, (column_name, opt_data_type)) =
        tuple((ident, opt(preceded(multispace1, data_type))))(i)?;

    let mut column_def = ColumnDefinition::new(&column_name);

    if let Some(data_type) = opt_data_type {
        column_def.with_data_type(data_type);
    }

    Ok((i, column_def))
}

/// Parses the drop behavior [(1)](DropBehavior).
///
/// # Errors
/// If the received input do not match a case-insensitive one of `RECEIVED` or
/// `CASCADE` keywords, this function call will fail.
pub fn drop_behavior(i: &[u8]) -> IResult<&[u8], DropBehavior> {
    alt((
        map(tag_no_case("CASCADE"), |_| DropBehavior::Cascade),
        map(tag_no_case("RESTRICT"), |_| DropBehavior::Restrict),
    ))(i)
}

/// Parses a referential action [(1)](ReferentialAction).
///
/// # Errors
/// If the received input do not match a case-insensitive variant of the
/// referential action enum, this function will return an error.
pub fn referential_action(i: &[u8]) -> IResult<&[u8], ReferentialAction> {
    alt((
        map(tag_no_case("CASCADE"), |_| ReferentialAction::Cascade),
        map(tag_no_case("SET NULL"), |_| ReferentialAction::SetNull),
        map(tag_no_case("SET DEFAULT"), |_| {
            ReferentialAction::SetDefault
        }),
        map(tag_no_case("RESTRICT"), |_| ReferentialAction::Restrict),
        map(tag_no_case("NO ACTION"), |_| ReferentialAction::NoAction),
    ))(i)
}

/// Parses a delete rule [(1)](DeleteRule).
///
/// # Errors
/// If the received input do not match the syntax of a delete rule, or the
/// internal referential action is invalid, this function call will return an
/// error.
pub fn delete_rule(i: &[u8]) -> IResult<&[u8], DeleteRule> {
    map(
        preceded(
            tuple((tag_no_case("ON DELETE"), multispace1)),
            referential_action,
        ),
        DeleteRule::new,
    )(i)
}

/// Parses a delete rule [(1)](UpdateRule).
///
/// # Errors
/// If the received input do not match the syntax of a update rule, or the
/// internal referential action is invalid, this function call will return an
/// error.
pub fn update_rule(i: &[u8]) -> IResult<&[u8], UpdateRule> {
    map(
        preceded(
            tuple((tag_no_case("ON UPDATE"), multispace1)),
            referential_action,
        ),
        UpdateRule::new,
    )(i)
}

/// Parses a referential triggered action [(1)](ReferentialTriggeredAction).
///
/// # Errors
/// If the input does not match any of the two possible syntaxes of the
/// referential triggered action, this function call will return an error.
pub fn referential_triggered_action(i: &[u8]) -> IResult<&[u8], ReferentialTriggeredAction> {
    alt((
        map(
            tuple((update_rule, opt(preceded(multispace1, delete_rule)))),
            |(update, opt_delete)| ReferentialTriggeredAction::UpdateFirst(update, opt_delete),
        ),
        map(
            tuple((delete_rule, opt(preceded(multispace1, update_rule)))),
            |(delete, opt_update)| ReferentialTriggeredAction::DeleteFirst(delete, opt_update),
        ),
    ))(i)
}

/// Parses a foreign key match type [(1)](MatchType).
///
/// # Errors
/// If the input does not match any of the three possible match types syntax,
/// this function call will return an error.
pub fn match_type(i: &[u8]) -> IResult<&[u8], MatchType> {
    alt((
        map(tag_no_case("FULL"), |_| MatchType::Full),
        map(tag_no_case("PARTIAL"), |_| MatchType::Partial),
        map(tag_no_case("SIMPLE"), |_| MatchType::Simple),
    ))(i)
}

/// Parses a column name list [(1)](ColumnNameList).
///
/// # Errors
/// If the column list has invalid identifiers, or if there's no columns to be
/// parsed, this function call will return an error.
pub fn column_name_list(i: &[u8]) -> IResult<&[u8], ColumnNameList> {
    map(
        separated_list1(tuple((multispace0, comma, multispace0)), ident),
        |list| ColumnNameList::new(&list),
    )(i)
}

/// Parses a referenced period specification
/// [(1)](ReferencedPeriodSpecification).
///
/// # Errors
/// If the input is not valid for a referenced period specification, this
/// function call will return an error.
pub fn referenced_period_specification(i: &[u8]) -> IResult<&[u8], ReferencedPeriodSpecification> {
    map(
        preceded(tuple((tag_no_case("PERIOD"), multispace1)), ident),
        |period_name| ReferencedPeriodSpecification::new(&period_name),
    )(i)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_str_eq;
    use test_case::test_case;

    use crate::ansi::ast::data_types::DataType;
    use crate::common::Ident;

    use super::*;

    #[test]
    fn parse_column_definition_ast() {
        let input_1 = "name VARCHAR";
        let (_, column_def_1) = column_definition(input_1.as_ref()).unwrap();
        assert_eq!(
            column_def_1,
            *ColumnDefinition::new(&Ident::new(b"name")).with_data_type(DataType::Varchar(None))
        );

        let input_2 = "name";
        let (_, column_def_2) = column_definition(input_2.as_ref()).unwrap();
        assert_eq!(column_def_2, ColumnDefinition::new(&Ident::new(b"name")));
    }

    #[test_case("name")]
    #[test_case("name VARCHAR")]
    fn parse_column_definition_serialisation(input: &str) {
        assert_str_eq!(
            input,
            column_definition(input.as_ref()).unwrap().1.to_string()
        );
    }

    #[test_case("CASCADE")]
    #[test_case("SET NULL")]
    #[test_case("SET DEFAULT")]
    #[test_case("RESTRICT")]
    #[test_case("NO ACTION")]
    fn parse_referential_action(input: &str) {
        assert_str_eq!(
            input,
            referential_action(input.as_ref()).unwrap().1.to_string()
        );
    }

    #[test_case("ON DELETE CASCADE")]
    #[test_case("ON DELETE SET NULL")]
    #[test_case("ON DELETE SET DEFAULT")]
    #[test_case("ON DELETE RESTRICT")]
    #[test_case("ON DELETE NO ACTION")]
    fn parse_delete_rule(input: &str) {
        assert_str_eq!(input, delete_rule(input.as_ref()).unwrap().1.to_string());
    }

    #[test_case("ON UPDATE CASCADE")]
    #[test_case("ON UPDATE SET NULL")]
    #[test_case("ON UPDATE SET DEFAULT")]
    #[test_case("ON UPDATE RESTRICT")]
    #[test_case("ON UPDATE NO ACTION")]
    fn parse_update_rule(input: &str) {
        assert_str_eq!(input, update_rule(input.as_ref()).unwrap().1.to_string());
    }

    #[test_case("ON UPDATE CASCADE")]
    #[test_case("ON DELETE CASCADE")]
    #[test_case("ON UPDATE CASCADE ON DELETE CASCADE")]
    #[test_case("ON DELETE CASCADE ON UPDATE CASCADE")]
    fn parse_referential_triggered_action(input: &str) {
        assert_str_eq!(
            input,
            referential_triggered_action(input.as_ref())
                .unwrap()
                .1
                .to_string()
        );
    }

    #[test_case("FULL")]
    #[test_case("PARTIAL")]
    #[test_case("SIMPLE")]
    fn parse_match_type(input: &str) {
        assert_str_eq!(input, match_type(input.as_ref()).unwrap().1.to_string());
    }

    #[test_case("name")]
    #[test_case("name_1, name_2")]
    #[test_case("name_1, name_2, name_3, name_4")]
    fn parse_column_name_list(input: &str) {
        assert_str_eq!(
            input,
            column_name_list(input.as_ref()).unwrap().1.to_string()
        );
    }

    #[test]
    #[should_panic]
    fn parse_empty_column_name_list() {
        column_name_list(b"").unwrap();
    }

    #[test]
    fn parse_referenced_period_specification() {
        let input = "PERIOD name_1";
        assert_str_eq!(
            input,
            referenced_period_specification(input.as_ref())
                .unwrap()
                .1
                .to_string()
        );
    }
}
