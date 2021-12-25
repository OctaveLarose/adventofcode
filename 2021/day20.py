#!/usr/bin/python
import itertools
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
    new_inp = set()
    for p in inp:
        surr = get_surrounding_coords(*p)
        for s in surr:
            if check_pixel(s, inp, algo):
                new_inp.add(s)
    return new_inp


def enhance_n_times(inp, algo, n):
    for _ in range(n):
        inp = enhance(inp, algo)  # TODO use reduce
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
    print("Part 1:", len(set(enhanced_coords)))


if __name__ == "__main__":
    main()
