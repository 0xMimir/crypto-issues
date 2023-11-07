FROM rust as builder

WORKDIR /home/rust/
COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt update -y
RUN apt install ca-certificates -y
RUN update-ca-certificates -y

RUN useradd -ms /bin/bash ubuntu
USER ubuntu

COPY --from=builder /home/rust/target/release/api /usr/local/bin/

EXPOSE 1111

CMD /usr/local/bin/api