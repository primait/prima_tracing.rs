FROM public.ecr.aws/prima/rust:1.59.0-2

COPY entrypoint /code/entrypoint

# Serve per avere l'owner dei file scritti dal container uguale all'utente Linux sull'host
USER app

WORKDIR /code

ENTRYPOINT ["/bin/bash"]
