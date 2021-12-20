#!/usr/bin/python

class Packet:
    LITERAL_ID = 4

    packet_str = ''
    bin_str = ''

    packet_version = None
    packet_type_id = None
    subpackets = []

    def __init__(self, packet_str):
        self.packet_str = packet_str
        self.bin_str = bin(int(self.packet_str, 16))[2:]

    def parse(self):
        self.packet_version = int(self.bin_str[:3], 2)
        self.packet_type_id = int(self.bin_str[3:6], 2)

        if self.packet_version == self.LITERAL_ID:
            self.parse_literal()
        else:
            self.parse_operator()
        return self

    def parse_literal(self):
        pass  # Not needed for part 1.

    def parse_operator(self):
        self.subpackets = self.get_subpackets()
        pass

    def get_subpackets(self):
        length_type_id = int(self.bin_str[7], 2)
        print(length_type_id)
        if length_type_id == 0:
            subpackets_nbr_bits = int(self.bin_str[7:20], 2)
            print(subpackets_nbr_bits)
        else:
            print("TODO implement")

        return []

    def print_info(self):
        print(f"Packet version: {self.packet_version}")
        print(f"Packet type ID: {self.packet_type_id}")

    def get_version_sum(self):
        return self.packet_version + sum([sp.packet_version for sp in self.subpackets])


def main():
    packet_str = "38006F45291200"
    # packet_str = open("inputs/input16").read()

    # assert Packet("D2FE28").parse().get_version_sum() == 6
    # assert Packet("8A004A801A8002F478").parse().get_version_sum() == 16
    # assert Packet("620080001611562C8802118E34").parse().get_version_sum() == 12
    # assert Packet("C0015000016115A2E0802F182340").parse().get_version_sum() == 23
    # assert Packet("A0016C880162017C3686B18A3D4780").parse().get_version_sum() == 31

    print("Part 1:", Packet(packet_str).parse().get_version_sum())


if __name__ == "__main__":
    main()
