#!/usr/bin/python
from collections import Counter


def get_connected_rooms(room, conns):
    connected_rooms = []
    for c in conns:
        if room not in c:
            continue
        conn_room = c[0] if c[1] == room else c[1]
        if conn_room == "start":
            continue
        connected_rooms.append(conn_room)
    return connected_rooms


def navigate(room, prev_path, conns, paths, msv):
    if room == "end":
        paths.append(prev_path + ["end"])
        return paths

    if room.islower() and room in prev_path and any([a.islower() and b >= msv for a, b in Counter(prev_path).items()]):
        return paths

    for cr in get_connected_rooms(room, conns):
        navigate(cr, prev_path + [room], conns, paths, msv)

    return paths


def navigator(conns, max_small_visits=1):
    return len(list(filter(None, navigate("start", [], conns, [], max_small_visits))))


def main():
    conns = [tuple(v.strip().split("-")) for v in open("inputs/input12", 'r').readlines()]

    print("Part 1:", navigator(conns))
    print("Part 2:", navigator(conns, 2))


if __name__ == "__main__":
    main()
