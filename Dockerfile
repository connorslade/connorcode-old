FROM rust:1.65

WORKDIR /connorcode
COPY . .

RUN cargo install --path .

EXPOSE 8080

CMD ["connorcode"]
