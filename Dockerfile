FROM rust:1.77.0

LABEL org.opencontainers.image.source = "https://github.com/Basicprogrammer10/connorcode"

WORKDIR /connorcode
COPY . .

# Install packages
RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get install -y sqlite3 libsqlite3-dev

# Build crates
RUN cargo install --path builder
RUN cargo install --path .

# Build static content
RUN builder web/static web/static web/dist/static
RUN builder web/template web/template web/dist/template

# Run server
EXPOSE 8080
CMD ["connorcode"]
