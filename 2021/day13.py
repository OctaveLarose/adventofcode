#!/usr/bin/python


def part1(coords, folds):
    new_coords = []

    for fold in folds:
        if fold[0] == 'x':
            for x, y in coords:
                if x < fold[1]:
                    new_coords.append((x, y))
                else:
                    new_coords.append((2 * fold[1] - x, y))
        else:
            for x, y in coords:
                if y < fold[1]:
                    new_coords.append((x, y))
                else:
                    new_coords.append((x, 2 * fold[1] - y))

        return len(set(new_coords))


def main():
    coords_str, folds_str = open("inputs/input13", "r").read().split("\n\n")
    coords = [(int(c[0]), int(c[1])) for c in map(lambda l: l.split(','), coords_str.split("\n"))]
    folds = [(f[0], int(f[1])) for f in map(lambda l: l.replace("fold along ", "").split("="), folds_str.split('\n'))]

    print("Part 1:", part1(coords, folds))


if __name__ == "__main__":
    main()
