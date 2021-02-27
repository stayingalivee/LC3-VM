use crate::defs::memory::*;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_image_file(memory: &mut Memory, image_path: str) {
    let mut buffer = Vec::new();
    File::open(image_path)?.read_to_end(&mut buffer)?;

    if let Ok(instructions) = get_instr_from_buffer(&buffer){
        for instr in instructions.clone(){
            print_instr(v);
        }
    }
}

fn get_instr_from_buffer(data: &[u8]) -> Result<Vec<u16>, Error> {
    if data.len() % 2 != 0 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "input must be a multiple of 2",
        ));
    }
    Ok(data
        .chunks(2)
        .map(|x| x[1] as u16 | (x[0] as u16) << 8)
        .collect())
}

fn print_instr(x: u16) {
    let mut number = x.clone();
    let mut i = 16;
    while i > 0 {
        let bit = (number & 0b1000000000000000) >> 15;
        print!("{}", bit);
        number = number << 1;
        i -= 1;
    }
    println!("");
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loading_image_file(){
        let max: usize = 65535;
        let mut memory = Memory::new(max);
        let image_path: str = "./halt.obj";
        read_image_file(&memory, image_path);
        
    }
}
