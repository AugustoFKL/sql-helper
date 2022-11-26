use crate::ansi::ast::common::{DropBehavior, TableName};
use std::fmt;

/// `DROP TABLE` statement (`<drop table statement>`).
///
/// # Supported syntax
/// ```doc
/// DROP TABLE <table name> <drop behavior>
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DropTable {
    /// `<table name>`
    table_name: TableName,
    /// `<drop behavior>`
    drop_behavior: DropBehavior,
}

impl DropTable {
    #[must_use]
    pub fn new(table_name: &TableName, drop_behavior: DropBehavior) -> Self {
        Self {
            table_name: table_name.clone(),
            drop_behavior,
        }
    }

    #[must_use]
    pub const fn table_name(&self) -> &TableName {
        &self.table_name
    }

    #[must_use]
    pub const fn drop_behavior(&self) -> DropBehavior {
        self.drop_behavior
    }
}

impl fmt::Display for DropTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DROP TABLE {} {}",
            self.table_name(),
            self.drop_behavior()
        )?;
        Ok(())
    }
}
