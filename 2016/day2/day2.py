"""AoC Day 2

Usage:
  day2.py <filename>
  day2.py (-h | --help)
  day2.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import numpy as np

def parse_instructions(filename):
    instructions = []
    fn = open(filename, 'r')
    for l in fn:
        instructions.append(l.strip())
    return instructions


def engineered_keypad_number(location):
    lookup = [ [  0,   0, 0x1,   0,   0],
               [  0, 0x2, 0x3, 0x4,   0],
               [0x5, 0x6, 0x7, 0x8, 0x9],
               [  0, 0xA, 0xB, 0xC,   0],
               [  0,   0, 0xD,   0,   0] ]

    # Location stored as Row/Column
    return lookup[location[0]][location[1]]

def engineered_keypad_move(direction, location):
    new_location = map(sum, zip(location, direction))
    if engineered_keypad_validation(new_location):
       location = new_location 
    return location

def engineered_keypad_validation(key_loc):
    if key_loc[0] < 0 or key_loc[1] < 0:
        return False
    if key_loc[0] > 4 or key_loc[1] > 4:
        return False
    if engineered_keypad_number(key_loc) == 0:
        return False
    return True


def standard_keypad_number(location):
    lookup = [ [1, 2, 3],
               [4, 5, 6], 
               [7, 8, 9] ]
    # Location stored as Row/Column
    return lookup[location[0]][location[1]]

def standard_keypad_validation(key_loc):
    if key_loc < 0:
        key_loc = 0
    if key_loc > 2:
        key_loc = 2
    return key_loc

def standard_keypad_number(location):
    lookup = [ [1, 2, 3],
               [4, 5, 6], 
               [7, 8, 9] ]
    # Location stored as Row/Column
    return lookup[location[0]][location[1]]

def standard_keypad_move(direction, location):
    location = map(sum, zip(location, direction))
    location[0] = standard_keypad_validation(location[0]) 
    location[1] = standard_keypad_validation(location[1])
    return location


def find_new_location(location, instructions, movement):
    move_translation = {'U':[-1,0], 'L':[0,-1], 'D':[1,0], 'R':[0,1]}
    for c in instructions:
        location = movement(move_translation[c], location)
    return location


if __name__ == '__main__':
    arguments = docopt(__doc__, version='AoC Day 2')
    instructions = parse_instructions(arguments["<filename>"])
    print instructions
    
    loc = [1,1]
    answer = []
    for i in instructions:
        loc = find_new_location(loc, i, standard_keypad_move)
        answer.append(standard_keypad_number(loc))
    print "Standard Keypad: " + str(answer)

    loc = [2,0]
    answer = []
    for i in instructions:
        loc = find_new_location(loc, i, engineered_keypad_move)
        answer.append(engineered_keypad_number(loc))
    np.set_printoptions(formatter={'int':lambda x:hex(int(x))})
    print np.array(answer)


