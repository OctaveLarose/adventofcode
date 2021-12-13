#!/usr/bin/python
from pprint import pprint


def get_connected_rooms(room, prev_path, conns):
    connected_rooms = []
    for c in conns:
        if room not in c:
            continue

        conn_room = c[0] if c[1] == room else c[1]

        if conn_room.islower() and conn_room in prev_path:
            continue

        connected_rooms.append(conn_room)
    return connected_rooms


def navigate(room, prev_path, conns):
    paths = []

    if room == "end":
        return prev_path + ["end"]
    for cr in get_connected_rooms(room, prev_path, conns):
        paths.append(navigate(cr, prev_path + [room], conns))
    return paths


def part1(conns):
    pprint(navigate("start", [], conns))
    return 42


def main():
    conns = [tuple(v.strip().split("-")) for v in open("inputs/tests/testinput12", 'r').readlines()]

    print("Part 1:", part1(conns))


if __name__ == "__main__":
    main()
