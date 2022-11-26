use std::fmt;

use crate::ansi::ast::common::{DropBehavior, SchemaName};

/// `DROP SCHEMA` statement [(1)].
///
/// # Supported syntax
/// ```doc
/// DROP SCHEMA <schema name> <drop behavior>
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_11_2_drop_schema_statement
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct DropSchema {
    /// `<schema name>`
    schema_name: SchemaName,
    /// `<drop behavior>`
    drop_behavior: DropBehavior,
}

impl DropSchema {
    #[must_use]
    pub fn new(schema_name: &SchemaName, drop_behavior: DropBehavior) -> Self {
        Self {
            schema_name: schema_name.clone(),
            drop_behavior,
        }
    }

    #[must_use]
    pub const fn schema_name(&self) -> &SchemaName {
        &self.schema_name
    }

    #[must_use]
    pub const fn drop_behavior(&self) -> DropBehavior {
        self.drop_behavior
    }
}

impl fmt::Display for DropSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DROP SCHEMA {} {};",
            self.schema_name(),
            self.drop_behavior()
        )?;
        Ok(())
    }
}
