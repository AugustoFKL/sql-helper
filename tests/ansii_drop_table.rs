use spectral::assert_that;

use sql_helper::ansi;
use sql_helper::ansi::ast::common::{
    DropBehavior, LocalOrSchemaQualifier, LocalQualifier, SchemaName, TableName,
};
use sql_helper::ansi::ast::drop_table::DropTable;
use sql_helper::ansi::Statement;
use sql_helper::common::Ident;

use crate::common::verified_stmt;

pub mod common;

#[track_caller]
pub fn parse_drop_table(input: &str) -> DropTable {
    let (_, stmt) = ansi::parser::parse_statement(input.as_ref()).unwrap();

    match stmt {
        Statement::DropTable(drop_table) => drop_table,
        _ => {
            unreachable!()
        }
    }
}

#[test]
fn test_drop_schema() {
    match verified_stmt("DROP TABLE table_name CASCADE") {
        Statement::DropTable(drop_table) => {
            let expected_tb = TableName::new(&Ident::new(b"table_name"));
            assert_that!(drop_table.table_name()).is_equal_to(&expected_tb);
            assert_that!(drop_table.drop_behavior()).is_equal_to(DropBehavior::Cascade);
        }
        _ => unreachable!(),
    };
    match verified_stmt("DROP TABLE table_name RESTRICT") {
        Statement::DropTable(drop_table) => {
            let expected_tb = TableName::new(&Ident::new(b"table_name"));
            assert_that!(drop_table.table_name()).is_equal_to(&expected_tb);
            assert_that!(drop_table.drop_behavior()).is_equal_to(DropBehavior::Restrict);
        }
        _ => unreachable!(),
    };
    match verified_stmt("DROP TABLE MODULE.table_name CASCADE") {
        Statement::DropTable(drop_table) => {
            let mut expected_tb = TableName::new(&Ident::new(b"table_name"));
            expected_tb.with_local_or_schema(LocalOrSchemaQualifier::LocalQualifier(
                LocalQualifier::Module,
            ));
            assert_that!(drop_table.table_name()).is_equal_to(&expected_tb);
            assert_that!(drop_table.drop_behavior()).is_equal_to(DropBehavior::Cascade);
        }
        _ => unreachable!(),
    };
    match verified_stmt("DROP TABLE schema_name.table_name CASCADE") {
        Statement::DropTable(drop_table) => {
            let mut expected_tb = TableName::new(&Ident::new(b"table_name"));
            expected_tb.with_local_or_schema(LocalOrSchemaQualifier::Schema(SchemaName::new(
                None,
                &Ident::new(b"schema_name"),
            )));
            assert_that!(drop_table.table_name()).is_equal_to(&expected_tb);
            assert_that!(drop_table.drop_behavior()).is_equal_to(DropBehavior::Cascade);
        }
        _ => unreachable!(),
    };
    match verified_stmt("DROP TABLE catalog_name.schema_name.table_name CASCADE") {
        Statement::DropTable(drop_table) => {
            let mut expected_tb = TableName::new(&Ident::new(b"table_name"));
            expected_tb.with_local_or_schema(LocalOrSchemaQualifier::Schema(SchemaName::new(
                Some(&Ident::new(b"catalog_name")),
                &Ident::new(b"schema_name"),
            )));
            assert_that!(drop_table.table_name()).is_equal_to(&expected_tb);
            assert_that!(drop_table.drop_behavior()).is_equal_to(DropBehavior::Cascade);
        }
        _ => unreachable!(),
    };
}
