use sql_helper::ansi;
use sql_helper::ansi::{CreateSchema, SchemaName, SchemaNameClause, Statement};
use sql_helper::common::Ident;

use crate::common::verified_stmt;

pub mod common;

#[track_caller]
fn parse_create_schema(input: &str) -> CreateSchema {
    let (_, stmt) = ansi::parser::parse_statement(input.as_ref()).unwrap();

    match stmt {
        Statement::CreateSchema(create_schema) => create_schema,
        _ => {
            unreachable!()
        }
    }
}

#[test]
fn test_create_schema() {
    verified_stmt("CREATE SCHEMA schema_name;");
    verified_stmt("CREATE SCHEMA catalog_name.schema_name;");
    verified_stmt("CREATE SCHEMA AUTHORIZATION authorization_name;");
    verified_stmt("CREATE SCHEMA schema_name AUTHORIZATION authorization_name;");
    verified_stmt("CREATE SCHEMA catalog_name.schema_name AUTHORIZATION authorization_name;");
}

#[test]
fn test_create_schema_structure() {
    let parsed_1 = parse_create_schema("CREATE SCHEMA schema_name;");
    let expected_1 = CreateSchema::new(SchemaNameClause::Simple(SchemaName::new(
        None,
        &Ident::new(b"schema_name"),
    )));
    assert_eq!(expected_1, parsed_1, "{}", parsed_1);

    let parsed_2 = parse_create_schema("CREATE SCHEMA catalog_name.schema_name;");
    let expected_2 = CreateSchema::new(SchemaNameClause::Simple(SchemaName::new(
        Some(&Ident::new(b"catalog_name")),
        &Ident::new(b"schema_name"),
    )));
    assert_eq!(expected_2, parsed_2, "{}", parsed_2);

    let parsed_3 = parse_create_schema("CREATE SCHEMA AUTHORIZATION authorization_name;");
    let expected_3 = CreateSchema::new(SchemaNameClause::Authorization(Ident::new(
        b"authorization_name",
    )));
    assert_eq!(expected_3, parsed_3, "{}", parsed_3);

    let parsed_4 =
        parse_create_schema("CREATE SCHEMA schema_name AUTHORIZATION authorization_name;");
    let expected_4 = CreateSchema::new(SchemaNameClause::NamedAuthorization(
        SchemaName::new(None, &Ident::new(b"schema_name")),
        Ident::new(b"authorization_name"),
    ));
    assert_eq!(expected_4, parsed_4, "{}", parsed_4);

    let parsed_5 = parse_create_schema(
        "CREATE SCHEMA catalog_name.schema_name AUTHORIZATION authorization_name;",
    );
    let expected_5 = CreateSchema::new(SchemaNameClause::NamedAuthorization(
        SchemaName::new(
            Some(&Ident::new(b"catalog_name")),
            &Ident::new(b"schema_name"),
        ),
        Ident::new(b"authorization_name"),
    ));
    assert_eq!(expected_5, parsed_5, "{}", parsed_5);
}
