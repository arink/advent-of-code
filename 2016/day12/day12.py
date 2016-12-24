"""AoC Day 12

Usage:
  day12.py <filename>
  day12.py (-h | --help)
  day12.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

class Instruction:
    COPY = 0
    INC = 1
    DEC = 2
    JNZ = 3

    def __init__(self, instruction_string):
        inst = instruction_string.rstrip().split()
        self.instruction_string = instruction_string.rstrip()
        self.value = 0
        self.register = 0
        if inst[0] == "cpy":
            self.instruction = Instruction.COPY
            self.value = inst[1]
            self.register = inst[2]
        elif inst[0] == "inc":
            self.instruction = Instruction.INC
            self.register = inst[1]
        elif inst[0] == "dec":
            self.instruction = Instruction.DEC
            self.register = inst[1]
        elif inst[0] == "jnz":
            self.instruction = Instruction.JNZ
            self.register = inst[1]
            self.value = int(inst[2])
        else:
            print "ERROR: unrecognized instruction: " + inst[0]

    def __str__(self):
        return self.instruction_string

    def decoded(self):
        return (self.instruction, self.register, self.value)
    

class Registers:
    def __init__(self, a, b, c, d):
        self.registers = {'a':a, 'b':b, 'c':c, 'd':d}

    def process_instruction(self, instruction, pc):
        if instruction[0] == Instruction.COPY:
            if instruction[2] in self.registers.keys():
                self.registers[instruction[1]] = self.registers[instruction[2]]
            else:
                self.registers[instruction[1]] = int(instruction[2])
            pc += 1
        elif instruction[0] == Instruction.INC:
            self.registers[instruction[1]] += 1
            pc += 1
        elif instruction[0] == Instruction.DEC:
            self.registers[instruction[1]] -= 1
            pc += 1
        elif instruction[0] == Instruction.JNZ:
            value = int(instruction[1], 16)
            if instruction[1] in self.registers.keys():
                value = self.registers[instruction[1]]
            if value != 0: 
                pc += instruction[2]
            else:
                pc += 1

        return pc

    def __str__(self):
        return str(self.registers)


def parse_instructions(filename):
    fn = open(filename, 'r')

    inst = []
    for l in fn:
        i = Instruction(l.rstrip())
        inst.append(i)

    pc = 0
    r = Registers(0,0,0,0)
    while pc < len(inst):
        pc = r.process_instruction(inst[pc].decoded(), pc) 
    print r

    pc = 0
    r = Registers(0,0,1,0)
    while pc < len(inst):
        pc = r.process_instruction(inst[pc].decoded(), pc) 
    print r



if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    parse_instructions(arguments["<filename>"])
