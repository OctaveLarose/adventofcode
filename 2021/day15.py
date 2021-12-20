#!/usr/bin/python
import sys
from math import sqrt
from queue import PriorityQueue


def get_adjacent(coord, cavern_size):
    x, y = coord % cavern_size, coord // cavern_size
    coords = filter(lambda t: all([0 <= v < cavern_size for v in t]), [(x, y - 1), (x - 1, y), (x, y + 1), (x + 1, y)])
    return [y * cavern_size + x for (x, y) in coords]


def slow_and_ugly_dijkstra(cavern):
    cavern_size = int(sqrt(len(cavern)))
    vertices = {i: 0 if i == 0 else sys.maxsize for i in range(cavern_size**2)}
    visited_vertices = []

    pq = PriorityQueue()
    pq.put((0, 0))

    while not pq.empty():
        (dist, current_vertex) = pq.get()
        visited_vertices.append(current_vertex)

        adjs = get_adjacent(current_vertex, cavern_size)
        for neighbor in adjs:
            if neighbor not in visited_vertices:
                distance = cavern[neighbor]
                old_cost = vertices[neighbor]
                new_cost = vertices[current_vertex] + distance
                if new_cost < old_cost:
                    pq.put((new_cost, neighbor))
                    vertices[neighbor] = new_cost

    return vertices[cavern_size**2 - 1]


def get_bigger_cavern(cavern):
    cavern_size = int(sqrt(len(cavern)))
    bigger_cavern = []

    for i in range(cavern_size):
        chunk = cavern[i * cavern_size:i * cavern_size + cavern_size]
        for j in range(5):
            bigger_cavern += map(lambda x: x + j if x + j <= 9 else (x + j) - 9, chunk)

    new_bigger_cavern = []
    for i in range(5):
        new_bigger_cavern += map(lambda x: x + i if x + i <= 9 else (x + i) - 9, bigger_cavern)

    return new_bigger_cavern


def main():
    cavern = [int(x) for x in list(open("inputs/input15", "r").read().replace('\n', ''))]

    print("Part 1:", slow_and_ugly_dijkstra(cavern))
    print("Part 2:", slow_and_ugly_dijkstra(get_bigger_cavern(cavern)))


if __name__ == "__main__":
    main()
