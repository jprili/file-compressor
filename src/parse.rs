use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::io::{ Read, BufReader };

use crate::tree::{ Tree };

/**
 * Parse the arguments.
 *
 * @param mode   Either compress or decompress.
 * @param source The path to the source file.
 * @param target The path to the target file.
 * @panic when there is a problem reading the file.
 */
pub fn parse(mode: &String, source: &String, target: &String) {
    let file = match File::open(source.as_str()) {
        Ok(f)      => f,
        Err(error) => panic!("ERROR: {:?}", error),
    };

    if mode.as_str() == "compress" {
        compress(&file, target);
    }
    decompress(&file, target);
}

/**
 * Compress the source file.
 * 1. Build Huffman tree from bytes.
 * 2. Write tree to file.
 * 3. Write encoded bits to file.
 *
 * @param file
 * @param target
 */
fn compress(file: &File, target: &String) -> () {
    let tree = _build_tree(file);
}

/**
 * Decompress the source file.
 * 1. Parse tree.
 * 2. Create file.
 * 3. Decode bits into file.
 *
 * @param file
 * @param target
 */
fn decompress(file: &File, target: &String) -> () {
}

/**
 * Build a Huffman tree from the file's bytes.
 *
 * @param file
 */
fn _build_tree(file: &File) -> Tree<Option<u8>>{
    let mut tree: Tree<Option<u8>> = Tree::new(None);
    let map = _get_character_frequencies(file);
    tree
}

fn _get_character_frequencies(file: &File) -> HashMap<char, u32> {
    let mut reader = BufReader::new(file); 
    let mut map: HashMap<char, u32>  = HashMap::new();
    let mut bytes      = [0; 1];

    let mut read_bytes: usize = reader
        .read(&mut bytes)
        .ok()
        .unwrap();

    while read_bytes > 0usize {
        if let Some(key) = map.get_mut(&(bytes[0] as char)) {
            *key = *key + 1;
        } else {
            map.insert(
                bytes[0] as char,
                0
            );
        }

        read_bytes = reader.read(&mut bytes).ok().unwrap();
    }

    println!("{:?}", map);
    map
}

