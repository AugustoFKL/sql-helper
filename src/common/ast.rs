use std::fmt;

/// SQL special character.
///
/// # Supported syntax
/// ```sql
///   <space>
/// | <double quote>
/// | <percent>
/// | <ampersand>
/// | <quote>
/// | <left paren>
/// | <right paren>
/// | <asterisk>
/// | <plus sign>
/// | <comma>
/// | <minus sign>
/// | <period>
/// | <solidus>
/// | <colon>
/// | <semicolon>
/// | <less than operator>
/// | <equals operator>
/// | <greater than operator>
/// | <question mark>
/// | <left bracket>
/// | <right bracket>
/// | <circumflex>
/// | <underscore>
/// | <vertical bar>
/// | <left brace>
/// | <right brace>
/// | <dollar sign>
/// | <apostrophe>
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SqlSpecialCharacter {
    /// `<space>`.
    Space,
    /// `<double quote>`.
    DoubleQuote,
    /// `<percent>`.
    Percent,
    /// `<ampersand>`.
    Ampersand,
    /// `<quote>`.
    Quote,
    /// `<left paren>`.
    LeftParen,
    /// `<right paren>`.
    RightParen,
    /// `<asterisk>`.
    Asterisk,
    /// `<plus sign>`.
    PlusSign,
    /// `<comma>`.
    Comma,
    /// `<minus sign>`.
    MinusSign,
    /// `<period>`.
    Period,
    /// `<solidus>`.
    Solidus,
    /// `<colon>`.
    Colon,
    /// `<semicolon>`.
    Semicolon,
    /// `<less than operator>`.
    LessThanOperator,
    /// `<equals operator>`.
    EqualsOperator,
    /// `<greater than operator>`.
    GreaterThanOperator,
    /// `<question mark>`.
    QuestionMark,
    /// `<left bracket>`.
    LeftBracket,
    /// `<right bracket>`.
    RightBracket,
    /// `<circumflex>`.
    Circumflex,
    /// `<underscore>`.
    Underscore,
    /// `<vertical bar>`.
    VerticalBar,
    /// `<left brace>`.
    LeftBrace,
    /// `<right brace>`.
    RightBrace,
    /// `<dollar sign>`.
    DollarSign,
}

impl fmt::Display for SqlSpecialCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Space => write!(f, " ")?,
            Self::DoubleQuote => write!(f, "\"")?,
            Self::Percent => write!(f, "%")?,
            Self::Ampersand => write!(f, "&")?,
            Self::Quote => write!(f, "'")?,
            Self::LeftParen => write!(f, "(")?,
            Self::RightParen => write!(f, ")")?,
            Self::Asterisk => write!(f, "*")?,
            Self::PlusSign => write!(f, "+")?,
            Self::Comma => write!(f, ",")?,
            Self::MinusSign => write!(f, "-")?,
            Self::Period => write!(f, ".")?,
            Self::Solidus => write!(f, "/")?,
            Self::Colon => write!(f, ":")?,
            Self::Semicolon => write!(f, ";")?,
            Self::LessThanOperator => write!(f, "<")?,
            Self::EqualsOperator => write!(f, "=")?,
            Self::GreaterThanOperator => write!(f, ">")?,
            Self::QuestionMark => write!(f, "?")?,
            Self::LeftBracket => write!(f, "[")?,
            Self::RightBracket => write!(f, "]")?,
            Self::Circumflex => write!(f, "^")?,
            Self::Underscore => write!(f, "_")?,
            Self::VerticalBar => write!(f, "|")?,
            Self::LeftBrace => write!(f, "{{")?,
            Self::RightBrace => write!(f, "}}")?,
            Self::DollarSign => write!(f, "$")?,
        }

        Ok(())
    }
}
