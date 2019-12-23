#!/usr/bin/env python

import math

def greatest_common_factor(a, b):
    smaller = min(a, b)
    larger = max(a, b)
    if (smaller == 0):
        return larger
    return greatest_common_factor(smaller, larger % smaller)

class Direction:
    Up = 0
    UpRight = 1
    Right = 2
    DownRight = 3
    Down = 4
    DownLeft = 5
    Left = 6
    UpLeft = 7

class Vector:

    def __init__(self, point_a, point_b):

        rise = point_b[1] - point_a[1]
        run = point_b[0] - point_a[0]

        if rise == 0 and run == 0:
            raise ValueError("Point A and Point B must be different")

        scale = greatest_common_factor(abs(rise), abs(run))
        # The only way for the scale to be zero should be if rise and run are both zero, which
        # should already have been guarded.
        assert scale != 0

        self.rise = rise
        self.run = run
        self.scale = scale

    def slope(self):
        return (self.rise / self.scale, self.run / self.scale)

    def length(self):
        return math.sqrt(self.rise**2, self.run**2)

    def direction(self):
        if self.run == 0:
            return Direction.Up if self.rise > 0 else Direction.Down
        if self.rise == 0:
            return Direction.Right if self.run > 0 else Direction.Left
        if self.run > 0:
            return Direction.UpRight if self.rise > 0 else Direction.DownRight
        if self.rise > 0:
            return Direction.UpLeft
        return Direction.DownLeft

    def __lt__(self, other):
        if not isinstance(other, self.__class__):
            raise ValueError("Cannot compare self:Vector to other:{}".format(other.__class__))

        if self.direction() < other.direction():
            return True

        if self.direction() > other.direction():
            return False

        # Both vectors have the same direction

        # There is only one slope for Up/Left/Down/Right so these are the same
        if self.direction() in [ Direction.Up, Direction.Down, Direction.Left, Direction.Right ]:
            return False

        return (float(self.rise) / self.run) > (float(other.rise) / other.run)

    def __str__(self):
       return '{ rise: %s, run: %s, scale: %s }' % (self.rise, self.run, self.scale)


def scale(slope):
    if slope[0] == 0:
        return (slope[0], slope[1] * 3)
    elif slope[1] == 0:
        return (slope[0] * 3, slope[1])
    else:
        return (slope[0] * 2, slope[1] * 2)

def to_direction(slope):
    return Vector(slope[0], slope[1])

def from_direction(d):
    return (d.rise, d.run)

expected = [
    (1, 0),
    (2, 1),
    (1, 1),
    (1, 2),
    (0, 1),
    (-1, 2),
    (-1, 1),
    (-2, 1),
    (-1, 0),
    (-2, -1),
    (-1, -1),
    (-1, -2),
    (0, -1),
    (1, -2),
    (1, -1),
    (2, -1)
]

scaled = map(scale, expected)
scrambled = sorted(scaled, key=lambda a: a[0])
directions = map(lambda to: Vector((0,0), to), scrambled)
directions_sorted = sorted(directions)

for i in range(0, len(expected)):
    sca = scaled[i]
    scr = scrambled[i]
    d = directions[i]
    ds = directions_sorted[i]
    print('%8s    %8s    %32s    %32s    %s    %s' % (sca, scr, d, ds, ds.direction(), (float(ds.rise) / ds.run) if ds.run != 0 else 0))

