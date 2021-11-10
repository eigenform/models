
use ans::models::interp::*;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: interp <ELF file>");
        return;
    }
    let mut vm = Interpreter::new();
    vm.load_elf(&args[1]);
    vm.run();
}
