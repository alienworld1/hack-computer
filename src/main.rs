pub mod alu;

use alu::ALU;

fn main() {
    let x = 0;
    let y = 0b1111111111111111u16 as i16;

    let result = ALU::compute(x, y, true, true, true, false, true, false);
    println!("{result:?}");
}
