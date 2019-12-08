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
    }

    class Program
    {
        static IEnumerable<Instruction> ParseInstructions(string source) =>
            source.Split(",").Select(i => new Instruction(i));

        static void Main(string[] args)
        {
            foreach (var lineSegment in ParseInstructions(Console.ReadLine())
                .LineSegments(new Coordinate(0, 0)))
            {
                Console.WriteLine(lineSegment);
            }
        }
    }
}
