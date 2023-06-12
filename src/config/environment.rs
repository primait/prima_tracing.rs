use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// All the possible environments in which the application can run.
#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Environment {
    Dev,
    Qa,
    Staging,
    Production,
}

impl FromStr for Environment {
    type Err = EnvironmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "qa" => Ok(Self::Qa),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err(EnvironmentParseError(s.to_string())),
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Dev => "dev",
            Self::Qa => "qa",
            Self::Staging => "staging",
            Self::Production => "production",
        };
        f.write_str(str)
    }
}

#[derive(Debug)]
pub struct EnvironmentParseError(String);

impl Display for EnvironmentParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} is not a valid environment string. Allowed strings are 'dev', 'qa', 'staging' and 'production'.",
            &self.0
        ))
    }
}
