#[derive(Debug, Copy, Clone)]
enum BitsPatternPart{
    One, Zero, DontCare,
}

pub struct BitsPattern{
    parts: Vec<BitsPatternPart>,
}

impl BitsPattern{
    pub fn new(pattern: &str) -> Self{
        let mut parts: Vec<BitsPatternPart> = Vec::new();
        for c in pattern.chars().rev(){
            match c {
                '1' => {
                    parts.push(BitsPatternPart::One);
                },
                '0' => {
                    parts.push(BitsPatternPart::Zero);
                },
                '?' => {
                    parts.push(BitsPatternPart::DontCare);
                }
                ' ' => continue,
                _ => panic!("[Error] find {}, pattern must only contains 1, 0 or ?", c),
            };
        }
        BitsPattern{
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
                BitsPatternPart::One => bits |= T::from(1) << T::from(i_u8),
                BitsPatternPart::Zero => (),
                BitsPatternPart::DontCare => bits |= T::from(if rand::random::<bool>(){1} else{0}) << T::from(i_u8),
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
                BitsPatternPart::One => {
                    if (bits >> T::from(i_u8)) & one != one {
                        return false;
                    }
                },
                BitsPatternPart::Zero => {
                    if (bits >> T::from(i_u8)) & one != zero {
                        return false;
                    }
                },
                BitsPatternPart::DontCare => (),
            }
        }
        true
    }
}