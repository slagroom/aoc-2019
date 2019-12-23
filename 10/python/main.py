from collections import defaultdict
import math
import sys

def get_indicies(str, ch):
    return [i for (i,c) in enumerate(str) if c == ch]

def greatest_common_factor(a, b):
    smaller = min(a, b)
    larger = max(a, b)
    if (smaller == 0):
        return larger
    return greatest_common_factor(smaller, larger % smaller)

class Vector:

    def __init__(self, rise, run):

        if rise == 0 and run == 0:
            raise ValueError("A Vector must have a non-zero value for rise or run")

        scale = greatest_common_factor(abs(rise), abs(run))
        # The only way for the scale to be zero should be if rise and run are both zero, which
        # should already have been guarded.
        assert scale != 0

        self.rise = rise / scale
        self.run = run / scale
        self.scale = scale

    @staticmethod
    def from_coords(a, b):
        rise = b[1] - a[1]
        run = b[0] - a[0]
        return Vector(rise, run)

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        return self.rise == other.rise and self.run == other.run

    def __ne__(self, other):
        return not self.__eq__(other)

    def __hash__(self):
        return hash((self.rise, self.run))

    def __repr__(self):
       return 'Vector(%s, %s)' % (self.rise, self.run)

    def __str__(self):
       return '{ rise: %s, run: %s, scale: %s }' % (self.rise, self.run, self.scale)


coordinates = [(x,y) for (y, line) in enumerate(sys.stdin) for x in get_indicies(line, '#')]

def get_paths_from(coord):
    paths = defaultdict(list)
    for other in coordinates:
        if other == coord:
            continue
        paths[Direction.from_coords(coord, other)].append(other)
    return paths

path_sets = dict(map(lambda c: (c, get_paths_from(c)), coordinates))

best_location = max(path_sets.items(), key=lambda t: len(t[1]))

print('part 1:', best_location[0], len(best_location[1]))

print()

for direction in best_location[1]:
    print(direction)
    for spot in best_location[1][direction]:
        print('   ', spot)
