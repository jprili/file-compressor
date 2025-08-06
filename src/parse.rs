use std::io;
use std::fs::File;

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

fn compress(file: &File, target: &String) -> () {

}

fn decompress(file: &File, target: &String) -> () {
}
