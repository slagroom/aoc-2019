using System;

namespace aoc
{
    public class PathSegment
    {
        public LineSegment LineSegment { get; private set; }

        public ulong PathOffset { get; private set; }

        public PathSegment(LineSegment lineSegment, ulong pathOffset)
        {
            LineSegment = lineSegment ?? throw new ArgumentNullException(nameof(lineSegment));
            PathOffset = pathOffset;
        }
    }
}