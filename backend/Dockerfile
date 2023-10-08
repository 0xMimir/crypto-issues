FROM rust as builder

WORKDIR /home/rust/
COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN useradd -ms /bin/bash ubuntu
USER ubuntu

COPY --from=builder /home/rust/target/release/api /usr/local/bin/

EXPOSE 1111

CMD /usr/local/bin/api