#!/usr/bin/python


def part1(coords, folds):
    return 42


def main():
    coords_str, folds_str = open("inputs/tests/testinput13", "r").read().split("\n\n")
    coords = [(int(c[0]), int(c[1])) for c in map(lambda l: l.split(','), coords_str.split("\n"))]
    folds = [(f[0], int(f[1])) for f in map(lambda l: l.replace("fold along ", "").split("="), folds_str.split('\n'))]

    print(coords, folds)
    print("Part 1:", part1(coords, folds))


if __name__ == "__main__":
    main()
