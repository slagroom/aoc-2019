FROM gcc:9

WORKDIR /usr/src/aoc

COPY . .

RUN gcc -o aoc -fsanitize=address main.c

CMD ["./aoc"]