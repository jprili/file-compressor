use std::fs::File;
use crate::tree::{ Node };

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
 * 2. Write tree to file.  3. Write encoded bits to file.
 *
 * @param file
 * @param target
 */
fn compress(file: &File, target: &String) -> () {

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
fn _build_tree(file: &File) -> Node {
   Node::new(None)
}
