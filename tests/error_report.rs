#[cfg(test)]
mod tests {
    use assert2::check;
    use prima_tracing::macros::error_report;

    #[derive(Debug, Default, thiserror::Error)]
    #[error("test error")]
    struct TestError;

    #[derive(Debug, Default, thiserror::Error)]
    #[error("wrapper")]
    struct WrapperError(#[from] TestError);

    #[derive(Debug, Default, thiserror::Error)]
    #[error("top level")]
    struct TopLevelError(#[from] WrapperError);

    #[test]
    fn it_prints_the_source_chain() {
        let error = TopLevelError::default();

        let report = error_report::Report::new(error);

        check!(format!("{report}") == "top level: wrapper: test error");
        check!(
            format!("{report:?}")
                == r#"0: TopLevelError(WrapperError(TestError))
1: WrapperError(TestError)
2: TestError"#
        );
    }
}
