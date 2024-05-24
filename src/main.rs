use std::io::Write;

use isa::ISA;
use rand::Rng;

mod bitspattern;
use bitspattern::BitsPattern;

mod isa;

fn main() {
    let instrs_num = 10;
    let output_path = std::path::Path::new("test.bin");
    let generator = isa::LAS32R::new();
    let candidate_instrs = generator.support_instrs();

    let mut f = std::fs::File::create(&output_path).unwrap();
    let mut rng = rand::thread_rng();
    for _ in 1..instrs_num{
        let i = rng.gen_range(0..candidate_instrs.len());
        let bits = generator.generate_instr(candidate_instrs[i]).unwrap();
        println!("{} {:#b}", candidate_instrs[i], bits);
        f.write_all(&bits.to_le_bytes()).unwrap();
    }
}
