#!/usr/bin/python


def parse(filename: str) -> ([int], [[int]]):
    with open(filename, 'r') as f:
        draws = [int(n) for n in f.readline().strip().split(",")]
        grids = [[(int(val), False) for val in (''.join([f.readline() for _ in range(5)])).strip().split()] for _ in f.readline()]  # Blank line skip with readline
    return draws, grids


def main():
    draws, grids = parse("testinput4.txt")

    print(f"Draws: {draws}")
    print(f"Grids: {grids}")

    # values = [list(line[:-1]) for line in open("input3", 'r').readlines()]
    # print(f"Part 1: {part1(values)}")
    # print(f"Part 2: {part2(values)}")


if __name__ == "__main__":
    main()
