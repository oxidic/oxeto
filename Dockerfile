FROM rust:latest as builder

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust:latest,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    cargo build --release && mv ./target/release/oxeto ./oxeto

FROM ubuntu:latest

ENV PATH="$PATH:/root/.oxido/bin"

EXPOSE 8000

RUN apt-get update && apt-get install -y openssl curl
RUN curl --proto '=https' --tlsv1.2 -sLSf -o oxate_install.sh https://raw.githubusercontent.com/oxidic/oxate/main/install.sh
RUN chmod +rwx oxate_install.sh
RUN ./oxate_install.sh
RUN oxate install

COPY --from=builder /oxeto /oxeto

CMD ["./oxeto"]