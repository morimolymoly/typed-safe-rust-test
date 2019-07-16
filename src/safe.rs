pub struct Buffer<EN, DIR>{
    pub enabled: EN,
    pub direction: DIR,
}

pub struct ENABLED;
pub struct DISABLED;
pub struct INPUT;
pub struct OUTPUT;
pub struct NOWAY;

impl<EN, DIR> Buffer<EN, DIR> {
}

impl Buffer<ENABLED, INPUT> {
    pub fn input(self) {
        println!("input!")
    }
}

impl Buffer<ENABLED, OUTPUT> {
    pub fn output(self) {
        println!("output!")
    }
}

pub fn get_disabled_buffer() ->  Buffer<DISABLED, NOWAY> {
    Buffer {
        enabled: DISABLED,
        direction: NOWAY,
    }
}

pub fn get_input_buffer() -> Buffer<ENABLED, INPUT> {
    Buffer {
        enabled: ENABLED,
        direction: INPUT,
    }
}

pub fn get_output_buffer() -> Buffer<ENABLED, OUTPUT> {
    Buffer {
        enabled: ENABLED,
        direction: OUTPUT,
    }
}
