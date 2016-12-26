"""AoC Day 15

Usage:
  day15.py <filename>
  day15.py (-h | --help)
  day15.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

def parse_disks(filename):
    fn = open(filename, 'r')

    disks = []
    for l in fn:
        pos = l.rstrip().split()
        # (Num positions, start location, disk number)
        disks.append( (int(pos[3]), int(pos[11][0]), int(pos[1][1])) )

    for i in range(100000000):
        d = 0
        all_zero = True
        while d < len(disks):
            disk = disks[d]
            loc = (disk[1] + i + d + 1) % disk[0]
            if loc != 0:
                all_zero = False
                break
            d += 1

        if all_zero:
            print "Drop at time " + str(i)
            break


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    parse_disks(arguments["<filename>"])
