#!/usr/bin/python


def fold(coords, folds, max_folds=None):
    for idx, fold in enumerate(folds):
        new_coords = []
        for x, y in coords:
            if fold[0] == 'x':
                new_coords.append((x, y) if x < fold[1] else (2 * fold[1] - x, y))
            else:
                new_coords.append((x, y) if y < fold[1] else (x, 2 * fold[1] - y))

        coords = set(new_coords)

        if max_folds is not None and idx >= max_folds - 1:
            return len(coords)

    return coords


def print_coords(coords):
    map_x, map_y = (max(l) + 1 for l in zip(*coords))
    map = [["."] * map_x for _ in range(map_y)]
    for x, y in coords:
        map[y][x] = '#'
    print(*map, sep='\n')


def main():
    coords_str, folds_str = open("inputs/input13", "r").read().split("\n\n")
    coords = [(int(c[0]), int(c[1])) for c in map(lambda l: l.split(','), coords_str.split("\n"))]
    folds = [(f[0], int(f[1])) for f in map(lambda l: l.replace("fold along ", "").split("="), folds_str.split('\n'))]

    print("Part 1:", fold(coords, folds, 1))
    print("Part 2:"), print_coords(fold(coords, folds))


if __name__ == "__main__":
    main()
