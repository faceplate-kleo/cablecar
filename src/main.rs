use uuid::Uuid;

pub mod mmap;

use mmap::*;

pub struct Cable {
    id: Uuid,
    state: u8,
}

impl Cable {
    pub fn new() -> Self {
        Cable {
            id: Uuid::new_v4(),
            state: 0,
        }
    }

    pub fn read_state(&self) -> u8 {
        self.state
    }

    pub fn write_state(&mut self, value: u8) {
        self.state = value;
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}

pub struct Gameboy {
    id: Uuid,
    link_port: Option<*mut Cable>,
    memory: [u8; MAX_RAM + 1],
}

impl Gameboy {
    pub fn new() -> Self {
        Gameboy {
            id: Uuid::new_v4(),
            link_port: None,
            memory: [0; MAX_RAM + 1],
        }
    }

    pub fn dump_memory(&self) {
        let mmap = MemoryMap::new();
        mmap.dump_memory(self.memory)
    }

    pub fn dump_io_region(&self) {
        let mmap = MemoryMap::new();
        mmap.dump_io(self.memory)
    }

    pub fn link_cable(&mut self, cable: *mut Cable) {
        self.link_port = Some(cable);
    }

    pub fn link_is_connected(&self) -> bool {
        match self.link_port {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn send(&self, outgoing: u8) -> Result<u8, ()>{
        match self.link_port {
            Some(mut link) => {
                unsafe { // TODO i hate unsafe
                    link.as_mut()
                        .expect("NO CABLE")
                        .write_state(outgoing);
                    Ok(outgoing)
                }
            }
            None => Err(())
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<u8, ()> {
        match address {
            addr if address <= MAX_RAM as u16 => {
                let old = self.memory[addr as usize];
                self.memory[addr as usize] = value;
                Ok(old)
            },
            _ => Err(()),
        }
    }

    pub fn read_byte(&mut self, address: u16) -> Result<u8, ()> {
        match address {
            addr if address < MAX_RAM as u16 => Ok(self.memory[addr as usize]),
            _ => Err(()),
        }
    }

    pub fn read_cable(&mut self) -> Result<u8, ()>{
        if self.link_is_connected() {
            unsafe { // TODO i hate unsafe
                let data = self.link_port.unwrap().as_ref().expect("NO CABLE").read_state();
                return self.write_byte(SERIAL_START as u16, data)
            }
        }
        Err(())
    }

    pub fn tick(&mut self) {
        let mode = self.read_byte(SERIAL_END as u16).unwrap();
        if mode == 0xFF {
            let mut curr = self.read_byte(SERIAL_START as u16).unwrap();
            let next = match curr {
                u8::MAX => 0,
                _ => curr + 1 // TODO in a perfect world this value should, y'know... mean something
            };
            self.write_byte(SERIAL_START as u16, next).unwrap();
            self.send(next).expect("GENERAL SEND FAILURE");

        } else {
            match self.read_cable() {
                Ok(_) => {}
                Err(_) => {
                    println!("GENERAL READ FAILURE")
                }
            }
        }
    }
}

fn main() {
    let mut gb_a = Gameboy::new();
    let mut gb_b = Gameboy::new();

    let mut link_cable = Cable::new();

    gb_a.link_cable(&mut link_cable);
    gb_b.link_cable(&mut link_cable);

    match gb_a.write_byte(SERIAL_END as u16, 0xFF) {
        Ok(_) => println!("set A boot rom to serial master"),
        Err(_) => println!("aw sheit"),
    }

    match gb_b.write_byte(SERIAL_END as u16, 0x01) {
        Ok(_) => println!("set B boot rom to serial slave"),
        Err(_) => println!("aw sheit"),
    }

    let ticks = 4;
    for _ in 0..ticks {
        gb_a.tick();
        gb_b.tick();
        println!("{}", link_cable.read_state());
    }
    println!("A");
    gb_a.dump_io_region();
    println!("B");
    gb_b.dump_io_region();




}
