use std::io::Write;

use rand::Rng;

#[derive(Debug, Copy, Clone)]
enum InstrPatternPart{
    One, Zero, DontCare,
}

struct InstrPattern<IdType>{
    id: IdType,
    parts: Vec<InstrPatternPart>,
}

impl<IdType> InstrPattern<IdType>{
    pub fn new(id: IdType, pattern: &str) -> Self{
        let mut parts: Vec<InstrPatternPart> = Vec::new();
        for c in pattern.chars().rev(){
            match c {
                '1' => {
                    parts.push(InstrPatternPart::One);
                },
                '0' => {
                    parts.push(InstrPatternPart::Zero);
                },
                '?' => {
                    parts.push(InstrPatternPart::DontCare);
                }
                ' ' => continue,
                _ => panic!("[Error] find {}, pattern must only contains 1, 0 or ?", c),
            };
        }
        InstrPattern{
            id,
            parts
        }
    }
    pub fn generate<T: std::ops::BitOrAssign + std::ops::Shl<Output = T> + std::convert::From<u8> + Default>(&self) -> T {
        let mut bits = T::default();
        // bits = bits ^ bits;
        let bits_size = std::mem::size_of::<T>()*8;
        assert!(self.parts.len() <= bits_size, "[Error] pattern is too long for this type");
        for i in 0..self.parts.len() {
            let part = self.parts[i];
            let i_u8 = u8::try_from(i).unwrap();
            match part {
                InstrPatternPart::One => bits |= T::from(1) << T::from(i_u8),
                InstrPatternPart::Zero => (),
                InstrPatternPart::DontCare => bits |= T::from(if rand::random::<bool>(){1} else{0}) << T::from(i_u8),
            }
        }
        bits
    }
    pub fn fit<T: std::ops::BitOrAssign + std::ops::Shr<Output = T> + std::ops::BitAnd<Output = T> + std::cmp::PartialEq + std::convert::From<u8> + Default + Copy>(&self, bits: T) -> bool {
        let one = T::from(1);
        let zero = T::from(0);
        if std::mem::size_of::<T>()*8 < self.parts.len() {
            return false;
        }
        for (i, part) in self.parts.iter().enumerate() {
            let i_u8 = u8::try_from(i).unwrap();
            match part {
                InstrPatternPart::One => {
                    if (bits >> T::from(i_u8)) & one != one {
                        return false;
                    }
                },
                InstrPatternPart::Zero => {
                    if (bits >> T::from(i_u8)) & one != zero {
                        return false;
                    }
                },
                InstrPatternPart::DontCare => (),
            }
        }
        true
    }
}

fn main() {
    let instrs_num = 10;
    let output_path = std::path::Path::new("test.bin");
    let patterns = vec![
        // MSB
        ["00000000000100000???????????????", "ADDW"],
        ["00000000000100010???????????????", "SUBW"], 
        ["00000000000100100???????????????", "SLT"], 
        ["00000000000100101???????????????", "SLTU"],
        ["00000000000101000???????????????", "NOR"],
        ["00000000000101001???????????????", "AND"],
        ["00000000000101010???????????????", "OR"],
        ["00000000000101011???????????????", "XOR"],
        ["00000000000101110???????????????", "SLLW"],
        ["00000000000101111???????????????", "SRLW"],
        ["00000000000110000???????????????", "SRAW"],
    ];

    assert!(InstrPattern::new("ADDW", "00000000000100000???????????????").fit(0x0010421c));

    let mut seeds = Vec::new();
    for p in  patterns {
        seeds.push(InstrPattern::new(p[1], p[0]));
    }

    let mut f = std::fs::File::create(&output_path).unwrap();
    let mut rng = rand::thread_rng();
    for _ in 1..instrs_num{
        let i = rng.gen_range(0..seeds.len());
        let instr = seeds[i].generate::<u32>();
        println!("{} {:#b}", seeds[i].id, instr);
        f.write_all(&instr.to_le_bytes()).unwrap();
    }
}
