FROM 279066465364.dkr.ecr.eu-west-1.amazonaws.com/prima-rust:1.97.1

USER root

WORKDIR /code
RUN mkdir -p /code/target && \
    chown -R app:app /code/target

COPY entrypoint /code/entrypoint

VOLUME /code/target

USER app

ENTRYPOINT ["./entrypoint"]
