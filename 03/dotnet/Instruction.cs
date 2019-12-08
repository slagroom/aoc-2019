using System;

namespace aoc
{
    public class Instruction
    {
        public Direction Direction { get; private set; }

        public ushort Distance { get; private set; }

        public Instruction(string s)
        {
            if (s is null) throw new ArgumentNullException(nameof(s));

            Direction = GetDirection(s[0]);
            Distance = ushort.Parse(s.Substring(1));    
        }

        public Coordinate ApplyTo(Coordinate start)
        {
            if (start is null) throw new ArgumentNullException(nameof(start));

            switch (Direction)
            {
                case Direction.Left:
                    return new Coordinate(start.X - (int)Distance, start.Y);
                case Direction.Right:
                    return new Coordinate(start.X + (int)Distance, start.Y);
                case Direction.Up:
                    return new Coordinate(start.X, start.Y + (int)Distance);
                case Direction.Down:
                    return new Coordinate(start.X, start.Y - (int)Distance);
            }

            throw new InvalidProgramException();
        }

        private Direction GetDirection(char c)
        {
            switch (c) 
            {
                case 'L':
                    return Direction.Left;
                case 'R':
                    return Direction.Right;
                case 'U':
                    return Direction.Up;
                case 'D':
                    return Direction.Down;
            }

            throw new ArgumentOutOfRangeException(nameof(c), "Must be L, R, U, or D");
        }

        public override string ToString() =>
            $"{{ Direction: {Direction}, Distance: {Distance} }}";
    }
}