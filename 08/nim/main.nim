const width = 25
const height = 6

var input = readLine(stdin)

var position = 0

# The greatest possible fewest zeros is all zeros (height * width) so start
# with that + 1.
var zeros = height * width + 1
var ones = 0
var twos = 0

var fewestZeros = zeros
var fewestZerosOnes = 0
var fewestZerosTwos = 0

var image {.noinit.}: array[height * width, char]
for p in image.mitems: p = '2'

for c in input:

    var i = position mod len(image)

    if i == 0:

        if zeros < fewestZeros:
            fewestZeros = zeros
            fewestZerosOnes = ones
            fewestZerosTwos = twos
        
        zeros = 0
        ones = 0
        twos = 0

    if image[i] == '2':
        image[i] = c

    if c == '0':
        zeros += 1
    elif c == '1':
        ones += 1
    elif c == '2':
        twos += 1
    else:
        raise newException(Exception, "invalid digit")

    position += 1

echo "part 1: ", fewestZerosOnes * fewestZerosTwos

echo "part 2: "
position = 0
for p in image:

    if p == '0': stdout.write("  ")
    else: stdout.write("\u2591\u2591")

    position += 1
    if position == width:
        position = 0
        echo ""

echo ""