"""AoC Day 19

Usage:
  day19.py <num_elves>
  day19.py (-h | --help)
  day19.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt


def move_presents_from_across(presents):
    dropped = 0
    num_elves = len(presents)
    i = 0
    while i < len(presents): 
        if presents[i]:
            across = (i + dropped + (num_elves - dropped)//2) 
            if across < len(presents):
                presents[across] = None
                dropped += 1
            else:
                break
        i += 1
    return list(filter(None, presents[i:] + presents[:i]))


def move_presents_from_left(presents):
    for i in range(0, len(presents), 2):
        to_left = (i+1) % len(presents)
        presents[to_left] = None
    return list(filter(None, presents))


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    num_elves = int(arguments["<num_elves>"])
   
    presents = list(range(1,int(num_elves) + 1))
    while len(presents) > 1:
        presents = move_presents_from_left(presents)
    print("Left: " + str(presents))

    presents = list(range(1,int(num_elves) + 1))
    while len(presents) > 1:
        presents = move_presents_from_across(presents)
    print("Across: " + str(presents))

