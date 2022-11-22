use std::fmt;

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
    /// `CHARACTER LARGE OBJECT[(<character large object length>)]`.
    CharacterLargeObject(Option<CharacterLargeObjectLength>),
    /// `CHAR LARGE OBJECT[<character large object length>]`.
    CharLargeObject(Option<CharacterLargeObjectLength>),
    /// `CLOB[<character large object length>]`.
    Clob(Option<CharacterLargeObjectLength>),
    /// `NUMERIC[(<precision>, [<scale>])]`
    Numeric(ExactNumberInfo),
    /// `DECIMAL[(<precision>, [<scale>])]`
    Decimal(ExactNumberInfo),
    /// `DECIMAL[(<precision>, [<scale>])]`
    Dec(ExactNumberInfo),
    /// `SMALLINT`
    Smallint,
    /// `INTEGER`
    Integer,
    /// `INT`
    Int,
    /// `BIGINT`
    Bigint,
    /// `FLOAT`
    Float,
    /// `REAL`
    Real,
    /// `DOUBLE PRECISION`
    DoublePrecision,
    /// `DECFLOAT[(<precision>)]`
    DecFloat(Option<u32>),
    /// BOOLEAN
    Boolean,
    /// `DATE`
    Date,
    /// `TIME [(<temporal precision>)] [<with or without time zone>]`
    Time(Option<u32>, WithOrWithoutTimeZone),
    /// `TIMESTAMP [(<temporal precision>)] [<with or without time zone>]`
    Timestamp(Option<u32>, WithOrWithoutTimeZone),
}

/// Character length of a string literal [(1)].
///
/// # Supported syntax
/// ```doc
/// <length> [<character length units>]
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#character-length
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CharacterLength {
    /// `<length>`
    length: u32,
    /// `[<character length units>]`
    opt_units: Option<CharLengthUnits>,
}

/// Character length units of a string literal [(1)].
///
/// # Supported syntax
/// ```doc
/// CHARACTERS
/// | OCTETS
/// ```
///
/// [(1)]: https://jakewheat.github.io/sql-overview/sql-2016-foundation-grammar.html#char-length-units
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum CharLengthUnits {
    /// `CHARACTERS`
    Characters,
    /// `OCTETS`
    Octets,
}

/// Character large object length information (`<character large object
/// length>`).
///
/// # Supported syntax
/// ```plaintext
/// <large object length> [<char length units>]
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CharacterLargeObjectLength {
    /// `<large object length>`
    length: LargeObjectLength,
    /// `[<char length units>]`
    opt_units: Option<CharLengthUnits>,
}

/// Large object length (`<large object length>`).
///
/// # Supported syntax
/// ```plaintext
/// <unsigned integer>[<multiplier>]
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct LargeObjectLength {
    /// `<unsigned integer>`.
    length: u32,
    /// `[<multiplier>]`.
    multiplier: Option<Multiplier>,
}

/// Multiplier information of scale (`<multiplier>`).
///
/// # Supported syntax
/// ```plaintext
/// K
/// | M
/// | G
/// | T
/// | P
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Multiplier {
    /// `K` (kilo)
    K,
    /// `M` (mega)
    M,
    /// `G` (giga)
    G,
    /// `T` (tera)
    T,
    /// `P` (peta)
    P,
}

/// Precision and scale information for exact numbers (`[(precision[,
/// scale])]`).
///
/// # Supported syntax
/// ```doc
/// [(<precision>[, scale])]
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum ExactNumberInfo {
    /// No info was provided.
    #[default]
    None,
    /// `(<precision>)`
    Precision(u32),
    /// `(<precision>, <scale>)`
    PrecisionAndScale(u32, u32),
}

/// Timezone info for temporal types (`<with or without time zone>`).
///
/// # Supported syntax
/// ```doc
/// WITH TIME ZONE
/// | WITHOUT TIME ZONE
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum WithOrWithoutTimeZone {
    /// No time zone info was provided.
    #[default]
    None,
    /// WITH TIME ZONE
    WithTimeZone,
    /// WITHOUT TIME ZONE
    WithoutTimeZone,
}

// TODO split data types
#[allow(clippy::too_many_lines)]
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
            Self::CharacterLargeObject(opt_character_large_object_length) => {
                write!(f, "CHARACTER LARGE OBJECT")?;

                if let Some(character_large_object_length) = opt_character_large_object_length {
                    write!(f, "({character_large_object_length})")?;
                }
            }
            Self::CharLargeObject(opt_character_large_object_length) => {
                write!(f, "CHAR LARGE OBJECT")?;

                if let Some(character_large_object_length) = opt_character_large_object_length {
                    write!(f, "({character_large_object_length})")?;
                }
            }
            Self::Clob(opt_character_large_object_length) => {
                write!(f, "CLOB")?;

                if let Some(character_large_object_length) = opt_character_large_object_length {
                    write!(f, "({character_large_object_length})")?;
                }
            }
            Self::Numeric(exact_number_info) => {
                write!(f, "NUMERIC{exact_number_info}")?;
            }
            Self::Decimal(exact_number_info) => {
                write!(f, "DECIMAL{exact_number_info}")?;
            }
            Self::Dec(exact_number_info) => {
                write!(f, "DEC{exact_number_info}")?;
            }
            Self::DecFloat(opt_precision) => {
                write!(f, "DECFLOAT")?;

                if let Some(precision) = opt_precision {
                    write!(f, "({precision})")?;
                }
            }
            Self::Smallint => {
                write!(f, "SMALLINT")?;
            }
            Self::Integer => {
                write!(f, "INTEGER")?;
            }
            Self::Int => {
                write!(f, "INT")?;
            }
            Self::Bigint => {
                write!(f, "BIGINT")?;
            }
            Self::Float => {
                write!(f, "FLOAT")?;
            }
            Self::Real => {
                write!(f, "REAL")?;
            }
            Self::DoublePrecision => {
                write!(f, "DOUBLE PRECISION")?;
            }
            Self::Boolean => {
                write!(f, "BOOLEAN")?;
            }
            Self::Date => {
                write!(f, "DATE")?;
            }
            Self::Time(opt_precision, tz_info) => {
                write!(f, "TIME")?;

                if let Some(precision) = opt_precision {
                    write!(f, "({precision})")?;
                }

                if !matches!(tz_info, WithOrWithoutTimeZone::None) {
                    write!(f, " {tz_info}")?;
                }
            }
            Self::Timestamp(opt_precision, tz_info) => {
                write!(f, "TIMESTAMP")?;

                if let Some(precision) = opt_precision {
                    write!(f, "({precision})")?;
                }

                if !matches!(tz_info, WithOrWithoutTimeZone::None) {
                    write!(f, " {tz_info}")?;
                }
            }
        }

        Ok(())
    }
}

impl CharacterLength {
    #[must_use]
    pub fn new(length: u32) -> Self {
        Self {
            length,
            opt_units: None,
        }
    }

    pub fn with_units(&mut self, units: CharLengthUnits) -> &mut Self {
        self.opt_units = Some(units);
        self
    }

    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    #[must_use]
    pub fn opt_units(&self) -> Option<CharLengthUnits> {
        self.opt_units
    }
}

impl fmt::Display for CharacterLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.length)?;

        if let Some(units) = self.opt_units() {
            write!(f, " {units}")?;
        }

        Ok(())
    }
}

impl fmt::Display for CharLengthUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Characters => write!(f, "CHARACTERS")?,
            Self::Octets => write!(f, "OCTETS")?,
        }
        Ok(())
    }
}

impl CharacterLargeObjectLength {
    #[must_use]
    pub fn new(length: LargeObjectLength) -> Self {
        Self {
            length,
            opt_units: None,
        }
    }

    pub fn with_units(&mut self, units: CharLengthUnits) -> &mut Self {
        self.opt_units = Some(units);
        self
    }

    #[must_use]
    pub fn length(&self) -> LargeObjectLength {
        self.length
    }

    #[must_use]
    pub fn opt_units(&self) -> Option<CharLengthUnits> {
        self.opt_units
    }
}

impl fmt::Display for CharacterLargeObjectLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.length())?;

        if let Some(units) = self.opt_units() {
            write!(f, " {units}")?;
        }

        Ok(())
    }
}

impl LargeObjectLength {
    #[must_use]
    pub fn new(length: u32) -> Self {
        Self {
            length,
            multiplier: None,
        }
    }

    pub fn with_multiplier(&mut self, multiplier: Multiplier) -> &mut Self {
        self.multiplier = Some(multiplier);
        self
    }

    #[must_use]
    pub fn length(&self) -> u32 {
        self.length
    }

    #[must_use]
    pub fn opt_multiplier(&self) -> Option<Multiplier> {
        self.multiplier
    }
}

impl fmt::Display for LargeObjectLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.length())?;

        if let Some(multiplier) = self.opt_multiplier() {
            write!(f, "{multiplier}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Multiplier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::K => write!(f, "K")?,
            Self::M => write!(f, "M")?,
            Self::G => write!(f, "G")?,
            Self::T => write!(f, "T")?,
            Self::P => write!(f, "P")?,
        }
        Ok(())
    }
}

impl fmt::Display for ExactNumberInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => {
                write!(f, "")
            }
            Self::Precision(precision) => {
                write!(f, "({precision})")
            }
            Self::PrecisionAndScale(precision, scale) => {
                write!(f, "({precision}, {scale})")
            }
        }
    }
}

impl fmt::Display for WithOrWithoutTimeZone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => {
                write!(f, "")?;
            }
            Self::WithTimeZone => {
                write!(f, "WITH TIME ZONE")?;
            }
            Self::WithoutTimeZone => {
                write!(f, "WITHOUT TIME ZONE")?;
            }
        }
        Ok(())
    }
}
