#![allow(dead_code, unused)]

use std::{
    fs::File,
    io::repeat,
    ops::{Deref, DerefMut},
};

use crate::vector_linked_list::IDRef;

use super::vector_linked_list::{LLError, LLNode, VLL};

type Disk = Vec<Option<u16>>;
type BlocksSize = u32;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct FileSize {
    blocks: BlocksSize,
    free_blocks: BlocksSize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileMap {
    nodes: VLL<FileSize>,
}

impl FileMap {
    fn new() -> FileMap {
        FileMap { nodes: VLL::new() }
    }
    fn with_capacity(capacity: usize) -> FileMap {
        FileMap {
            nodes: VLL::with_capacity(capacity),
        }
    }
    fn push(&mut self, blocks: BlocksSize, free: BlocksSize) {
        self.nodes.push_last(FileSize {
            blocks,
            free_blocks: free,
        });
    }
    fn move_after(&mut self, after: usize, id: usize) -> bool {
        let old_prev = self.nodes.nodes[id].prev.unwrap();
        let FileSize {
            blocks,
            free_blocks: free,
        } = self.nodes.nodes[id].data;
        if self.nodes.nodes[after].data.free_blocks < free {
            return false;
        }
        self.nodes.nodes[old_prev].data.free_blocks += blocks + free;
        self.nodes.nodes[id].data.free_blocks = self.nodes.nodes[after].data.free_blocks - blocks;
        self.nodes.nodes[after].data.free_blocks = 0;
        self.nodes.move_after(after, id).unwrap();
        true
    }
}

pub fn parse_input(input: &str) -> FileMap {
    let length = input.len();
    let mut file_map = FileMap::with_capacity(length);
    let mut id = 0_usize;
    let mut bytes = input.bytes();

    loop {
        let blocks = (bytes.next().unwrap() - b'0') as BlocksSize;
        if let Some(free_blocks) = bytes.next() {
            let free_blocks = (free_blocks - b'0') as BlocksSize;
            file_map.push(blocks, free_blocks);
        } else {
            file_map.push(blocks, 0);
            break;
        }
    }
    file_map
}

pub fn compact_disk(file_map: &FileMap) -> FileMap {
    let mut new_file_map = file_map.clone();
    let mut target_block: IDRef = None;
    for old_block in file_map.nodes.iter_rev() {
        target_block = None;
        for possible_block in new_file_map.nodes.iter() {
            if possible_block.data.free_blocks >= old_block.data.blocks {
                target_block = Some(possible_block.id);
                break;
            }
        }
        if let Some(target_block) = target_block {
            new_file_map.move_after(target_block, old_block.id);
        }
    }
    new_file_map
}

pub fn checksum(file_map: &FileMap) -> BlocksSize {
    fn triangular_number(n: BlocksSize) -> BlocksSize {
        (n * (n + 1)) >> 1
    }
    let mut sum = 0;
    let mut position = 0;
    for LLNode {
        id,
        prev: _,
        next: _,
        data: FileSize {
            blocks,
            free_blocks,
        },
    } in file_map.nodes.iter()
    {
        sum += BlocksSize::try_from(*id).unwrap()
            * (triangular_number(position + blocks) - triangular_number(position));
        position += blocks + free_blocks;
    }
    sum
}

fn disk2string(file_map: &FileMap) -> String {
    let ret = file_map
        .nodes
        .iter()
        .fold(Vec::<u8>::new(), |mut acc: Vec<u8>, x| {
            acc.extend(std::iter::repeat(x.id as u8 + b'0').take(x.data.blocks as usize));
            acc.extend(std::iter::repeat(b'.').take(x.data.free_blocks as usize));
            acc
        });
    String::from_utf8(ret).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_input() {
        let input = "2333133121414131402";
        let file_map = super::parse_input(input);
        //assert_eq!(disk.len(), 20);
        let disk_str = super::disk2string(&file_map);
        println!("{:?}", disk_str);
        assert_eq!(disk_str, "00...111...2...333.44.5555.6666.777.888899");
        let compact_disk = super::compact_disk(&file_map);
        let compact_disk_str = super::disk2string(&compact_disk);
        println!("{:?}", compact_disk_str);
        assert_eq!(
            compact_disk_str,
            "00992111777.44.333....5555.6666.....8888.."
        );
        let checksum = super::checksum(&compact_disk);
        println!("{checksum}");
        assert_eq!(checksum, 2858);
    }
}
