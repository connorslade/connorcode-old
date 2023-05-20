FROM rust:1.69

WORKDIR /connorcode
COPY . .

# Build packages
RUN cargo install --path builder
RUN cargo install --path .

# Build static content
RUN builder web/static web/static web/dist/static
RUN builder web/template web/template web/dist/template

# Run server
EXPOSE 8080
CMD ["connorcode"]
