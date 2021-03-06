use std::fs;
use std::path::Path;

use bitvec::slice::BitSlice;

use bitvec::prelude::*;
use itertools::Itertools;

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    sub_packets: PacketKind,
}

fn read_literal(bits: &BitSlice<Msb0, u8>) -> (u64, &BitSlice<Msb0, u8>) {
    let mut value: u64 = 0;
    let mut remaing = bits;
    loop {
        println!("in loop readin literal");
        let (new_chunk, s_remaing) = remaing.split_at(5);
        remaing = s_remaing;
        let (last, num_chunk) = new_chunk.split_at(1);
        let is_last = last.load_be::<u8>() == 0;
        value = (value << 4) + num_chunk.load_be::<u64>();
        if is_last {
            break;
        }
    }
    (value, remaing)
}
impl Packet {
    fn eval(&self) -> u64 {
        match &self.sub_packets {
            PacketKind::Literal(v) => *v,
            PacketKind::Op(packs) => match self.type_id {
                0 => packs.iter().map(Packet::eval).sum(),
                1 => packs.iter().map(Packet::eval).product(),
                2 => packs.iter().map(Packet::eval).min().unwrap(),
                3 => packs.iter().map(Packet::eval).max().unwrap(),
                5 => {
                    assert_eq!(packs.len(), 2);
                    if packs[1].eval() > packs[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    assert_eq!(packs.len(), 2);
                    if packs[0].eval() < packs[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    assert_eq!(packs.len(), 2);
                    if packs[0].eval() == packs[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("should not happen"),
            },
        }
    }
    fn sum_version(&self) -> u32 {
        let ver = self.version as u32;
        match &self.sub_packets {
            PacketKind::Op(vecs) => {
                let s: u32 = vecs.iter().map(|x| Packet::sum_version(x)).sum();
                ver + s
            }
            _ => ver,
        }
    }
    fn new_from_file<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let content = fs::read_to_string(path).expect("nvalid input");

        Self::new_from_string(content)
    }

    fn new_from_string(content: String) -> Self {
        let s = content
            .chars()
            .map(|a| a.to_digit(16).unwrap() as u8)
            .tuples()
            .map(|(a, b)| a * 16 + b)
            .collect::<Vec<_>>();

        let slice = BitSlice::<Msb0, _>::from_slice(&s).unwrap();
        let (packet, r) = Packet::new_from_bitslice(slice);
        //Make ure only zeros are left
        assert_eq!(r.count_ones(), 0);
        packet
    }

    fn new_from_bitslice(bits: &BitSlice<Msb0, u8>) -> (Self, &BitSlice<Msb0, u8>) {
        let mut slice = bits;
        let version = slice[..3].load_be::<u8>();
        let type_id = slice[3..6].load_be::<u8>();
        println!("version {}, type {}", version, type_id);

        slice = &slice[6..];
        match type_id {
            4 => {
                let (value, remaing) = read_literal(slice);
                (
                    Self {
                        version,
                        type_id,
                        sub_packets: PacketKind::Literal(value),
                    },
                    remaing,
                )
            }
            _ => {
                //operator
                let (length_type_id, remaining) = slice.split_first().unwrap();
                if !!length_type_id {
                    let (sub_packets, remaining) = remaining.split_at(11);
                    let nr_sub_packets = sub_packets.load_be::<usize>();

                    println!("{} nr_sub_packets", nr_sub_packets);
                    let mut limited_slice = remaining;
                    let mut sub_packets = Vec::new();
                    for _ in 0..nr_sub_packets {
                        let (packet, remaining_slice) = Packet::new_from_bitslice(limited_slice);
                        limited_slice = remaining_slice;
                        sub_packets.push(packet);
                    }
                    return (
                        Self {
                            version,
                            type_id,
                            sub_packets: PacketKind::Op(sub_packets),
                        },
                        limited_slice,
                    );
                } else {
                    let (length, remaining) = remaining.split_at(15);
                    let length = length.load_be::<usize>();

                    println!("{} bits to be used", length);
                    let (mut limited_slice, remaining) = remaining.split_at(length);
                    assert_eq!(limited_slice.len(), length);
                    let mut sub_packets = Vec::new();
                    while limited_slice.len() > 0 {
                        println!("New length {}", limited_slice.len());
                        let (packet, remaining_slice) = Packet::new_from_bitslice(limited_slice);
                        limited_slice = remaining_slice;
                        sub_packets.push(packet);
                    }
                    return (
                        Self {
                            version,
                            type_id,
                            sub_packets: PacketKind::Op(sub_packets),
                        },
                        remaining,
                    );
                }
            }
        }
    }
}
#[derive(Debug)]
enum PacketKind {
    Literal(u64),
    Op(Vec<Packet>),
}

fn main() {
    let packet = Packet::new_from_file("src/input.txt");
    println!("Part1 {:?}", &packet.sum_version());

    //let packet = Packet::new_from_string("9C005AC2F8F0".to_string());

    println!("Eval:{}, Struktur: {:?}", &packet.eval(), &packet);
}

#[test]
fn test_D2FE28() {
    let packet = Packet::new_from_string("D2FE28".to_string());
    assert_eq!(packet.version, 6);
    assert_eq!(packet.type_id, 4);

    match packet.sub_packets {
        PacketKind::Literal(l) => assert_eq!(l, 2021),
        _ => assert!(false),
    }
}

#[test]
fn test_38006F45291200() {
    let packet = Packet::new_from_string("38006F45291200".to_string());
    assert_eq!(packet.version, 1);
    assert_eq!(packet.type_id, 6);
    match packet.sub_packets {
        PacketKind::Op(l) => {
            assert_eq!(l.len(), 2);
            match l[0].sub_packets {
                PacketKind::Literal(l) => assert_eq!(l, 10),
                _ => assert!(false),
            }
            match l[1].sub_packets {
                PacketKind::Literal(l) => assert_eq!(l, 20),
                _ => assert!(false),
            }
        }
        _ => assert!(false),
    }
}

#[test]
fn test_EE00D40C823060() {
    let packet = Packet::new_from_string("EE00D40C823060".to_string());
    assert_eq!(packet.version, 7);
    assert_eq!(packet.type_id, 3);
    if let PacketKind::Op(l) = packet.sub_packets {
        assert_eq!(l.len(), 3);
        match l[0].sub_packets {
            PacketKind::Literal(l) => assert_eq!(l, 1),
            _ => assert!(false),
        }
        match l[1].sub_packets {
            PacketKind::Literal(l) => assert_eq!(l, 2),
            _ => assert!(false),
        }
        match l[2].sub_packets {
            PacketKind::Literal(l) => assert_eq!(l, 3),
            _ => assert!(false),
        }
    } else {
        assert!(false)
    }
}

#[test]
fn test_8A004A801A8002F478() {
    let packet = Packet::new_from_string("8A004A801A8002F478".to_string());
    assert_eq!(packet.version, 4);
    if let PacketKind::Op(l) = &packet.sub_packets {
        assert_eq!(l.len(), 1);

        assert_eq!(l[0].version, 1);
        if let PacketKind::Op(l) = &l[0].sub_packets {
            assert_eq!(l.len(), 1);

            assert_eq!(l[0].version, 5);
        }
    } else {
        assert!(false)
    }
    assert_eq!(packet.sum_version(), 16);
}

#[test]
fn test_620080001611562C8802118E34() {
    let packet = Packet::new_from_string("620080001611562C8802118E34".to_string());

    assert_eq!(packet.sum_version(), 12);
}

#[test]
fn test_C0015000016115A2E0802F182340() {
    let packet = Packet::new_from_string("C0015000016115A2E0802F182340".to_string());

    assert_eq!(packet.sum_version(), 23);
}

#[test]
fn test_A0016C880162017C3686B18A3D4780() {
    let packet = Packet::new_from_string("A0016C880162017C3686B18A3D4780".to_string());

    assert_eq!(packet.sum_version(), 31);
}

#[test]
fn test_C200B40A82() {
    let packet = Packet::new_from_string("C200B40A82".to_string());

    assert_eq!(packet.eval(), 3);
}

#[test]
fn test_04005AC33890() {
    let packet = Packet::new_from_string("04005AC33890".to_string());

    assert_eq!(packet.eval(), 54);
}
#[test]
fn test_880086C3E88112() {
    let packet = Packet::new_from_string("880086C3E88112".to_string());

    assert_eq!(packet.eval(), 7);
}

#[test]
fn test_CE00C43D881120() {
    let packet = Packet::new_from_string("CE00C43D881120".to_string());

    assert_eq!(packet.eval(), 9);
}

#[test]
fn test_D8005AC2A8F0() {
    let packet = Packet::new_from_string("D8005AC2A8F0".to_string());

    assert_eq!(packet.eval(), 1);
}

#[test]
fn test_F600BC2D8F() {
    let packet = Packet::new_from_string("F600BC2D8F".to_string());

    assert_eq!(packet.eval(), 0);
}

#[test]
fn test_9C005AC2F8F0() {
    let packet = Packet::new_from_string("9C005AC2F8F0".to_string());

    assert_eq!(packet.eval(), 0);
}

#[test]
fn test_9C0141080250320F1802104A08() {
    let packet = Packet::new_from_string("9C0141080250320F1802104A08".to_string());

    assert_eq!(packet.eval(), 1);
}

//180616639580
