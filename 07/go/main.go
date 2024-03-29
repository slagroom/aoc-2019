package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type intcodeProcessor struct {
	programCounter uint32
	memory         map[uint32]int32
	inputFn        func() int32
	outputFn       func(int32)
	onHalt         func()
}

func NewIntcodeProcessor(
	input func() int32,
	output func(int32),
	onHalt func()) intcodeProcessor {

	return intcodeProcessor{
		programCounter: 0,
		memory:         make(map[uint32]int32),
		inputFn:        input,
		outputFn:       output,
		onHalt:         onHalt,
	}
}

func (self *intcodeProcessor) Store(address uint32, word int32) {
	self.memory[address] = word
}

func (self *intcodeProcessor) fetch(address uint32) int32 {
	if word, ok := self.memory[address]; ok {
		return word
	}
	panic(fmt.Errorf("Attempted to access invalid memory address '%v'", address))
}

func (self *intcodeProcessor) instruction() int32 {
	return self.fetch(self.programCounter)
}

func (self *intcodeProcessor) opCode() int32 {
	return self.instruction() % 100
}

func (self *intcodeProcessor) argMode(position uint32) int32 {
	mode_mask := int32(math.Pow10(int(position + 1)))
	shifted := self.instruction() / mode_mask
	mode := shifted % 10
	return mode
}

func (self *intcodeProcessor) valArg(position uint32) int32 {
	value := self.fetch(self.programCounter + position)
	mode := self.argMode(position)
	switch mode {
	case 0:
		return self.fetch(uint32(value)) // position mode
	case 1:
		return value // immediate mode
	default:
		panic(fmt.Errorf("Invalid parameter mode '%v' in position '%v' at address '%v'", mode, position, self.programCounter))
	}
}

func (self *intcodeProcessor) refArg(position uint32) uint32 {
	mode := self.argMode(position)
	switch mode {
	case 0:
		return uint32(self.fetch(self.programCounter + position))
	default:
		panic(fmt.Errorf("Invalid parameter mode '%v' for ref arg in position '%v' at address '%v'", mode, position, self.programCounter))
	}
}

func (self *intcodeProcessor) Run() {
	halt := false
	for !halt {
		switch self.opCode() {
		case 1:
			self.add()
		case 2:
			self.mul()
		case 3:
			self.input()
		case 4:
			self.output()
		case 5:
			self.jnz()
		case 6:
			self.jz()
		case 7:
			self.lt()
		case 8:
			self.eq()
		case 99:
			halt = true
		default:
			panic(fmt.Errorf("Invalid instruction '%v' at address '%v'", self.opCode(), self.programCounter))
		}
	}
	self.onHalt()
}

func (self *intcodeProcessor) add() {
	arg1 := self.valArg(1)
	arg2 := self.valArg(2)
	dst := self.refArg(3)
	self.Store(dst, arg1+arg2)
	self.programCounter += 4
}

func (self *intcodeProcessor) mul() {
	arg1 := self.valArg(1)
	arg2 := self.valArg(2)
	dst := self.refArg(3)
	self.Store(dst, arg1*arg2)
	self.programCounter += 4
}

func (self *intcodeProcessor) input() {
	dst := self.refArg(1)
	self.Store(dst, self.inputFn())
	self.programCounter += 2
}

func (self *intcodeProcessor) output() {
	arg := self.valArg(1)
	self.outputFn(arg)
	self.programCounter += 2
}

func (self *intcodeProcessor) jnz() {
	arg1 := self.valArg(1)
	arg2 := uint32(self.valArg(2))
	switch arg1 {
	case 0:
		self.programCounter += 3
	default:
		self.programCounter = arg2
	}
}

func (self *intcodeProcessor) jz() {
	arg1 := self.valArg(1)
	arg2 := uint32(self.valArg(2))
	switch arg1 {
	case 0:
		self.programCounter = arg2
	default:
		self.programCounter += 3
	}
}

func (self *intcodeProcessor) lt() {
	arg1 := self.valArg(1)
	arg2 := self.valArg(2)
	dst := self.refArg(3)
	switch arg1 < arg2 {
	case true:
		self.Store(dst, 1)
	case false:
		self.Store(dst, 0)
	}
	self.programCounter += 4
}

func (self *intcodeProcessor) eq() {
	arg1 := self.valArg(1)
	arg2 := self.valArg(2)
	dst := self.refArg(3)
	switch arg1 == arg2 {
	case true:
		self.Store(dst, 1)
	case false:
		self.Store(dst, 0)
	}
	self.programCounter += 4
}

func readProgram(scanner *bufio.Scanner) []int32 {
	mapInt := func(strs []string) []int32 {
		arr := []int32{}
		for _, substr := range strs {
			i, err := strconv.ParseInt(substr, 10, 32)
			if err != nil {
				panic(err)
			}
			arr = append(arr, int32(i))
		}
		return arr
	}
	scanner.Scan()
	return mapInt(strings.Split(scanner.Text(), ","))
}

func Permutations(arr []int32) <-chan []int32 {
	out := make(chan []int32)
	go func() {
		if len(arr) == 1 {
			out <- arr
		} else if len(arr) > 1 {
			for i, v := range arr {
				others := make([]int32, 0, len(arr)-1)
				others = append(others, arr[:i]...)
				others = append(others, arr[i+1:]...)
				for subperm := range Permutations(others) {
					perm := make([]int32, 0, len(arr))
					perm = append(perm, v)
					perm = append(perm, subperm...)
					out <- perm
				}
			}
		}
		close(out)
	}()
	return out
}


func newAmplifier(
	program []int32,
	input <-chan int32,
	output chan<- int32,
	onHalt func()) func() {

	computer := NewIntcodeProcessor(
		func() int32 { return <-input },
		func(o int32) { output<- o },
		onHalt)

	for addr, word := range program {
		computer.Store(uint32(addr), word)
	}

	return func() {
		computer.Run()
	}
}

func part1(program []int32, phases []int32) int32 {

	in := make(chan int32, 2)
	out := make(chan int32, 2)

	amplifiers := []func(){}

	for i, v := range phases {

		in <- v

		if i == 0 {
			in <- 0
		}

		amplifiers = append(amplifiers, newAmplifier(program, in, out, func(){}))

		in = out
		out = make(chan int32, 2)
	}

	for _, amp := range amplifiers {
		go amp()
	}

	return <-in
}

func part2(program []int32, phases []int32) int32 {

	count := len(phases)

	chans := make([]chan int32, count)
	for i, v := range phases {
		chans[i] = make(chan int32, 1<<16)
		chans[i] <- v
	}

	chans[0] <- 0

	waits := make([]chan struct{}, count)
	for i := range waits {
		waits[i] = make(chan struct{}, 1)
	}

	amplifiers := make([]func(), count)
	for i := range amplifiers {

		input := chans[i]
		output := chans[(i+1)%count]
		wait := waits[i]
		onHalt := func() { wait <- struct{}{} }

		amplifiers[i] = newAmplifier(program, input, output, onHalt)
	}

	for _, amp := range amplifiers {
		go amp()
	}

	for _, wait := range waits {
		<- wait
	}

	return <- chans[0]
}

func main() {

	scanner := bufio.NewScanner(os.Stdin)

	program := readProgram(scanner)

	max := int32(math.MinInt32)

	for phases := range Permutations([]int32{0, 1, 2, 3, 4}) {
		out := part1(program, phases)
		if out > max {
			max = out
		}
	}

	fmt.Println("part 1:", max)

	max = int32(math.MinInt32)

	for phases := range Permutations([]int32{5, 6, 7, 8, 9}) {
		out := part2(program, phases)
		if out > max {
			max = out
		}
	}

	fmt.Println("part 2:", max)
}
