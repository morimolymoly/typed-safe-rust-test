mod safe;

#[allow(unused)]
fn main() {
    println!("Hello, world!");

    let b = safe::get_disabled_buffer();
    // b.input();
    // b.output();
    let bi = safe::get_input_buffer();
    bi.input();
    //bi.output();

    let bo = safe::get_output_buffer();
    bo.output();
    //bo.input();

    {
        let bo = safe::Buffer{
            enabled: safe::ENABLED,
            direction: safe::NOWAY,
        };
        //bo.input();
        //bo.output();
    }
}
