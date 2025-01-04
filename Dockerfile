FROM rust

COPY ./ ./

RUN cargo build --release

EXPOSE 3030

CMD ["./target/release/rusti"]