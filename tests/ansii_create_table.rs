use test_case::test_case;

use crate::common::verified_stmt;

pub mod common;

#[test_case("CREATE TABLE table_name (id INT)")]
#[test_case("CREATE GLOBAL TEMPORARY TABLE table_name (id INT)")]
#[test_case("CREATE LOCAL TEMPORARY TABLE table_name (id INT, name VARCHAR(20))")]
fn test_create_table(input: &str) {
    verified_stmt(input);
}

#[should_panic]
#[test_case("CREATE TABLE (id INT)")]
#[test_case("CREATE TABLE GLOBAL tb (id INT)")]
#[test_case("CREATE TABLE LOCAL tb (id INT)")]
#[test_case("CREATE TABLE tb ()")]
fn test_create_table_should_fail(input: &str) {
    verified_stmt(input);
}
