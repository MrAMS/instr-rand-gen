use std::collections::HashMap;

use super::ISA;
use super::super::BitsPattern;

pub struct LAS32R {
    instrs: Vec<&'static str>,
    pattern_map: HashMap<String, BitsPattern>,
}

impl LAS32R {
    pub fn new() -> Self {
        let patterns = vec![
                        // MSB
            ["ADDW",    "00000000000100000???????????????"],
            ["SUBW",    "00000000000100010???????????????"], 
            ["SLT",     "00000000000100100???????????????"], 
            ["SLTU",    "00000000000100101???????????????"],
            ["NOR",     "00000000000101000???????????????"],
            ["AND",     "00000000000101001???????????????"],
            ["OR",      "00000000000101010???????????????"],
            ["XOR",     "00000000000101011???????????????"],
            ["SLLW",    "00000000000101110???????????????"],
            ["SRLW",    "00000000000101111???????????????"],
            ["SRAW",    "00000000000110000???????????????"],
        ];
        let mut map = HashMap::new();
        for pair in &patterns {
            map.insert(pair[0].to_string(), BitsPattern::new(pair[1]));
        }
        LAS32R{
            instrs: patterns.iter().map(|x| x[0]).collect(),
            pattern_map: map,
        }
    }
}

impl ISA for LAS32R {
    type InstrType = u32;
    fn support_instrs(&self) -> &Vec<&'static str> {
        &self.instrs
    }
    fn generate_instr(&self, instr: &str) -> Option<Self::InstrType> {
        match self.pattern_map.get(&instr.to_string()) {
            None => None,
            Some(i) => Some(i.generate()),
        }
    }
}