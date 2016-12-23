"""AoC Day 11

Usage:
  day11.py 
  day11.py (-h | --help)
  day11.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import itertools
import copy 

SG = 0
SM = 1
PG = 2
PM = 3
TG = 4
TM = 5
RG = 6
RM = 7
CG = 8
CM = 9
EG = 10
EM = 11
DG = 12
DM = 13

initial_floor_plan = [  [SG, SM, PG, PM],
                        [CM, CG, TG, RG, RM],
                        [TM],
                        []
                     ]

second_floor_plan = [  [EG, EM, DG, DM, SG, SM, PG, PM],
                        [CM, CG, TG, RG, RM],
                        [TM],
                        []
                     ]

class Devices:
    def compatible_generator(self,x):
        return x - 1

    def is_microchip(self,x):
        if (x % 2) == 1:
            return True
        return False

    def is_generator(self,x):
        return not self.is_microchip(x)

    def is_pair(self,x, y):
        if x == y - 1 and self.is_generator(x):
            return True
        return False
    
    def num_generators(self,floor):
        num = 0
        for d in floor:
            if self.is_generator(d):
                num += 1
        return num

    def floors_are_valid_configs(self,floors):
        valid = True

        for f in floors:
            num_generators = self.num_generators(f)

            if num_generators != 0:
                for d in f:
                    if self.is_microchip(d) and self.compatible_generator(d) not in f:
                       valid = False
                       break

            if not valid:
                break
        return valid



def enumerate_all_moves(floor_plan, location):
    new_floor = [location - 1, location + 1]
    moves = {}

    for f in new_floor:
        if f < 0 or f >= len(floor_plan):
            continue
        moves[f] = []

        for (i,j) in itertools.combinations(floor_plan[location], 2):
            moved_devices = copy.deepcopy(floor_plan)
            moved_devices[location].remove(i)
            moved_devices[location].remove(j)
            moved_devices[f].append(i)
            moved_devices[f].append(j)
            moves[f].append(moved_devices)
        if len(moves[f]) == 0 or f == location - 1: 
            for i in floor_plan[location]:
                moved_devices = copy.deepcopy(floor_plan)
                moved_devices[location].remove(i)
                moved_devices[f].append(i)
                moves[f].append(moved_devices)

    return moves
       
def current_state_hash(floor_plan, floor):
    for f in floor_plan:
        f.sort()
    modified_floor_plan = copy.deepcopy(floor_plan)

    d = Devices()
    for f in modified_floor_plan:
        if len(f) > 1:
            i = 0
            while i < len(f)-1:
                if d.is_pair(f[i], f[i+1]):
                    f[i] = 'P'
                    f[i+1] = 'P'
                    i += 2
                else:
                    i += 1
        f.sort()
    return hash(str(modified_floor_plan) + str(floor))


# Contains floor plan, current floor, moves
def min_steps(plan, num):
    bfs = []
    visited_states = set()

    d = Devices()
    bfs = [ (plan, 0, 0) ]
    visited_states.add(current_state_hash(initial_floor_plan, 0))

    while len(bfs):
        (floor_plan, iterations, location) = bfs.pop(0)

        if len(floor_plan[3]) == num:
           print iterations
           print floor_plan
           break

        moves = enumerate_all_moves(floor_plan, location)
        for e,possible in moves.iteritems():
            for m in possible:
                if d.floors_are_valid_configs(m):
                    state = current_state_hash(m, e)
                    if state not in visited_states:
                        visited_states.add(state)
                        bfs.append( (m, iterations + 1, e) )


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    min_steps(initial_floor_plan, 10)
    min_steps(second_floor_plan, 14)
