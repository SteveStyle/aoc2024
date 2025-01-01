#![allow(dead_code, unused)]

type Disk = Vec<Option<u16>>;

pub fn parse_input(input: &str) -> Disk {
    let length = input.bytes().map(|c| (c - b'0') as usize).sum();
    let mut ret = Vec::with_capacity(length);
    let mut id = 0u16;
    let mut bytes = input.bytes();

    loop {
        let file_size = (bytes.next().unwrap() - b'0') as usize;
        ret.extend(std::iter::repeat(Some(id)).take(file_size));
        if let Some(free_space) = bytes.next() {
            let free_space = (free_space - b'0') as usize;
            ret.extend(std::iter::repeat(None).take(free_space));
            id += 1;
        } else {
            break;
        }
    }
    ret
}

pub fn compact_disk(disk: &Disk) -> Disk {
    let mut new_disk = disk.clone();
    let mut i = 0_usize;
    let mut j = new_disk.len() - 1;
    loop {
        while i < j && new_disk[i].is_some() {
            i += 1;
        }
        while i < j && new_disk[j].is_none() {
            j -= 1;
        }
        if i >= j {
            break;
        }
        new_disk[i] = new_disk[j];
        new_disk[j] = None;
        i += 1;
        j -= 1;
    }
    new_disk
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
