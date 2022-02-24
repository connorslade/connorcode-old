FROM rust:1.58

WORKDIR /connorcode
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["connorcode"]
