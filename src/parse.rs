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
    let map = _get_character_frequencies(file).clone();
    let tree = _build_tree(
        map.clone(), 
        &Tree::new(None as Option<u32>)
    );
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

// Build a Huffman tree from the file's bytes.
fn _build_tree(
        map: HashMap<usize, u32>,
        tree: &Tree<Option<u32>>
    ) -> Tree<Option<u32>>{
    if (map).is_empty() {
        (*tree).clone()
    } else {
        todo!();
    }
}


// Obtain a histogram of all the character frequencies.
fn _get_character_frequencies(file: &File) -> HashMap<usize, u32> {
    let mut reader = BufReader::new(file); 
    let mut map: HashMap<usize, u32>  = HashMap::new();
    let mut bytes = [0; 1];

    let mut read_bytes: usize = reader
        .read(&mut bytes)
        .ok()
        .unwrap();

    while read_bytes > 0usize {
        let char_key: usize = bytes[0] as usize;
        if let Some(key) = map.get_mut(&char_key) {
            *key = *key + 1;
        } else {
            map.insert(
                char_key,
                0
            );
        }

        read_bytes = reader.read(&mut bytes).ok().unwrap();
    }
    map
}

