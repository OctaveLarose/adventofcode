#!/usr/bin/python


def get_corrupted_char_in_line(line) -> (bool, str):
    pairs = ['()', '[]', '{}', '<>']
    stack = []
    for c in line:
        stack.append(c)

        while len(stack) != 0 and stack[-1] in ")]}>":
            if ''.join(stack[-2:]) in pairs:
                stack = stack[:-2]
            else:
                return c

        if len(stack) == 0:
            return None

    return None


def part1(lines) -> int:
    invalid_chars = [get_corrupted_char_in_line(l) for l in lines]
    return sum([{')': 3, ']': 57, '}': 1197, '>': 25137, None: 0}.get(c) for c in invalid_chars])


def main():
    lines = [list(l.strip()) for l in open("inputs/tests/testinput10", 'r').readlines()]

    print("Part 1:", part1(lines))


if __name__ == "__main__":
    main()
