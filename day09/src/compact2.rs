#![allow(dead_code, unused)]

type Disk = Vec<Option<u16>>;

#[derive(Clone)]
pub struct File {
    id: u16,
    blocks: usize,
    free: usize,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct FileMap {
    data: Vec<File>,
    first : Option<usize>,
    last : Option<usize>,
}

impl FileMap {
    fn new() -> FileMap {
        FileMap { data: Vec::new (), first: None, last: None }
    }
    fn with_capacity(capacity: usize) ->FileMap {
        FileMap {data: Vec::with_capacity(capacity), first: None, last: None}
    }
    fn push( &mut self, id: u16, blocks: usize, free: usize ) {
        if self.first.is_none() {
            self.data.push( File { id, blocks, free, prev: None, next: None });
            self.first = Some(0);
            self.last = Some(0);
        }
        else {
            let last = self.last.unwrap();
            self.data[last].next = Some(self.data.len());
            self.data.push(File {id, blocks, free, prev: Some(last), next: None});
            self.last = Some(self.data.len()-1);
        }
    }
    fn move_after(&mut self, after: usize, location: usize, id: u16, blocks: usize ) {
        
    }

}

pub fn parse_input(input: &str) -> FileMap {
    let length = input.len();
    let mut file_map = Vec::with_capacity(length);
    let mut id = 0u16;
    let mut bytes = input.bytes();

    loop {
        let blocks = (bytes.next().unwrap() - b'0') as usize;
        if let Some(free) = bytes.next() {
            let free = (free - b'0') as usize;
            file_map.push(File { id, blocks, free });
            id += 1;
        } else {
            file_map.push(File {
                id,
                blocks,
                free: 0,
            });
            break;
        }
    }
    file_map
}

pub fn compact_disk(file_map: &FileMap) -> FileMap {
    let mut new_file_map = file_map.clone();
    let mut i = 0_usize;
    let mut j = file_map.len() - 1;
    loop {
        let mut i = 0;
        for 
        while i < j && new_file_map[i].is_some() {
            i += 1;
        }
        while i < j && new_file_map[j].is_none() {
            j -= 1;
        }
        if i >= j {
            break;
        }
        new_file_map[i] = new_file_map[j];
        new_file_map[j] = None;
        i += 1;
        j -= 1;
    }
    new_file_map
}

pub fn checksum(disk: &Disk) -> usize {
    let mut sum = 0;
    for (i, &c) in disk.iter().enumerate() {
        match c {
            Some(v) => sum += i * (v as usize),
            None => break,
        }
    }
    sum
}

fn disk2string(disk: &Disk) -> String {
    disk.iter().fold(String::new(), |mut acc, x| {
        match x {
            Some(id) => acc.push_str(&id.to_string()),
            None => acc.push('.'),
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_input() {
        let input = "2333133121414131402";
        let disk = super::parse_input(input);
        //assert_eq!(disk.len(), 20);
        let disk_str = super::disk2string(&disk);
        println!("{:?}", disk_str);
        assert_eq!(disk_str, "00...111...2...333.44.5555.6666.777.888899");
        let compact_disk = super::compact_disk(&disk);
        let compact_disk_str = super::disk2string(&compact_disk);
        println!("{:?}", compact_disk_str);
        assert_eq!(
            compact_disk_str,
            "0099811188827773336446555566.............."
        );
        let checksum = super::checksum(&compact_disk);
        println!("{checksum}");
        assert_eq!(checksum, 1928);
    }
}
