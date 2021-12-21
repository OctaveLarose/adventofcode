#!/usr/bin/python
from functools import reduce


class Packet:
    SUM_ID = 0
    PROD_ID = 1
    MIN_ID = 2
    MAX_ID = 3
    LITERAL_ID = 4
    GTHAN_ID = 5
    LTHAN_ID = 6
    EQUAL_ID = 7

    def __init__(self, input_str):
        if all([c in '01' for c in input_str]):
            self.bin_str = input_str
        else:
            self.packet_str = input_str
            self.bin_str = ''.join([bin(int(c, 16))[2:].zfill(4) for c in self.packet_str])

        # Those two warrant creating subclasses
        self.subpackets = []
        self.packet_value = None

        self.packet_version = None
        self.packet_type_id = None
        self.packet_len = None

    def parse(self):
        self.packet_version = int(self.bin_str[:3], 2)
        self.packet_type_id = int(self.bin_str[3:6], 2)
        self.packet_len = 6

        if self.packet_type_id == self.LITERAL_ID:
            self.parse_literal()
        else:
            self.parse_operator()

        return self

    def parse_literal(self):
        LEN_CHUNK = 5
        value_str = self.bin_str[6:]
        nbr_bits = ''
        for i in range(0, len(value_str), LEN_CHUNK):
            chunk = value_str[i:i + LEN_CHUNK]
            nbr_bits += chunk[1:]
            self.packet_len += LEN_CHUNK
            if chunk[0] == '0':
                break

        self.packet_value = int(nbr_bits, 2)

    def parse_operator(self):
        self.process_subpackets()
        pass

    def process_subpackets(self):
        length_type_id = int(self.bin_str[6], 2)
        self.packet_len += 1
        if length_type_id == 0:
            subpackets_nbr_bits = int(self.bin_str[7:22], 2)
            self.packet_len += 15 + subpackets_nbr_bits
            subpackets_str = self.bin_str[22:22 + subpackets_nbr_bits]
            idx = 0
            while idx <= len(subpackets_str) and not all([c == '0' for c in subpackets_str[idx:]]):
                p = Packet(subpackets_str[idx:]).parse()
                self.subpackets.append(p)
                idx += p.get_packet_len()
        else:
            subpackets_nbr = int(self.bin_str[7:18], 2)
            self.packet_len += 11
            subpackets_str = self.bin_str[18:]
            idx = 0
            for i in range(subpackets_nbr):
                p = Packet(subpackets_str[idx:]).parse()
                self.subpackets.append(p)
                idx += p.get_packet_len()
                self.packet_len += p.get_packet_len()

    def get_packet_len(self):
        return self.packet_len

    # Part 1 function
    def get_version_sum(self):
        return self.packet_version + sum([sp.get_version_sum() for sp in self.subpackets])

    # Part 2 function
    def calculate(self):
        if self.packet_type_id == self.SUM_ID:
            return sum([sp.calculate() for sp in self.subpackets])
        elif self.packet_type_id == self.PROD_ID:
            return reduce(lambda x, y: x * y, [sp.calculate() for sp in self.subpackets])
        elif self.packet_type_id == self.MIN_ID:
            return min([sp.calculate() for sp in self.subpackets])
        elif self.packet_type_id == self.MAX_ID:
            return max([sp.calculate() for sp in self.subpackets])
        elif self.packet_type_id == self.LITERAL_ID:
            return self.packet_value
        elif self.packet_type_id == self.GTHAN_ID:
            return int(self.subpackets[0].calculate() > self.subpackets[1].calculate())
        elif self.packet_type_id == self.LTHAN_ID:
            return int(self.subpackets[0].calculate() < self.subpackets[1].calculate())
        elif self.packet_type_id == self.EQUAL_ID:
            return int(self.subpackets[0].calculate() == self.subpackets[1].calculate())
        else:
            raise NotImplementedError


def part1_asserts():
    assert Packet("D2FE28").parse().get_version_sum() == 6
    assert Packet("8A004A801A8002F478").parse().get_version_sum() == 16
    assert Packet("620080001611562C8802118E34").parse().get_version_sum() == 12
    assert Packet("C0015000016115A2E0802F182340").parse().get_version_sum() == 23
    assert Packet("A0016C880162017C3686B18A3D4780").parse().get_version_sum() == 31


def part2_asserts():
    assert Packet("C200B40A82").parse().calculate() == 3
    assert Packet("04005AC33890").parse().calculate() == 54
    assert Packet("880086C3E88112").parse().calculate() == 7
    assert Packet("CE00C43D881120").parse().calculate() == 9
    assert Packet("D8005AC2A8F0").parse().calculate() == 1
    assert Packet("F600BC2D8F").parse().calculate() == 0
    assert Packet("9C005AC2F8F0").parse().calculate() == 0
    assert Packet("9C0141080250320F1802104A08").parse().calculate() == 1


def main():
    packet_str = open("inputs/input16").read()

    part1_asserts()
    part2_asserts()

    print("Part 1:", Packet(packet_str).parse().get_version_sum())
    print("Part 2:", Packet(packet_str).parse().calculate())


if __name__ == "__main__":
    main()
