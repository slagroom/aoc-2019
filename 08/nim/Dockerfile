FROM nimlang/nim

WORKDIR /usr/src/aoc

COPY . .

RUN nim compile --out=aoc main.nim

CMD ["./aoc"]