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
    vertices = {i: sys.maxsize for i in range(SIZE**2)}
    vertices[0] = 0
    visited_vertices = []

    pq = PriorityQueue()
    pq.put((0, (0, 0)))

    while not pq.empty():
        (dist, current_vertex) = pq.get()
        visited_vertices.append(current_vertex)
        x1, y1 = current_vertex
        current_vertex_id = SIZE * y1 + x1

        adjs = get_adjacent(current_vertex, cavern)
        for neighbor in adjs:
            x, y = neighbor
            neighbor_id = SIZE * y + x
            distance = cavern[y][x]
            if neighbor not in visited_vertices:
                old_cost = vertices[neighbor_id]
                new_cost = vertices[current_vertex_id] + distance
                if new_cost < old_cost:
                    pq.put((new_cost, neighbor))
                    vertices[neighbor_id] = new_cost

    print(vertices)
    return vertices[SIZE**2 - 1]


def main():
    cavern = [[int(x) for x in l.strip()] for l in open("inputs/tests/testinput15", "r").readlines()]

    print("Part 1:", dijkstra(cavern))


if __name__ == "__main__":
    main()
