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
	programCounter uint64
	relativeBase   int64
	memory         map[uint64]int64
	inputFn        func() int64
	outputFn       func(int64)
	onHalt         func()
}

func NewIntcodeProcessor(
	input func() int64,
	output func(int64),
	onHalt func()) intcodeProcessor {

	return intcodeProcessor{
		programCounter: 0,
		relativeBase:   0,
		memory:         make(map[uint64]int64),
		inputFn:        input,
		outputFn:       output,
		onHalt:         onHalt,
	}
}

func (self *intcodeProcessor) Store(address uint64, word int64) {
	self.memory[address] = word
}

func (self *intcodeProcessor) fetch(address uint64) int64 {
	if word, ok := self.memory[address]; ok {
		return word
	}
	return 0
}

func (self *intcodeProcessor) instruction() int64 {
	return self.fetch(self.programCounter)
}

func (self *intcodeProcessor) opCode() int64 {
	return self.instruction() % 100
}

func (self *intcodeProcessor) argMode(position uint64) int64 {
	mode_mask := int64(math.Pow10(int(position + 1)))
	shifted := self.instruction() / mode_mask
	mode := shifted % 10
	return mode
}

func (self *intcodeProcessor) valArg(position uint64) int64 {
	value := self.fetch(self.programCounter + position)
	mode := self.argMode(position)
	switch mode {
	case 0:
		return self.fetch(uint64(value)) // position mode
	case 1:
		return value // immediate mode
	case 2:
		return self.fetch(uint64(value + self.relativeBase)) // relative mode
	default:
		panic(fmt.Errorf("Invalid parameter mode '%v' in position '%v' at address '%v'", mode, position, self.programCounter))
	}
}

func (self *intcodeProcessor) refArg(position uint64) uint64 {
	mode := self.argMode(position)
	switch mode {
	case 0:
		return uint64(self.fetch(self.programCounter + position)) // position mode
	case 2:
		return uint64(self.fetch(self.programCounter + position) + self.relativeBase) // relative mode
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
		case 9:
			self.adjustBase()
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
	arg2 := uint64(self.valArg(2))
	switch arg1 {
	case 0:
		self.programCounter += 3
	default:
		self.programCounter = arg2
	}
}

func (self *intcodeProcessor) jz() {
	arg1 := self.valArg(1)
	arg2 := uint64(self.valArg(2))
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

func (self *intcodeProcessor) adjustBase() {
	arg1 := self.valArg(1)
	self.relativeBase += arg1
	self.programCounter += 2
}

func readProgram(scanner *bufio.Scanner) []int64 {
	mapInt := func(strs []string) []int64 {
		arr := []int64{}
		for _, substr := range strs {
			i, err := strconv.ParseInt(substr, 10, 64)
			if err != nil {
				panic(err)
			}
			arr = append(arr, int64(i))
		}
		return arr
	}
	scanner.Scan()
	return mapInt(strings.Split(scanner.Text(), ","))
}

func main() {

	scanner := bufio.NewScanner(os.Stdin)
	program := readProgram(scanner)

	computer := NewIntcodeProcessor(
		func() int64 {
			scanner.Scan()
			input, err := strconv.ParseInt(scanner.Text(), 10, 64)
			if err != nil {
				panic(err)
			}
			return input
		},
		func(o int64) { fmt.Println(o) },
		func() { })
		
	for addr, word := range program {
		computer.Store(uint64(addr), word)
	}

	computer.Run()
}