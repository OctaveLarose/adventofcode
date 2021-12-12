#!/usr/bin/python

def get_connected_rooms(room, previous_room, conns):
    return list(filter(lambda r: r is None or r == previous_room, [c[1] if c[0] == room else (c[0] if c[1] == room else None) for c in conns]))


def navigate(room, previous_room, conns):
    paths = []

    if room == "end":
        return ["end"]
    for cr in get_connected_rooms(room, previous_room, conns):
        paths.append([room] + navigate(cr, room, conns))
    return paths


def part1(conns):
    print(conns)

    print(navigate("start", None, conns))
    return 42


def main():
    conns = [tuple(v.strip().split("-")) for v in open("inputs/tests/testinput12", 'r').readlines()]
    rooms = set([room for pair in conns for room in pair])

    print("Part 1:", part1(conns))


if __name__ == "__main__":
    main()
