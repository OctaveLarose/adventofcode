#!/usr/bin/python
from functools import reduce


def get_neighbor_coords(x, y) -> [(int, int)]:
    return [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]


def get_low_points(coords) -> [int]:
    lps = []
    for (x, y), v in coords.items():
        if all(map(lambda cv: cv is None or cv > v, [coords.get(s) for s in get_neighbor_coords(x, y)])):
            lps.append(((x, y), v))
    return lps


def get_basin(point, coords):
    (x, y), v = point

    coords_around = [(s, coords.get(s)) for s in get_neighbor_coords(x, y) if (coords.get(s) is not None and v < coords.get(s) < 9)]

    ret_list = [point]
    for p in coords_around:
        ret_list += get_basin(p, coords)
    return set(ret_list)


def part2(coords) -> int:
    basins_sizes = [len(get_basin(l, coords)) for l in get_low_points(coords)]
    return reduce(lambda x, y: x * y, sorted(basins_sizes)[-3:])


def main():
    hm = [[int(i) for i in l.strip()] for l in open("inputs/input9", 'r').readlines()]
    coords = {(x, y): v for y, l in enumerate(hm) for x, v in enumerate(l)}

    print("Part 1:", sum([lp[1] + 1 for lp in get_low_points(coords)]))
    print("Part 2:", part2(coords))


if __name__ == "__main__":
    main()
