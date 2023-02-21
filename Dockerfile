FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release && mv ./target/release/quorum_simulation quorum_simulation && chmod +x quorum_simulation

ENTRYPOINT ["./quorum_simulation"]