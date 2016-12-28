"""AoC Day 17

Usage:
  day17.py 
  day17.py (-h | --help)
  day17.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import hashlib

UP = 0
DOWN = 1
LEFT = 2
RIGHT = 3

OPEN = ' '
CLOSED = 'X'

def calculate_open_doors(passcode):
    doors = []
    m = hashlib.md5()
    m.update(passcode.encode('utf-8'))
    hashed_passcode = m.hexdigest()

    for i in range(4):
        if hashed_passcode[i] <= 'a':
            doors.append(CLOSED)
        else:
            doors.append(OPEN)
    return doors


def valid_location(loc):
    if loc[0] < 0 or loc[0] > 3 or loc[1] < 0 or loc[1] > 3:
        return False
    return True


def new_location(loc, move):
    if move == UP: tup = (loc[0] - 1, loc[1])
    if move == DOWN: tup = (loc[0] + 1, loc[1])
    if move == LEFT: tup =  (loc[0], loc[1] - 1)
    if move == RIGHT: tup = (loc[0], loc[1] + 1)
    return tup

if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')

    #start_passcode = "ihgpwlah"
    #start_passcode = "ulqzkmiv"
    start_passcode = "qtetzkpl"

    path_lookup = {UP:'U', DOWN:'D', RIGHT:'R', LEFT:'L'}
    
    vault = (3,3)

    min_moves = 0xffffffff
    min_moves_str = ""
    max_moves = 0
    state = [ ((0,0), 0, start_passcode) ]
    while len(state):
        (location, moves, passcode) = state.pop(0)

        doors = calculate_open_doors(passcode)

        i = 0
        for i in range(len(doors)):
            if doors[i] == OPEN:
                new_loc = new_location(location, i)
                if new_loc == vault:
                    if (moves + 1) < min_moves:
                        min_moves = moves + 1
                        min_moves_str = passcode[len(start_passcode):] + path_lookup[i]
                    if (moves + 1) > max_moves:
                        max_moves = moves + 1
                    continue

                if valid_location(new_loc):
                    state.insert(0, (new_loc, moves + 1, passcode + path_lookup[i]))
    print("Max = " + str(max_moves))
    print("Min = " + str(min_moves) + " (" + min_moves_str + ")")
