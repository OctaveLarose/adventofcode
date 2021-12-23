#!/usr/bin/python
import ast
import copy
import itertools
import math

SHOULD_EXPLODE = 1
SHOULD_SPLIT = 2
NOTHING_NEEDED = 3


def get_magnitude(pair):
    if isinstance(pair, int):
        return pair
    return get_magnitude(pair[0]) * 3 + get_magnitude(pair[1]) * 2


def analyse_list(l, coords, looking_for):
    if isinstance(l, int):
        if looking_for == SHOULD_SPLIT and l >= 10:
            return coords, SHOULD_SPLIT
        return None, NOTHING_NEEDED

    if looking_for == SHOULD_EXPLODE and len(coords) >= 4:
        return coords, SHOULD_EXPLODE

    for n in [0, 1]:
        v = analyse_list(l[n], coords + [n], looking_for)
        if v != (None, NOTHING_NEEDED):
            return v

    return None, NOTHING_NEEDED


def get_sublist_elem(pair, coord):
    for c in coord:
        pair = pair[c]
    return pair


def replace_sublist_elem(pair, coord, new_val):
    for c in coord[:-1]:
        pair = pair[c]
    pair[coord[-1]] = new_val


def handle_explode(pair, coord):
    idx_left = coord.copy() if not all([c == 0 for c in coord]) else None
    idx_right = coord.copy() if not all([c == 1 for c in coord]) else None

    if idx_left is not None:
        idx_left = idx_left[:len(idx_left) - idx_left[::-1].index(1) - 1] + [0]
        while isinstance(get_sublist_elem(pair, idx_left), list):
            idx_left += [1]

    if idx_right is not None:
        idx_right = idx_right[:len(idx_right) - idx_right[::-1].index(0) - 1] + [1]
        while isinstance(get_sublist_elem(pair, idx_right), list):
            idx_right += [0]

    x, y = get_sublist_elem(pair, coord)

    if idx_left is not None:
        replace_sublist_elem(pair, idx_left, x + get_sublist_elem(pair, idx_left))
    if idx_right is not None:
        replace_sublist_elem(pair, idx_right, y + get_sublist_elem(pair, idx_right))

    replace_sublist_elem(pair, coord, 0)

    return pair


def handle_split(pair, coord):
    target_nbr = get_sublist_elem(pair, coord)
    new_val = [target_nbr // 2, math.ceil(target_nbr / 2)]
    replace_sublist_elem(pair, coord, new_val)
    return pair


def add(p1, p2):
    curr = [copy.deepcopy(p1)] + [copy.deepcopy(p2)]
    res = None
    while res != NOTHING_NEEDED:
        c, res = analyse_list(curr, [], SHOULD_EXPLODE)
        if res == SHOULD_EXPLODE:
            curr = handle_explode(curr, c)
            continue
        c, res = analyse_list(curr, [], SHOULD_SPLIT)
        if res == SHOULD_SPLIT:
            curr = handle_split(curr, c)
    return curr


def repeated_add(pairs):
    s = pairs[0]
    for p in pairs[1:]:
        s = add(s, p)
    return get_magnitude(s)


def main():
    pairs = [ast.literal_eval(expr) for expr in open("inputs/input18").read().splitlines()]

    print("Part 1:", repeated_add(pairs))
    print("Part 2:", max([get_magnitude(add(a, b)) for a, b in list(itertools.permutations(pairs, 2))]))


if __name__ == "__main__":
    main()
