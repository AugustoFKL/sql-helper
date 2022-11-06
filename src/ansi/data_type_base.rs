use std::borrow::Borrow;
use std::fmt;

use sqlparser::ast::DataType as SpDataType;

use crate::ansi::DataType;

/// Information about character length [(1)], including length and possibly
/// unit.
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-length
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CharacterLength {
    /// Default (if VARYING) or maximum (if not VARYING) length
    length: u64,
    /// Optional unit. If not informed, the ANSI handles it as CHARACTERS
    /// implicitly
    opt_unit: Option<CharacterLengthUnit>,
}

/// Possible units for characters [(1)].
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#char-length-units
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CharacterLengthUnit {
    /// CHARACTERS unit
    Characters,
    /// OCTETS unit
    Octets,
}

impl CharacterLength {
    /// Returns the length field value.
    #[must_use]
    pub fn length(&self) -> u64 {
        self.length
    }

    /// Returns the opt_unit field optional value.
    #[must_use]
    pub fn opt_unit(&self) -> Option<&CharacterLengthUnit> {
        self.opt_unit.as_ref()
    }
}

impl From<&sqlparser::ast::CharacterLength> for CharacterLength {
    fn from(value: &sqlparser::ast::CharacterLength) -> Self {
        Self {
            length: value.length,
            opt_unit: value.unit.as_ref().map(Into::into),
        }
    }
}

impl fmt::Display for CharacterLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.length())?;
        if let Some(unit) = self.opt_unit() {
            write!(f, " {unit}")?;
        }
        Ok(())
    }
}

impl From<&sqlparser::ast::CharLengthUnits> for CharacterLengthUnit {
    fn from(value: &sqlparser::ast::CharLengthUnits) -> Self {
        match value {
            sqlparser::ast::CharLengthUnits::Characters => Self::Characters,
            sqlparser::ast::CharLengthUnits::Octets => Self::Octets,
        }
    }
}

impl fmt::Display for CharacterLengthUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Characters => {
                write!(f, "CHARACTERS")?;
            }
            Self::Octets => {
                write!(f, "OCTETS")?;
            }
        }
        Ok(())
    }
}

impl From<SpDataType> for DataType {
    fn from(value: SpDataType) -> Self {
        match value {
            SpDataType::Character(opt_len) => {
                Self::Character(opt_len.map(|len| len.borrow().into()))
            }
            SpDataType::Char(opt_len) => Self::Char(opt_len.map(|len| len.borrow().into())),
            SpDataType::CharacterVarying(opt_len) => {
                Self::CharacterVarying(opt_len.map(|len| len.borrow().into()))
            }
            SpDataType::CharVarying(opt_len) => {
                Self::CharVarying(opt_len.map(|len| len.borrow().into()))
            }
            SpDataType::Varchar(opt_len) => Self::Varchar(opt_len.map(|len| len.borrow().into())),
            _ => {
                unimplemented!()
            }
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Character(opt_len) => {
                write!(f, "CHARACTER")?;
                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::Char(opt_len) => {
                write!(f, "CHAR")?;
                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::CharacterVarying(opt_len) => {
                write!(f, "CHARACTER VARYING")?;
                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::CharVarying(opt_len) => {
                write!(f, "CHAR VARYING")?;
                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
            Self::Varchar(opt_len) => {
                write!(f, "VARCHAR")?;
                if let Some(len) = opt_len {
                    write!(f, "({len})")?;
                }
            }
        }

        Ok(())
    }
}
