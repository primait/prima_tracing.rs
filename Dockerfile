FROM rust:1.59.0

WORKDIR /code

COPY entrypoint /code/entrypoint

RUN cargo install cargo-make

# Serve per avere l'owner dei file scritti dal container uguale all'utente Linux sull'host
USER app

ENTRYPOINT ["/bin/bash"]
