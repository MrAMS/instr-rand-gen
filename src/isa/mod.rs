pub trait ISA {
    type InstrType;
    fn support_instrs(&self) -> &Vec<&'static str>;
    fn generate_instr(&self, instr: &str) -> Option<Self::InstrType>;
}

pub mod la32r;
pub use la32r::LAS32R;

