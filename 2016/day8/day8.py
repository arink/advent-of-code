"""AoC Day 8

Usage:
  day8.py <width> <tall> <filename>
  day8.py (-h | --help)
  day8.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import re

class Display:
    def __init__(self, width, tall):
        self.width = width
        self.tall = tall
        self.display = [(['.'] * width) for i in range(tall)]

    def __str__(self):
        return '\n'.join(''.join(*zip(*row)) for row in self.display)

    def create_rectangle(self, width, tall):
        for w in range(width):
            for t in range(tall):
                self.display[t][w] = '#'

    def rotate(self, row_col_inst, row_col_used, amount):
        if row_col_inst == 'row':
            prev = []
            for w in range(self.width):
                prev.append(self.display[row_col_used][w])
            for w in range(self.width):
                self.display[row_col_used][w] = prev[w - amount % self.width]
        elif row_col_inst == 'column':
            prev = []
            for t in range(tall):
                prev.append(self.display[t][row_col_used])
            for t in range(tall):
                self.display[t][row_col_used] = prev[t - amount % self.tall]
        else:
            print "Expected either row or column, not: " + row_col_inst

    def num_pixels_on(self):
        count = 0
        for w in range(self.width):
            for t in range(self.tall):
                if self.display[t][w] == '#':
                    count += 1
        return count


def process_instructions(display, filename):
    fn = open(filename, 'r')

    for l in fn:
        instructions = l.split()
        print instructions
        if instructions[0] == "rect":
            size = instructions[1].split('x')
            display.create_rectangle(int(size[0]), int(size[1]))
        elif l.startswith("rotate"):
            row_col_inst = instructions[1]
            row_col_used = int(instructions[2].split('=')[1])
            amount = int(instructions[4])
            display.rotate(row_col_inst, row_col_used, amount)
        else:
            print "Unrecognized instruction: " + l
        print display


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    width = int(arguments["<width>"])
    tall = int(arguments["<tall>"])
    
    d = Display(width, tall)
    process_instructions(d, arguments["<filename>"])
    print d.num_pixels_on()
