FROM rust:alpine

WORKDIR /usr/src/aoc

COPY . .

RUN rustc -D warnings -o aoc main.rs

CMD ["./aoc"]