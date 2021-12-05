#!/usr/bin/python
from pprint import pprint

GRID_SIZE = 5


def parse(filename: str) -> ([int], [[int, bool]]):
    with open(filename, 'r') as f:
        draws = [int(n) for n in f.readline().strip().split(",")]
        grids = []
        while f.readline():
            grids.append([[int(val), False] for val in (''.join([f.readline() for _ in range(GRID_SIZE)])).strip().split()])
    return draws, grids


def has_won(grid: [(int, bool)]) -> bool:
    return any([all(r) for r in [[c for n, c in grid[i:i + GRID_SIZE]] for i in range(0, GRID_SIZE * GRID_SIZE, GRID_SIZE)]]) or \
           any([all(c) for c in [[c for n, c in grid[i:GRID_SIZE * GRID_SIZE:GRID_SIZE]] for i in range(GRID_SIZE)]])


def part1(draws: [int], grids: [[int]]) -> int:
    for d in draws:
        for g in grids:
            for idx, (n, c) in enumerate(g):
                g[idx][1] = (n == d) or g[idx][1]
            if has_won(g):
                return sum([n for (n, c) in g if not c]) * d


def part2(draws: [int], grids: [[int]]) -> int:
    win_list = [False] * len(grids)
    for d in draws:
        for g_idx, g in enumerate(grids):
            for idx, (n, c) in enumerate(g[:]):
                g[idx][1] = (n == d) or g[idx][1]
            if has_won(g):
                win_list[g_idx] = True
                if all(win_list):
                    return sum([n for (n, c) in g if not c]) * d


def main():
    draws, grids = parse("input4.txt")

    print(f"Part 1: {part1(draws, grids)}")
    print(f"Part 2: {part2(draws, grids)}")


if __name__ == "__main__":
    main()
