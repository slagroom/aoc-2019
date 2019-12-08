using System;

namespace aoc
{
    public class Coordinate : IEquatable<Coordinate>
    {
        public int X { get; private set; }

        public int Y { get; private set; }

        public Coordinate(int x, int y)
        {
            X = x;
            Y = y;
        }

        public override bool Equals(object obj)
        {
            return Equals(obj as Coordinate);
        }

        public bool Equals(Coordinate p)
        {
            if (p is null) return false;

            if (ReferenceEquals(this, p)) return true;

            return (X == p.X) && (Y == p.Y);
        }

        public override int GetHashCode() => X * 0x00010000 + Y;

        public static bool operator ==(Coordinate left, Coordinate right) => 
            left is null
                ? right is null
                : left.Equals(right);

        public static bool operator !=(Coordinate left, Coordinate right) =>  !(left == right);

        public override string ToString() => $"{{ X: {X}, Y: {Y} }}";
    }
}