using System;
using System.Collections.Generic;
using System.Linq;

namespace aoc
{
    public static class ExtensionMethods
    {
        public static IEnumerable<LineSegment> LineSegments(
            this IEnumerable<Instruction> instructions,
            Coordinate start)
        {
            foreach (var instruction in instructions)
            {
                var end = instruction.ApplyTo(start);
                yield return new LineSegment(start, end);
                start = end;
            }
        }

        public static IEnumerable<PathSegment> PathSegments(this IEnumerable<LineSegment> lineSegments)
        {
            var pathOffset = 0u;
            foreach (var lineSegment in lineSegments)
            {
                yield return new PathSegment(lineSegment, pathOffset);
                pathOffset += lineSegment.Length;
            }
        }

        public static IEnumerable<PathSegment> PathSegments(this IEnumerable<Instruction> instructions) =>
            instructions.LineSegments(new Coordinate(0, 0)).PathSegments();

        public static IDictionary<Coordinate, ulong> Intersections(
            this IEnumerable<PathSegment> self,
            IEnumerable<PathSegment> other)
        {
            var intersections = new Dictionary<Coordinate, ulong>();

            foreach (var a in self)
            {
                foreach (var b in other)
                {
                    var key = a.LineSegment.Intersection(b.LineSegment);
                    if (key is null) continue;

                    var value = a.PathOffset + b.PathOffset
                        + a.LineSegment.PositionOf(key)
                        + b.LineSegment.PositionOf(key);

                    intersections[key] = value;
                }
            }

            return intersections;
        }
    }

    class Program
    {
        static IEnumerable<Instruction> ParseInstructions(string source) =>
            source.Split(",").Select(i => new Instruction(i));

        static IEnumerable<Instruction> ReadInstructions() =>
            ParseInstructions(Console.ReadLine());

        static void Main(string[] args)
        {
            var intersections = ReadInstructions().PathSegments()
                .Intersections(
                    ReadInstructions().PathSegments());

            var part1 = intersections.Keys.Select(c => c.X + c.Y).Min();

            Console.WriteLine($"part 1: {part1}");

            var part2 = intersections.Values.Min();

            Console.WriteLine($"part 2: {part2}");
        }
    }
}
