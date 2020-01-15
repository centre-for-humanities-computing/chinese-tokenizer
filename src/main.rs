// Implemetation of jieba-rs on folder of Chinese texts
// 
// Author:  rdkm
// Date:    2020-01-13
//
// Built using the jieba-rs port written by messense
// https://github.com/messense/jieba-rs
//
//
// For future use:
//
// All .txt files need to be in a folder called 'data'.
// Glob only goes one directory down.
// Output folder in root directory called 'out'
// Before use, check model parameter in line 53.
//
// TODO: Main function can be split into individual functions.


extern crate glob;

use glob::glob;
use std::{io,fs,time};
use jieba_rs::Jieba;


fn main() -> io::Result<()> {

    // match input directories in output directory
    for folder in glob("data/**").unwrap() {
        
        // Unwrap glob path to formatted string
        let directory = format!("out/{}", &folder.unwrap().display());
        // Create directory if it doesn't already exist
        fs::create_dir_all(directory)?;
    }

    // loop over novels
    for entry in glob("data/**/*.txt").unwrap() {
        
        // Initialize Jieba and set a timer
        let now = time::Instant::now();
        let jieba = Jieba::new();

        // set filepath reference
        let filepath = &entry.unwrap();
        
        // read novel to string
        let contents =fs::read_to_string(&filepath).expect("Unable to read! ");
       
        // Jieba takes Vec<&str>
        let lines: Vec<&str> = contents.split('\n').collect();
        
        // empty Vec for tokenised text
        let mut tokens: Vec<&str> = Vec::new(); 
       
        // for line in novel, tokenize using Jieba
        for line in lines {
            // the boolean is which tokenizer to use
                // True = Hidden Markov Model
                // False = MMSEG
            let mut v = jieba.cut(line, true);
            tokens.append(&mut v); 
       }    
       
        // Vec<&> back to String
        // Note that this results in whitespace seperated text.
        // You could change this to .join(",") for comma seperated.
        let vs: String = tokens.join("  "); 
        
        // Make sure to set outpath to match your output directory
        // For this particular project, make sure to write to the correct
        // folder based on whether or not using MMSEG or HMM
        let outpath = format!("out/{}", &filepath.display());
        fs::write(outpath, vs).expect("Unable to write!");
       
        // print running time per tokenized file
        println!("File tokenized in {}ms", now.elapsed().as_millis());
   }
   Ok(())
}
