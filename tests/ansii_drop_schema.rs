use sql_helper::ansi;
use sql_helper::ansi::ast::common::{DropBehavior, SchemaName};
use sql_helper::ansi::ast::drop_schema::DropSchema;
use sql_helper::ansi::Statement;
use sql_helper::common::Ident;

use crate::common::verified_stmt;

pub mod common;

#[track_caller]
pub fn parse_drop_schema(input: &str) -> DropSchema {
    let (_, stmt) = ansi::parser::parse_statement(input.as_ref()).unwrap();

    match stmt {
        Statement::DropSchema(drop_schema) => drop_schema,
        _ => {
            unreachable!()
        }
    }
}

#[test]
fn test_drop_schema() {
    verified_stmt("DROP SCHEMA schema_name CASCADE;");
    verified_stmt("DROP SCHEMA schema_name RESTRICT;");
    verified_stmt("DROP SCHEMA catalog_name.schema_name CASCADE;");
    verified_stmt("DROP SCHEMA catalog_name.schema_name RESTRICT;");
}

#[test]
fn test_drop_schema_structure() {
    let parsed_1 = parse_drop_schema("DROP SCHEMA schema_name CASCADE;");
    let expected_1 = DropSchema::new(
        &SchemaName::new(None, &Ident::new(b"schema_name")),
        DropBehavior::Cascade,
    );
    assert_eq!(expected_1, parsed_1, "{}", parsed_1);

    let parsed_2 = parse_drop_schema("DROP SCHEMA schema_name RESTRICT;");
    let expected_2 = DropSchema::new(
        &SchemaName::new(None, &Ident::new(b"schema_name")),
        DropBehavior::Restrict,
    );
    assert_eq!(expected_2, parsed_2, "{}", parsed_2);

    let parsed_3 = parse_drop_schema("DROP SCHEMA catalog_name.schema_name CASCADE;");
    let expected_3 = DropSchema::new(
        &SchemaName::new(
            Some(&Ident::new(b"catalog_name")),
            &Ident::new(b"schema_name"),
        ),
        DropBehavior::Cascade,
    );
    assert_eq!(expected_3, parsed_3, "{}", parsed_3);

    let parsed_4 = parse_drop_schema("DROP SCHEMA catalog_name.schema_name RESTRICT;");
    let expected_4 = DropSchema::new(
        &SchemaName::new(
            Some(&Ident::new(b"catalog_name")),
            &Ident::new(b"schema_name"),
        ),
        DropBehavior::Restrict,
    );
    assert_eq!(expected_4, parsed_4, "{}", parsed_4);
}
