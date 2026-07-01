FROM public.ecr.aws/primaassicurazioni/rust:1.96.0-1

USER root

WORKDIR /code
RUN mkdir -p /code/target && \
    chown -R app:app /code/target

COPY entrypoint /code/entrypoint

VOLUME /code/target

USER app

ENTRYPOINT ["./entrypoint"]
