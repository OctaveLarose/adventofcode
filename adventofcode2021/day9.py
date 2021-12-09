#!/usr/bin/python
from functools import reduce
from pprint import pprint


def get_low_points(coords) -> [int]:
    lps = []
    for (x, y), v in coords.items():
        sc = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        if all(map(lambda cv: cv is None or cv > v, [coords.get(s) for s in sc])):
            lps.append(((x, y), v))
    return lps


def get_basin(point, coords):
    (x, y), v = point
    sc = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]

    coords_around = []
    for s in sc:
        if coords.get(s) == v + 1 and coords.get(s) != 9:
            coords_around.append((s, v + 1))

    print(coords_around)
    ret_list = [point]
    for p in coords_around:
        ret_list += get_basin(p, coords)
    return set(ret_list)


def part2(coords) -> int:
    lps = get_low_points(coords)
    basins_sizes = []
    for l in lps:
        b = get_basin(l, coords)
        print(b)
        basins_sizes.append(len(b))

    print(basins_sizes)

    print(sorted(basins_sizes)[-3:])
    print(sorted(basins_sizes))
    return reduce(lambda x, y: x * y, sorted(basins_sizes)[-3:])


def main():
    # hm = [[int(i) for i in l.strip()] for l in open("inputs/tests/testinput9", 'r').readlines()]
    hm = [[int(i) for i in l.strip()] for l in open("inputs/input9", 'r').readlines()]
    coords = {(x, y): v for y, l in enumerate(hm) for x, v in enumerate(l)}

    print("Part 1:", sum([lp[1] + 1 for lp in get_low_points(coords)]))

    print("Part 2:", part2(coords))


if __name__ == "__main__":
    main()
