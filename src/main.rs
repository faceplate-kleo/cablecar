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

    pub fn read(&self) -> u8 {
        self.state
    }

    pub fn write(&mut self, value: u8) {
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

    pub fn write_byte(&mut self, address: u16, value: u8) -> Result<u8, ()> {
        println!("TICK: {}", address);
        println!("TICK: {}", MAX_RAM);
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

    pub fn tick(&mut self) {}
}

fn main() {
    let mut gb_a = Gameboy::new();

    match gb_a.write_byte(SERIAL_START as u16, 0xFF) {
        Ok(_) => println!("Boot rom written"),
        Err(_) => println!("aw sheit"),
    }

    gb_a.dump_memory();
    gb_a.dump_io_region();
}
