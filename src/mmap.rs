use std::collections::HashMap;

pub const MAX_RAM: usize = (64 * 1024) - 1;

// REGIONS

pub const ROM_00_START: usize = 0x0;
pub const ROM_00_END: usize = 0x3FFF;

pub const ROM_01_START: usize = 0x4000;
pub const ROM_01_END: usize = 0x7FFF;

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9FFF;

pub const EXRAM_START: usize = 0xA000;
pub const EXRAM_END: usize = 0xBFFF;

pub const WRAM_START: usize = 0xC000;
pub const WRAM_END: usize = 0xDFFF;

pub const ECHO_RAM_START: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;

pub const OAM_START: usize = 0xFE00;
pub const OAM_END: usize = 0xFE9F;

pub const RESERVED_START: usize = 0xFEA0;
pub const RESERVED_END: usize = 0xFEFF;

pub const IO_START: usize = 0xFF00;
pub const IO_END: usize = 0xFF7F;

pub const HRAM_START: usize = 0xFF80;
pub const HRAM_END: usize = 0xFFFE;

pub const IE: usize = 0xFFFF;

// IO RANGES

pub const JOYPAD: usize = 0xFF00;
pub const SERIAL_START: usize = 0xFF01;
pub const SERIAL_END: usize = 0xFF02;
pub const TIMER_START: usize = 0xFF04;
pub const TIMER_END: usize = 0xFF07;
pub const INTERRUPTS: usize = 0xFF0F;
pub const AUDIO_START: usize = 0xFF10;
pub const AUDIO_END: usize = 0xFF26;
pub const WAVE_START: usize = 0xFF30;
pub const WAVE_END: usize = 0xFF3F;
pub const LCD_START: usize = 0xFF40;
pub const LCD_END: usize = 0xFF4B;
pub const VRAM_BANK_SELECT: usize = 0xFF4F;
pub const DISABLE_BOOT_ROM: usize = 0xFF50;
pub const VRAM_DMA_START: usize = 0xFF51;
pub const VRAM_DMA_END: usize = 0xFF55;
pub const BJ_OBJ_START: usize = 0xFF68;
pub const BJ_OBJ_END: usize = 0xFF6B;
pub const WRAM_BANK_SELECT: usize = 0xFF70;

pub struct MemoryMap {
    region_names: HashMap<usize, &'static str>,
    io_range_names: HashMap<usize, &'static str>,
    region_sizes: HashMap<&'static str, usize>,
}

impl MemoryMap {
    pub fn new() -> Self {
        MemoryMap {
            region_names: HashMap::from([
                (ROM_00_START, "ROM_00_START"),
                (ROM_01_START, "ROM_01_START"),
                (VRAM_START, "VRAM_START"),
                (EXRAM_START, "EXRAM_START"),
                (WRAM_START, "WRAM_START"),
                (ECHO_RAM_START, "ECHO_RAM_START"),
                (OAM_START, "OAM_START"),
                (RESERVED_START, "RESERVED_START"),
                (IO_START, "IO_START"),
                (HRAM_START, "HRAM_START"),
                (IE, "IE"),
            ]),
            io_range_names: HashMap::from([
                (JOYPAD, "JOYPAD"),
                (SERIAL_START, "SERIAL_START"),
                (TIMER_START, "TIMER_START"),
                (INTERRUPTS, "INTERRUPTS"),
                (AUDIO_START, "AUDIO_START"),
                (WAVE_START, "WAVE_START"),
                (LCD_START, "LCD_START"),
                (VRAM_BANK_SELECT, "VRAM_BANK_SELECT"),
                (DISABLE_BOOT_ROM, "DISABLE_BOOT_ROM"),
                (VRAM_DMA_START, "VRAM_DMA_START"),
                (BJ_OBJ_START, "BJ_OBJ_START"),
                (WRAM_BANK_SELECT, "WRAM_BANK_SELECT"),
            ]),
            region_sizes: HashMap::from([
                ("ROM_00_START", ROM_00_END - ROM_00_START + 1),
                ("ROM_01_START", ROM_01_END - ROM_01_START + 1),
                ("VRAM_START", VRAM_END - VRAM_START + 1),
                ("EXRAM_START", EXRAM_END - EXRAM_START + 1),
                ("WRAM_START", WRAM_END - WRAM_START + 1),
                ("ECHO_RAM_START", ECHO_RAM_END - ECHO_RAM_START + 1),
                ("OAM_START", OAM_END - OAM_START + 1),
                ("RESERVED_START", RESERVED_END - RESERVED_START + 1),
                ("IO_START", IO_END - IO_START + 1),
                ("HRAM_START", HRAM_END - HRAM_START + 1),
                ("IE", 1),
                ("SERIAL_START", SERIAL_END - SERIAL_START + 1),
                ("TIMER_START", TIMER_END - TIMER_START + 1),
                ("AUDIO_START", AUDIO_END - AUDIO_START + 1),
                ("WAVE_START", WAVE_END - WAVE_START + 1),
                ("LCD_START", LCD_END - LCD_START + 1),
                ("VRAM_DMA_START", VRAM_DMA_END - VRAM_DMA_START + 1),
                ("BJ_OBJ_START", BJ_OBJ_END - BJ_OBJ_START + 1),
            ])
        }
    }

    pub fn dump_memory(&self, memory: [u8; MAX_RAM+1]) {
        let threshold = 256;
        for i in 0..MAX_RAM+1 {
            let mut size: usize = 0;
            let name ;
            if self.region_names.contains_key(&i) {
                name = self.region_names.get(&i).unwrap();
                if !name.contains("_END") {
                    size = self.region_sizes.get(name).unwrap_or_else(|| &1usize).to_owned();
                }
            } else if self.io_range_names.contains_key(&i) {
                name = self.io_range_names.get(&i).unwrap();
                if !name.contains("_END") {
                    size = self.region_sizes.get(name).unwrap_or_else(|| &1usize).to_owned();
                }
            } else {
                continue
            }

           println!("{}", name.replace("_START", ""));

            if size > threshold {
                println!(" [ BEEG ] ");
                continue
            }

           for j in 0..size {
                if j % 8 == 0 {
                    println!();
                }
                print!("{:02X} ", memory[i+j]);
            }

            println!("\n");
        }
    }

    pub fn dump_io(&self, memory: [u8; MAX_RAM+1]) {
        println!("IO DUMP");
        for i in IO_START..IO_END+1 {
            if i % 8 == 0 {
                println!();
            }
            print!("{:02X} ", memory[i]);
        }
        println!();
    }
}