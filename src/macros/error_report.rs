//! Backporting of [std::error::Report](https://doc.rust-lang.org/stable/std/error/struct.Report.html)
//! Waiting for it to become stable
use std::{error::Error as StdError, fmt};

pub struct Report<E = Box<dyn StdError>> {
    error: E,
}

impl<E> Report<E> {
    pub fn new(error: E) -> Self {
        Self { error }
    }
}

impl<E> Report<E>
where
    E: StdError,
{
    pub fn sources(&self) -> Sources<'_> {
        error_sources(&self.error)
    }
}

impl<E> fmt::Display for Report<E>
where
    E: StdError,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error)?;

        let sources = self.error.source().into_iter().flat_map(error_sources);

        for cause in sources {
            write!(f, ": {cause}")?;
        }

        Ok(())
    }
}

impl<E> fmt::Debug for Report<E>
where
    E: StdError,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0: {:?}", self.error)?;

        let sources = self
            .error
            .source()
            .into_iter()
            .flat_map(error_sources)
            .enumerate();

        for (i, cause) in sources {
            write!(f, "\n{}: {cause:?}", i + 1)?;
        }

        Ok(())
    }
}

fn error_sources(err: &dyn StdError) -> Sources<'_> {
    Sources { current: Some(err) }
}

pub struct Sources<'a> {
    current: Option<&'a dyn StdError>,
}

impl<'a> Iterator for Sources<'a> {
    type Item = &'a dyn StdError;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(StdError::source);
        current
    }
}
