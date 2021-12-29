#!/usr/bin/python
import itertools
import math
from functools import reduce


def get_surrounding_coords(x, y):
    return [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
            (x - 1, y), (x, y), (x + 1, y),
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]


def check_pixel(coord, inp, algo):
    surr = get_surrounding_coords(*coord)
    idx = 0
    for i in range(0, 9):
        idx += 2 ** (8 - i) if surr[i] in inp else 0
    return algo[idx] == '#'


def enhance(inp, algo, has_padded):
    max_x = max([a[0] for a in inp])
    max_y = max([a[1] for a in inp])
    min_x = min([a[0] for a in inp])
    min_y = min([a[1] for a in inp])

    new_inp = set()
    if not has_padded:
        for y in range(min_y - 1, max_y + 2):
            for x in range(min_x - 1, max_x + 2):
                if check_pixel((x, y), inp, algo):
                    new_inp.add((x, y))
    else:
        for y in range(min_y + 1, max_y):
            for x in range(min_x + 1, max_x):
                if check_pixel((x, y), inp, algo):
                    new_inp.add((x, y))
    return new_inp


def pad(inp):
    max_x = max([a[0] for a in inp])
    max_y = max([a[1] for a in inp])
    min_x = min([a[0] for a in inp])
    min_y = min([a[1] for a in inp])

    for x in range(min_x - 4, max_x + 3):
        for i in range(2):
            inp.add((x + 1, min_y - 2 - i))
            inp.add((x + 1, max_y + 2 + i))
        # inp.add((x + 1, min_y - 2))
        # inp.add((x + 1, max_y + 2))

    for y in range(min_y - 4, max_y + 3):
        # inp.add((min_x - 2, y + 1))
        # inp.add((max_x + 2, y + 1))
        for i in range(2):
            inp.add((min_x - 2 - i, y + 1))
            inp.add((max_x + 2 + i, y + 1))
    return inp


def enhance_n_times(inp, algo, n):
    should_pad = False
    for i in range(n):
        inp = enhance(inp, algo, should_pad)
        print(f'{i + 1}/{n} complete.')
        if algo[0] == '#' and algo[511] == '.':
            should_pad = not should_pad
        if should_pad:
            inp = pad(inp)
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

    for y, x in itertools.product(range(-center_x - 2, center_x + 2), range(-center_y - 2, center_y + 2)):
        print('#' if (x, y) in coords else '.', end='' if x != center_x else '\n')


def main():
    inp_lines = open("inputs/input20").read().split('\n\n')
    # inp_lines = open("inputs/tests/testinput20").read().split('\n\n')
    algo, inp = inp_lines[0], [[1 if c == '#' else 0 for c in l] for l in inp_lines[1].split('\n')]

    coords = map_to_coords(inp)
    enhanced_coords = enhance_n_times(coords, algo, 2)
    draw_map(enhanced_coords)
    # ugly_coords = [e for e in enhanced_coords if -57 < e[0] < 57 and -57 < e[1] < 57]
    # enhanced_coords_2 = enhance_n_times(coords, algo, 50)
    # draw_map(enhanced_coords_2)
    # ugly_coords_2 = [e for e in enhanced_coords_2 if -100 < e[0] < 100 and -100 < e[1] < 100]

    print("Part 1:", len(enhanced_coords))
    # print("Part 2:", len(ugly_coords_2))


if __name__ == "__main__":
    main()
