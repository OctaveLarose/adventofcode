#!/usr/bin/python
import itertools


def get_surrounding_coords(x, y):
    return [(x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
            (x - 1, y), (x, y), (x + 1, y),
            (x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]


def enhance(inp, algo, infinite_status):
    min_x, max_x = min([a[0] for a in inp]), max([a[0] for a in inp])
    min_y, max_y = min([a[1] for a in inp]), max([a[1] for a in inp])

    new_inp = set()
    for y in range(min_y - 1, max_y + 2):
        for x in range(min_x - 1, max_x + 2):
            idx = 0
            surr = get_surrounding_coords(x, y)
            for i in range(0, 9):
                if min_x <= surr[i][0] <= max_x and min_y <= surr[i][1] <= max_y:
                    idx += 2 ** (8 - i) if surr[i] in inp else 0
                else:
                    idx += 2 ** (8 - i) if infinite_status else 0

            if algo[idx] == '#':
                new_inp.add((x, y))

    return new_inp


def enhance_n_times(inp, algo, n):
    infinite_status = False
    for i in range(n):
        inp = enhance(inp, algo, infinite_status)
        infinite_status = not infinite_status if algo[0] == '#' and algo[511] == '.' else infinite_status
    return inp


def map_to_coords(inp):
    center = (len(inp[0]) // 2, len(inp) // 2)
    return [(x - center[0], y - center[1]) for x, y in itertools.product(range(len(inp)), range(len(inp))) if inp[y][x] == 1]


def draw_map(coords):
    center_x, center_y = max([c[0] for c in coords]) + 1, max([c[1] for c in coords]) + 1

    for y, x in itertools.product(range(-center_x - 2, center_x + 2), range(-center_y - 2, center_y + 2)):
        print('#' if (x, y) in coords else '.', end='' if x != center_x else '\n')


def main():
    inp_lines = open("inputs/input20").read().split('\n\n')
    # inp_lines = open("inputs/tests/testinput20").read().split('\n\n')
    algo, inp = inp_lines[0], [[1 if c == '#' else 0 for c in l] for l in inp_lines[1].split('\n')]

    coords = map_to_coords(inp)
    print("Part 1:", len(enhance_n_times(coords, algo, 2)))
    print("Part 2:", len(enhance_n_times(coords, algo, 50)))


if __name__ == "__main__":
    main()
