pub struct ALU;

#[derive(Debug)]
pub struct ALUResult {
    pub output: i16,
    pub output_is_zero: bool,
    pub output_is_negative: bool,
}

impl ALU {
    pub fn compute(
        x: i16,
        y: i16,
        zx: bool,
        nx: bool,
        zy: bool,
        ny: bool,
        f: bool,
        no: bool,
    ) -> ALUResult {
        let mut x_intermediate = match zx {
            true => 0,
            false => x,
        };
        let mut y_intermediate = match zy {
            true => 0,
            false => y,
        };

        if nx {
            x_intermediate = !x_intermediate;
        }

        if ny {
            y_intermediate = !y_intermediate;
        }

        let mut output = match f {
            true => x_intermediate + y_intermediate,
            false => x_intermediate & y_intermediate,
        };

        if no {
            output = !output
        }

        ALUResult {
            output,
            output_is_zero: output == 0,
            output_is_negative: output < 0,
        }
    }
}
