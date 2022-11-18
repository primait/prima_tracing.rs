FROM public.ecr.aws/prima/rust:1.59.0-2

WORKDIR /code

COPY entrypoint /code/entrypoint

# Serve per avere l'owner dei file scritti dal container uguale all'utente Linux sull'host
USER app

ENTRYPOINT ["/bin/bash"]
