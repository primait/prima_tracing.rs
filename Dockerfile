FROM public.ecr.aws/prima/rust:1.82.0

WORKDIR /code

USER app

COPY ["entrypoint", "/entrypoint"]

ENTRYPOINT ["/entrypoint"]

