"""AoC Day 18

Usage:
  day18.py <filename> <num_rows>
  day18.py (-h | --help)
  day18.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

TRAP = '^'
SAFE = '.'


def tile_state(left, center, right):
    if left == TRAP and center == TRAP and right == SAFE:
        return TRAP
    elif center == TRAP and right == TRAP and left == SAFE:
        return TRAP
    elif center == SAFE and right == SAFE and left == TRAP:
        return TRAP
    elif center == SAFE and left == SAFE and right == TRAP:
        return TRAP
    return SAFE


def generate_floor_plan(filename, num_rows):
    f = open(filename, 'r')

    rows = [list(f.readline().rstrip())]

    while len(rows) < num_rows:
        next_row = []
        # Special case for left column next to wall
        next_row.append(tile_state(SAFE, rows[-1][0], rows[-1][1]))
    
        # Middle columns
        for i in range(1, len(rows[-1]) - 1):
            next_row.append(tile_state(rows[-1][i-1], rows[-1][i], rows[-1][i+1]))

        # Special case for right column next to wall
        next_row.append(tile_state(rows[-1][-2], rows[-1][-1], SAFE))

        rows.append(next_row)

    return rows


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    tiles = generate_floor_plan(arguments["<filename>"], int(arguments["<num_rows>"]))

    safe_count = 0
    for row in tiles:
        for c in row:
            if c == SAFE:
                safe_count += 1
    print("Safe: " + str(safe_count))
