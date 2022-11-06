use crate::ansi::data_type_base::CharacterLength;

/// Structures and functions that are used to handle the conversion from the
/// parser data type [(1)] to the `ANSI` data type [(2)].
///
/// [(1)]: sqlparser::ast::DataType
/// [(2)]: crate::ansi::DataType
pub mod data_type_base;

/// Errors when using the `ANSI` dialect.
pub enum Error {}

/// `ANSI` data types [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#_6_1_data_type
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DataType {
    /// CHARACTER\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Character(Option<CharacterLength>),
    /// CHAR\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Char(Option<CharacterLength>),
    /// CHARACTER VARYING\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    CharacterVarying(Option<CharacterLength>),
    /// CHAR VARYING\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    CharVarying(Option<CharacterLength>),
    /// VARCHAR\[([<character_length>])].
    ///
    /// [<character_length>]: CharacterLength
    Varchar(Option<CharacterLength>),
}
