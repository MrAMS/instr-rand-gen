use std::io::Write;

use rand::Rng;

#[derive(Debug, Clone)]
struct InstrPatternPart {
    value: Vec<bool>,
    width: usize,
    unspcified: bool,
}

impl InstrPatternPart{
    pub fn clear(&mut self){
        self.value.clear();
        self.width = 0;
        self.unspcified = true;
    }
}

struct InstrPattern{
    patterns: Vec<InstrPatternPart>,
}

impl InstrPattern{
    pub fn new(pattern: &str) -> Self{
        let mut patterns: Vec<InstrPatternPart> = Vec::new();
        let mut nxt_part = InstrPatternPart{
            value: Vec::new(),
            width: 0,
            unspcified: false,
        };
        for c in pattern.chars().rev(){
            let unspcified: bool;
            match c {
                '1' | '0' => {
                    unspcified = false;
                },
                '?' => {
                    unspcified = true;
                }
                ' ' => continue,
                _ => panic!("[Error] pattern must only contains 1, 0 or ?"),
            };
            if nxt_part.width == 0 {
                nxt_part.unspcified = unspcified;
            }else if nxt_part.unspcified != unspcified {
                patterns.push(nxt_part.clone());
                nxt_part.clear();
                nxt_part.unspcified = unspcified;
            }
            nxt_part.width += 1;
            if !unspcified {
                nxt_part.value.push(c == '1');
            }
        }
        patterns.push(nxt_part);
        InstrPattern{
            patterns
        }
    }
    pub fn generate(&self) -> Self{
        let mut patterns = self.patterns.clone();
        for part in patterns.iter_mut(){
            if part.unspcified{
                for _ in 0..part.width{
                    part.value.push(rand::random::<bool>());
                }
            }
        }
        InstrPattern{
            patterns
        }
    }
    pub fn write_bits<T: std::ops::BitOr<Output = T> + std::ops::BitXor<Output = T> + std::ops::Shl<Output = T> + std::convert::From<u8> + Default>(&self) -> T {
        let mut bits = T::default();
        // bits = bits ^ bits;
        let mut i: u8 = 0;
        let bits_size = std::mem::size_of::<T>()*8;
        for part in self.patterns.iter(){
            for bit in part.value.iter(){
                assert!(i <= u8::try_from(bits_size).unwrap(), "[Error] pattern is too long for this type");
                if *bit {
                    bits = bits | T::from(1) << T::from(i);
                }
                i += 1;
            }
        }
        bits
    }
}

fn main() {
    let instrs_num = 10;
    let output_path = std::path::Path::new("test.bin");
    let patterns = vec![
        "00000000000100000???????????????", // ADDW
        "00000000000100010???????????????", // SUBW
        "00000000000100100???????????????", // SLT
        "00000000000100101???????????????", // SLTU
    ];

    let mut seeds = Vec::new();
    for p in  patterns {
        seeds.push(InstrPattern::new(p));
    }

    let mut f = std::fs::File::create(&output_path).unwrap();
    let mut rng = rand::thread_rng();
    for _ in 1..instrs_num{
        let i = rng.gen_range(0..seeds.len());
        let instr = seeds[i].generate().write_bits::<u32>();
        println!("{:#b}", instr);
        f.write_all(&instr.to_le_bytes()).unwrap();
    }
}
