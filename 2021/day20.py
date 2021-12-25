#!/usr/bin/python
import itertools


def enhance(inp, algo):
    return inp


def enhance_n_times(inp, algo, n):
    for _ in range(n):
        inp = enhance(inp, algo)  # TODO use reduce
    return inp


def map_to_coords(inp):
    center = (len(inp[0]) // 2, len(inp) // 2)
    coords = []

    # TODO use itertools/zip
    for y, l in enumerate(inp):
        for x, _ in enumerate(l):
            if inp[y][x] == 1:
                coords.append((x - center[0], y - center[1]))

    return coords


def draw_map(coords):
    center_x = max([c[0] for c in coords])
    center_y = max([c[1] for c in coords])
    center_x += 1 if center_x % 2 != 0 else 0
    center_y += 1 if center_y % 2 != 0 else 0

    for y in range(-center_y, center_y + 1):
        for x in range(-center_x, center_x + 1):
            print('#' if (x, y) in coords else '.', end='' if x != center_x else '\n')


def main():
    # inp_lines = open("inputs/input20").read().split('\n\n')
    inp_lines = open("inputs/tests/testinput20").read().split('\n\n')
    algo, inp = inp_lines[0], [[1 if c == '#' else 0 for c in l] for l in inp_lines[1].split('\n')]

    coords = map_to_coords(inp)
    enhanced_coords = enhance_n_times(coords, algo, 2)
    draw_map(enhanced_coords)
    print("Part 1:", len(enhanced_coords))



if __name__ == "__main__":
    main()
