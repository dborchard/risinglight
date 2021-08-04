use crate::storage::block::Block;
use byteorder::LittleEndian;
use positioned_io_preview::{ReadBytesAtExt, WriteBytesAtExt};
pub const PAGE_SIZE: usize = 4096;

pub struct Page {
    content: [u8; PAGE_SIZE],
    block: Option<Block>,
}

impl Page {
    pub fn new(blk: Option<Block>) -> Page {
        Page {
            content: [0; 4096],
            block: blk,
        }
    }
    pub fn get_block(&self) -> Option<Block> {
        self.block.clone()
    }
    pub fn get_mut_content(&mut self) -> &mut [u8] {
        &mut self.content
    }
    pub fn get_content(&self) -> &[u8] {
        &self.content
    }
    pub fn set_int(&mut self, offset: u64, val: i32) {
        self.content
            .as_mut()
            .write_i32_at::<LittleEndian>(offset, val)
            .unwrap();
    }

    pub fn get_int(&self, offset: u64) -> i32 {
        self.content
            .as_ref()
            .read_i32_at::<LittleEndian>(offset)
            .unwrap()
    }

    pub fn set_u64(&mut self, offset: u64, val: u64) {
        self.content
            .as_mut()
            .write_u64_at::<LittleEndian>(offset, val)
            .unwrap();
    }

    pub fn get_u64(&self, offset: u64) -> u64 {
        self.content
            .as_ref()
            .read_u64_at::<LittleEndian>(offset)
            .unwrap()
    }

    pub fn set_string(&mut self, offset: u64, val: String) {
        let size = val.chars().count();
        self.content
            .as_mut()
            .write_u64_at::<LittleEndian>(offset, size as u64)
            .unwrap();
        let vec = val.as_bytes();
        for i in 0..size {
            self.content
                .as_mut()
                .write_u8_at(offset + 8 + (i as u64), vec[i])
                .unwrap();
        }
    }

    pub fn get_string(&self, offset: u64) -> String {
        let size = self
            .content
            .as_ref()
            .read_u64_at::<LittleEndian>(offset)
            .unwrap();
        let mut vec = Vec::<u8>::new();
        for i in 0..size {
            vec.push(
                self.content
                    .as_ref()
                    .read_u8_at(offset + 8 + (i as u64))
                    .unwrap(),
            );
        }
        String::from_utf8_lossy(&vec).to_string()
    }

    pub fn write_u8_vec(&mut self, offset: u64, vec: &Vec<u8>) {
        for i in 0..vec.len() {
            self.content
                .as_mut()
                .write_u8_at(offset + i as u64, vec[i])
                .unwrap();
        }
    }

    pub fn read_u8_vec(&self, start_pos: u64, stop_pos: u64, vec: &mut Vec<u8>) {
        for i in start_pos..stop_pos {
            vec.push(self.content.as_ref().read_u8_at(i).unwrap());
        }
    }
}

#[cfg(test)]
mod page_tests {
    use super::*;

    #[test]
    fn int_read_write() {
        let mut page = Page::new(Some(Block {
            name: "lightdb.bin".to_string(),
            id: 0,
        }));
        page.set_int(10, 20);
        let val = page.get_u64(10);
        assert_eq!(val, 20);
    }

    fn u64_read_write() {
        let mut page = Page::new(Some(Block {
            name: "lightdb.bin".to_string(),
            id: 0,
        }));
        page.set_u64(0, 20);
        let val = page.get_u64(0);
        assert_eq!(val, 20);
    }

    #[test]
    fn string_read_write() {
        let mut page = Page::new(Some(Block {
            name: "lightdb.bin".to_string(),
            id: 0,
        }));
        page.set_string(10, String::from("abcde"));
        let string = page.get_string(10);
        assert_eq!(string, String::from("abcde"));
    }
}
