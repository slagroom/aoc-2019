import sys

def reverse(iter):
    it = []
    for x in iter:
        it.append(x)
    return reversed(it)

def get_indicies(str, ch):
    return [i for (i,c) in enumerate(str) if c == ch]


asteroid_map = reverse(sys.stdin)

coordinates = [(x,y) for (y, line) in enumerate(asteroid_map) for x in get_indicies(line, '#')]

for coord in coordinates:
    print(coord)