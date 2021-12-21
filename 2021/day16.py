#!/usr/bin/python

class Packet:
    LITERAL_ID = 4

    packet_str = None
    bin_str = ''

    packet_version = None
    packet_type_id = None

    # Those two warrant creating subclasses
    subpackets = []
    packet_value = None

    packet_len = None

    def __init__(self, input_str):
        if all([c in '01' for c in input_str]):
            self.bin_str = input_str
        else:
            self.packet_str = input_str
            self.bin_str = bin(int(self.packet_str, 16))[2:]

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
        value_str = self.bin_str[6:]
        nbr_bits = ''
        for i in range(0, len(value_str), 5):
            chunk = value_str[i:i + 5]
            nbr_bits += chunk[1:]
            self.packet_len += len(chunk)
            if chunk[0] == '0':
                break

        self.packet_value = int(nbr_bits, 2)

    def parse_operator(self):
        self.process_subpackets()
        pass

    def process_subpackets(self):
        length_type_id = int(self.bin_str[7], 2)
        print(length_type_id)
        if length_type_id == 0:
            subpackets_nbr_bits = int(self.bin_str[7:20], 2)
            self.packet_len += 15 + subpackets_nbr_bits
            subpackets_str = self.bin_str[20:20+subpackets_nbr_bits]
            idx = 0
            while idx <= len(subpackets_str) and not all([c == '0' for c in subpackets_str[idx:]]):
                p = Packet(subpackets_str[idx:]).parse()
                self.subpackets.append(p)
                idx += p.get_packet_len()
        else:
            print("TODO implement")

    def print_info(self):
        print(f"Packet version: {self.packet_version}")
        print(f"Packet type ID: {self.packet_type_id}")
        print(f"Packet length: {self.packet_len}")
        if self.packet_value is not None:
            print(f"Packet value: {self.packet_value}")

    def get_packet_len(self):
        return self.packet_len

    def get_version_sum(self):
        return self.packet_version + sum([sp.get_version_sum() for sp in self.subpackets])


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
