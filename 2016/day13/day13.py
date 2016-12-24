"""AoC Day 13

Usage:
  day13.py 
  day13.py (-h | --help)
  day13.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import numpy as np

WALL = '#'
OPEN = ' '

def count_bits(num):
    count = 0
    while(num):
        num &= num - 1
        count += 1
    return(count)


def is_wall_per_equation(row, column, key):
    res = column*column + 3*column + 2*column*row + row + row*row + key
    wall = (count_bits(res) % 2) == 1
    return wall


def build_maze(size, key):
    maze = np.ndarray((size,size), dtype=np.dtype('a1'))

    for r in range(size):
        for c in range(size):
            if is_wall_per_equation(r, c, key):
                maze[r][c] = WALL
            else:
                maze[r][c] = OPEN
    return maze


def build_locations(maze, visited, state):
    potentials = [ (state[0] - 1, state[1]), (state[0] + 1, state[1]),
                   (state[0], state[1] - 1), (state[0], state[1] + 1)]

    res = []
    for p in potentials:
        if p[0] < 0 or p[1] < 0:
            continue
        if p[0] >= len(maze) or p[1] >= len(maze):
            continue
        if p not in visited and maze[p[0]][p[1]] != WALL:
            res.append( (p[0], p[1], state[2] + 1))
    return res

def solve(maze, destination, location):
    visited = [location]
    bfs = [(location[0], location[1], 0)]

    while len(bfs):
        state = bfs.pop(0)
        if state[0] == destination[0] and state[1] == destination[1]:
            print "Finished in " + str(state[2]) + " steps"
            break
        visited.append( (state[0], state[1]))
        new_states = build_locations(maze, visited, state)
        bfs.extend(new_states)


def max_visited(maze, location, steps):
    visited = set()
    visited.add( (location[0], location[1]) )
    bfs = [(location[0], location[1], 0)]

    while len(bfs):
        state = bfs.pop(0)
        if state[2] == steps + 1:
            print "Went " + str(steps) + ".  Num visited = " + str(len(visited))
            break
        visited.add( (state[0], state[1]))
        new_states = build_locations(maze, visited, state)
        bfs.extend(new_states)


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')

    np.set_printoptions(threshold=np.inf)
    np.set_printoptions(linewidth=200)

    # Test
    #size = 10
    #maze = build_maze(size, 10)
    #solve(maze, (4, 7), (1,1))

    # Actual
    size = 80
    maze = build_maze(size, 1358)
    solve(maze, (39, 31), (1,1))
    max_visited(maze, (1,1), 50)


