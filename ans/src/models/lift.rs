
use crate::effects::*;
use crate::mem::*;



pub struct Lifter {
}
impl Lifter {
}


#[cfg(test)]
mod test {
    use crate::models::lift::*;
    use crate::mem::*;
    #[test]
    fn test() {
        let mut mem = Memory::new(0x0010_0000);
        //let entrypt = mem.load_elf("./rv32/test.elf");
    }
}
