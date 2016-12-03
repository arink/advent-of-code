"""AoC Day 1

Usage:
  day1.py <filename>
  day1.py (-h | --help)
  day1.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

def get_directions(filename):
    fn = open(filename, 'r')
    directions = [x.strip() for x in fn.read().split(',')]
    return directions


def calculate_location(directions):
    direction_change = {'R':90, 'L':-90}
    direction_move = {0:0, 1:1, 2:0, 3:1}
    location = [0, 0]
    facing = 0
    for d in directions:
        turn = d[0];
        length = int(d[1:])
        
        facing = (facing + direction_change[turn]) % 360
        if facing >= 180:
            adjust = -1;
        else:
            adjust = 1;
        location[ direction_move[facing / 90]] += (adjust * length)
    return location


def calculate_distance(location):
    return abs(location[0]) + abs(location[1])


def first_visited_twice(directions):
    visited = set()
    direction_change = {'R':90, 'L':-90}
    direction_move = {0:0, 1:1, 2:0, 3:1}
    location = [0, 0]
    facing = 0
    for d in directions:
        turn = d[0];
        length = int(d[1:])
       
        facing = (facing + direction_change[turn]) % 360
        if facing >= 180:
            adjust = -1;
        else:
            adjust = 1;

        
        intermediate_location = list(location)
        for i in range(1,length):
            intermediate_location[direction_move[facing / 90]] += adjust
            loc = (intermediate_location[0], intermediate_location[1])

            if loc in visited:
                return calculate_distance(loc)
            else:
                visited.add(loc)

        location[ direction_move[facing / 90]] += (adjust * length)
    

def distance(filename):
    directions = get_directions(filename)
    location = calculate_location(directions)
    print calculate_distance(location)
    print first_visited_twice(directions)


if __name__ == '__main__':
    arguments = docopt(__doc__, version='AoC Day 1')
    print arguments["<filename>"]
    distance(arguments["<filename>"])

