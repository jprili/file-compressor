/*
 * Module to compress and decompress a file.
 *
 * Uses basic Huffman encoding to compress.
 */
use std::env;
use std::process::ExitCode;

mod parse;

const ARGUMENT_NUM: usize = 4;
const ARG_START: usize = 1;
const ARG_END:   usize = 3;

/*
 * Usage:
 *  compress filename.extension compressed
 *  decompress compressed filename.extension
 */
fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != ARGUMENT_NUM {
        eprintln!("Usage:");
        eprintln!("    file-compressor [de]compress <source> <target>");
        ExitCode::FAILURE
    } else {
        let mode:   &String = &args[ARG_START];
        let source: &String = &args[ARG_START + 1];
        let target: &String = &args[ARG_END];
        parse::parse(mode, source, target);
        ExitCode::SUCCESS
    }
}
