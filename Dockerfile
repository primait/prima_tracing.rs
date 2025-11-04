FROM public.ecr.aws/primaassicurazioni/rust:1.83.0

WORKDIR /code

USER app

COPY ["entrypoint", "/entrypoint"]

ENTRYPOINT ["/entrypoint"]

