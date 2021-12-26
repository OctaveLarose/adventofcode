#!/usr/bin/python
import itertools
import math
from functools import reduce


def get_surrounding_coords(x, y):
    return [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1), (x - 1, y), (x, y), (x + 1, y), (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]
    # coords = []
    # for y1 in range(y - 1, y + 2):
    #     for x1 in range(x - 1, x + 2):
    #         coords.append((x1, y1))
    # return coords
    # return list(itertools.product(range(y - 1, y + 2), range(x - 1, x + 2)))


def check_pixel(coord, inp, algo):
    surr = get_surrounding_coords(*coord)
    idx = sum([2**(8 - i) if surr[i] in inp else 0 for i in range(0, 9)])
    return algo[idx] == '#'


def enhance(inp, algo):
    # pixels_to_check = set()
    # for p in inp:
    #     pixels_to_check.update(get_surrounding_coords(*p))

    new_inp = set()
    max_x = max([a[0] for a in inp]) + 15
    max_y = max([a[1] for a in inp]) + 15
    min_x = min([a[0] for a in inp]) - 15
    min_y = min([a[1] for a in inp]) - 15

    print(min_x, "to", max_x, ";", min_y, "to", max_y)
    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            if check_pixel((x, y), inp, algo):
                new_inp.add((x, y))
    # new_inp = [s for s in pixels_to_check if check_pixel(s, inp, algo)]
    return new_inp


def enhance_n_times(inp, algo, n):
    for _ in range(n):
        inp = enhance(inp, algo)  # TODO use reduce
        # draw_map(inp)
        # input()
    return inp


def map_to_coords(inp):
    center = (len(inp[0]) // 2, len(inp) // 2)

    coords = []
    for y, l in enumerate(inp):
        for x, c in enumerate(l):
            if c == 1:
                coords.append((x - center[0], y - center[1]))
    return coords
    # return [(x - center[0], y - center[1]) for x, y in itertools.product(range(len(inp)), range(len(inp))) if inp[y][x] == 1]


def draw_map(coords):
    center_x = max([c[0] for c in coords])
    center_y = max([c[1] for c in coords])
    center_x += 1 if center_x % 2 != 0 else 0
    center_y += 1 if center_y % 2 != 0 else 0

    for y, x in itertools.product(range(-center_x, center_x + 1), range(-center_y, center_y + 1)):
        print('#' if (x, y) in coords else '.', end='' if x != center_x else '\n')


def main():
    inp_lines = open("inputs/input20").read().split('\n\n')
    # inp_lines = open("inputs/tests/testinput20").read().split('\n\n')
    algo, inp = inp_lines[0], [[1 if c == '#' else 0 for c in l] for l in inp_lines[1].split('\n')]

    coords = map_to_coords(inp)
    enhanced_coords = enhance_n_times(coords, algo, 2)
    draw_map(enhanced_coords)
    ugly_coords = [e for e in enhanced_coords if -57 < e[0] < 57 and -57 < e[1] < 57]
    draw_map(ugly_coords)
    print("Part 1:", len(ugly_coords))


if __name__ == "__main__":
    main()
