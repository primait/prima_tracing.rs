use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// All the possible countries in which the application can run.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Country {
    Common,
    Es,
    It,
    Uk,
}

impl FromStr for Country {
    type Err = CountryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "common" => Ok(Self::Common),
            "it" => Ok(Self::It),
            "es" => Ok(Self::Es),
            "uk" => Ok(Self::Uk),
            _ => Err(CountryParseError(s.to_string())),
        }
    }
}

impl Display for Country {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let country = match self {
            Self::Common => "common",
            Self::Es => "es",
            Self::It => "it",
            Self::Uk => "uk",
        };
        f.write_str(country)
    }
}

#[derive(Debug)]
pub struct CountryParseError(String);

impl Display for CountryParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} is not a valid country string. Allowed strings are 'common', 'it', 'es' and 'uk'.",
            &self.0
        ))
    }
}
