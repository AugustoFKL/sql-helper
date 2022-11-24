use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

use crate::ansi::ast::create_table::{
    CreateTable, TableContentsSource, TableElement, TableElementList, TableScope,
};
use crate::ansi::parser::common::{column_definition, table_name};
use crate::common::parsers::{comma, left_paren, right_paren, statement_terminator};

/// Parses a `CREATE TABLE` statement.
///
/// # Errors
/// If the create table statement is malformed or has unsupported features, this
/// function call will fail. Check the create table statement documentation
/// [(1)][`CreateTable`] for supported syntax.
pub fn create_table(i: &[u8]) -> IResult<&[u8], CreateTable> {
    let (i, (opt_table_scope, table_name, table_contents_source)) = tuple((
        preceded(
            tag_no_case("CREATE"),
            opt(preceded(multispace1, table_scope)),
        ),
        preceded(
            delimited(multispace1, tag_no_case("TABLE"), multispace1),
            table_name,
        ),
        delimited(multispace1, table_contents_source, statement_terminator),
    ))(i)?;

    let mut create_table = CreateTable::new(&table_name, &table_contents_source);
    if let Some(table_scope) = opt_table_scope {
        create_table.with_table_scope(table_scope);
    }

    Ok((i, create_table))
}

fn table_scope(i: &[u8]) -> IResult<&[u8], TableScope> {
    alt((
        map(tag_no_case("GLOBAL TEMPORARY"), |_| TableScope::Global),
        map(tag_no_case("LOCAL TEMPORARY"), |_| TableScope::Local),
    ))(i)
}

fn table_contents_source(i: &[u8]) -> IResult<&[u8], TableContentsSource> {
    alt((map(
        table_element_list,
        TableContentsSource::TableElementList,
    ),))(i)
}

fn table_element_list(i: &[u8]) -> IResult<&[u8], TableElementList> {
    map(
        delimited(
            tuple((left_paren, multispace0)),
            separated_list1(tuple((multispace0, comma, multispace0)), table_element),
            tuple((multispace0, right_paren)),
        ),
        |list| TableElementList::new(&list),
    )(i)
}

fn table_element(i: &[u8]) -> IResult<&[u8], TableElement> {
    alt((map(column_definition, TableElement::ColumnDefinition),))(i)
}
