"""AoC Day 10

Usage:
  day10.py <filename>
  day10.py (-h | --help)
  day10.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
from collections import defaultdict


class Instruction:
    def __init__(self, from_bot, low_type, low_id, high_type, high_id):
        self.bot = from_bot
        self.low_type = low_type
        self.low_id = low_id
        self.high_type = high_type
        self.high_id = high_id


    def __str__(self):
        low = self.low_type + " " + str(self.low_id)
        high =self.high_type + " " + str(self.high_id)
        return str(self.bot) + ":" + low + "/" + high


    def assign(self, receptacle, key, micro):
        if key in receptacle.keys():
            receptacle[key].append(micro)
        else:
            receptacle[key] = [micro]


    def resolve(self, bots, output):
        low_micro = bots[self.bot].pop(0)
        high_micro = bots[self.bot].pop()
        if len(bots[self.bot]) == 0:
            bots.pop(self.bot, None)

        if low_micro == 17 and high_micro == 61:
            print "COMPARE: Bot = " + str(self.bot)

        if self.low_type == "output":
            self.assign(output, self.low_id, low_micro)
        else:
            self.assign(bots, self.low_id, low_micro)

        if self.high_type == "output":
            self.assign(output, self.high_id, high_micro)
        else:
            self.assign(bots, self.high_id, high_micro)


def process_instructions(bots, instructions, output):
    i = 0
    while i < len(instructions):
        bot_id = instructions[i].bot
        if bot_id in bots.keys() and len(bots[bot_id]) >= 2:
            bots[bot_id].sort()
            instructions[i].resolve(bots, output)
            instructions.pop(i)
            i = 0
        else:
            i += 1

def parse_instructions(filename):
    fn = open(filename, 'r')
    bots = {}
    instructions = []
    output = {}

    for l in fn:
        tokens = l.rstrip().split()
        if tokens[0] == "value":
            bot_id = int(tokens[5])
            value = int(tokens[1])
            if bot_id in bots.keys():
                bots[bot_id].append(value)
            else:
                bots[bot_id] = [value]
            bots[bot_id].sort()
        else:
            i = Instruction(int(tokens[1]), tokens[5], int(tokens[6]), tokens[10], int(tokens[11]))
            instructions.append(i)
        process_instructions(bots, instructions, output)

    while len(instructions):
        process_instructions(bots, instructions, output)
    return (bots, instructions)


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    (bots, instructions) = parse_instructions(arguments["<filename>"])
