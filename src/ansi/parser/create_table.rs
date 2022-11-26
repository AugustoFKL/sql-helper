use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

use crate::ansi::ast::create_table::{
    CreateTable, TableContentsSource, TableElement, TableElementList, TableScope,
};
use crate::ansi::parser::common::{column_definition, table_name};
use crate::common::parsers::{delimited_ws0, paren_delimited, preceded_ws1, statement_terminator};
use crate::common::tokens::comma;

/// Parses a `CREATE TABLE` statement.
///
/// # Errors
/// If the create table statement is malformed or has unsupported features, this
/// function call will fail. Check the create table statement documentation
/// [(1)][`CreateTable`] for supported syntax.
pub fn create_table(i: &[u8]) -> IResult<&[u8], CreateTable> {
    let (i, (opt_table_scope, table_name, table_contents_source)) = terminated(
        tuple((
            preceded(tag_no_case("CREATE"), opt(preceded_ws1(table_scope))),
            preceded(preceded_ws1(tag_no_case("TABLE")), preceded_ws1(table_name)),
            preceded_ws1(table_contents_source),
        )),
        statement_terminator,
    )(i)?;

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
        paren_delimited(separated_list1(delimited_ws0(comma), table_element)),
        |list| TableElementList::new(&list),
    )(i)
}

fn table_element(i: &[u8]) -> IResult<&[u8], TableElement> {
    alt((map(column_definition, TableElement::ColumnDefinition),))(i)
}
