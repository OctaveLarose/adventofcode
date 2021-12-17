#!/usr/bin/python
import sys
from queue import PriorityQueue

def get_adjacent(coord, cavern):
    x, y = coord
    coords = [(x, y - 1), (x - 1, y), (x, y + 1), (x + 1, y)]
    return list(filter(lambda t: 0 <= t[0] < len(cavern[0]) and 0 <= t[1] < len(cavern[0]), [(x1, y1) for x1, y1 in coords]))


def get_min_cost_vertex(vertices):
    return vertices.index(min(vertices))


def dijkstra(cavern):
    SIZE = len(cavern)
    vertices = [sys.maxsize for i in range(SIZE**2)]
    visited_vertices = []
    vertices[0] = 0

    while len(visited_vertices) != SIZE**2:
        mcv = get_min_cost_vertex(vertices)
        x, y = mcv % SIZE, mcv // SIZE
        adjs = get_adjacent((x, y), cavern)
        for xa, ya in adjs:
            vertices[ya * SIZE + xa] = vertices[mcv] + cavern[ya][xa]
        visited_vertices.append(((x, y), vertices[mcv]))
        vertices[mcv] = sys.maxsize - 1

    return visited_vertices


def part1(cavern):
    print(dijkstra(cavern))
    return 42


def main():
    cavern = [[int(x) for x in l.strip()] for l in open("inputs/tests/testinput15", "r").readlines()]
    # cavern[0][0][0] = cavern[0][0][1]
    print(cavern)
    print("Part 1:", part1(cavern))


if __name__ == "__main__":
    main()
