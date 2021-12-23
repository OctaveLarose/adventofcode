#!/usr/bin/python
import ast
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
    curr = [p1] + [p2]
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


def run_tests():
    assert handle_explode([[[[[9, 8], 1], 2], 3], 4], [0, 0, 0, 0]) == [[[[0, 9], 2], 3], 4]
    assert handle_explode([7, [6, [5, [4, [3, 2]]]]], [1, 1, 1, 1]) == [7, [6, [5, [7, 0]]]]
    assert handle_explode([[3, [2, [1, [7, 3]]]], [6, [5, [4, [3, 2]]]]], [0, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]]
    assert handle_explode([[3, [2, [8, 0]]], [9, [5, [4, [3, 2]]]]], [1, 1, 1, 1]) == [[3, [2, [8, 0]]], [9, [5, [7, 0]]]]
    assert handle_explode([[[[0, 7], 4], [7, [[8, 4], 9]]], [1, 1]], [0, 1, 1, 0]) == [[[[0, 7], 4], [15, [0, 13]]], [1, 1]]

    assert handle_split([[[[0, 7], 4], [15, [0, 13]]], [1, 1]], [0, 1, 0]) == [[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]]
    assert handle_split([[[[0, 7], 4], [[7, 8], [0, 13]]], [1, 1]], [0, 1, 1, 1]) == [[[[0, 7], 4], [[7, 8], [0, [6, 7]]]], [1, 1]]

    assert add([[[[4, 3], 4], 4], [7, [[8, 4], 9]]], [1, 1]) == [[[[0, 7], 4], [[7, 8], [6, 0]]], [8, 1]]
    assert add([[[0, [4, 5]], [0, 0]], [[[4, 5], [2, 6]], [9, 5]]], [7, [[[3, 7], [4, 3]], [[6, 3], [8, 8]]]]) == [
        [[[4, 0], [5, 4]], [[7, 7], [6, 0]]], [[8, [7, 7]], [[7, 9], [5, 0]]]]
    assert add([[[[4, 0], [5, 4]], [[7, 7], [6, 0]]], [[8, [7, 7]], [[7, 9], [5, 0]]]], [[2, [[0, 8], [3, 4]]], [[[6, 7], 1], [7, [1, 6]]]]) == [
        [[[6, 7], [6, 7]], [[7, 7], [0, 7]]], [[[8, 7], [7, 7]], [[8, 8], [8, 0]]]]
    assert add([[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]], [2,9]) == [[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]
    assert add([[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]], [1,[[[9,3],9],[[9,0],[0,7]]]]) == [[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]

    assert add([[[[7, 0], [7, 7]], [[7, 7], [7, 8]]], [[[7, 7], [8, 8]], [[7, 7], [8, 7]]]],
               [7, [5, [[3, 8], [1, 4]]]]) == [[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]




def main():
    pairs = [ast.literal_eval(expr) for expr in open("inputs/input18").read().splitlines()]
    # pairs = [ast.literal_eval(expr) for expr in open("inputs/tests/testinput18").read().splitlines()]

    run_tests()

    print("Part 1:", repeated_add(pairs))
    # print("Part 2:", get_max_magn())


if __name__ == "__main__":
    main()
