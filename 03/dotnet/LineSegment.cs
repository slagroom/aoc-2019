using System;
using System.Collections.Generic;

namespace aoc
{
    public class LineSegment
    {
        public Coordinate Start { get; private set; }

        public Coordinate End { get; private set; }

        public LineSegment(Coordinate start, Coordinate end)
        {
            if (start is null) throw new ArgumentNullException(nameof(start));
            if (end is null) throw new ArgumentNullException(nameof(end));
            if (start == end) throw new ArgumentException($"{nameof(start)} and {nameof(end)} must be different");

            Start = start;
            End = end;

            if (!Vertical && !Horizontal)
                throw new ArgumentException($"{nameof(start)} and {nameof(end)} must have a common value for X or Y");
        }

        public Coordinate Intersection(LineSegment other)
        {
            if (other is null) throw new ArgumentNullException(nameof(other));

            if (Horizontal == other.Horizontal) return null;

            if (InRangeOf(other) && other.InRangeOf(this))
            {
                return Horizontal
                    ? new Coordinate(other.Fixed, Fixed)
                    : new Coordinate(Fixed, other.Fixed);
            }
            
            return null;
        }

        public IEnumerable<Coordinate> Coordinates {
            get {
                var current = Start;
                do  {
                    yield return current;
                    current = Next(current);
                } while (current != End);
            }
        }

        private bool InRangeOf(LineSegment other) => (other.Min <= Fixed) && (Fixed <= other.Max);
        
        private bool Vertical => Start.X == End.X;
        private bool Horizontal => Start.Y == End.Y;

        private int Min => Horizontal ? Math.Min(Start.Y, End.Y) : Math.Min(Start.X, End.X);
        private int Max => Horizontal ? Math.Max(Start.Y, End.Y) : Math.Min(Start.X, End.X);

        private int Fixed => Horizontal ? Start.Y : Start.X;

        private Coordinate Next(Coordinate last) {

            if (Horizontal)
            {
                if (Start.X < End.X)
                    return new Coordinate(last.X + 1, last.Y);
                else
                    return new Coordinate(last.X - 1, last.Y);
            }

            if (Start.Y < End.Y)
                return new Coordinate(last.X, last.Y + 1);

            return new Coordinate(last.X, last.Y - 1);
        }

        public override string ToString() => $"[{Start}] -> [{End}]";
    }
}