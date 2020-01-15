# Tokenizer for Chinese texts #

This is a short program for tokenizing Chinese text, using a Rust port of jieba.

The default tokenizer is a maximum likelihood matching algorithm working from a Chinese lexicon (i.e. dictionary-based). However, jieba-rs also implements a Hidden Markov Model tokenizer. The preferred tokenizer can be easily selected by making the necessary changes in src/main.rs.

## Getting started

In order to run on your machine, you'll need to first install Rust and the Cargo package manager. This is done a number of different ways, depending on whether you use macOS, Linux, or Windows. Use Google to find out the easiest way to do this for your machine.

Once that's completed, you'll need to copy your data into the empty 'data' folder. Note that the current structure of this program only allows for folder structures one level deep. In other words:

``` 
data/subfolder/file.txt
```

Be sure to check the comments at the beginning of src/main.rs. Some paths and variables may need to be modified to suit your needs.

## Building the program

With Rust, you have two options when running the program. Firstly, you can simply do the following in the root directory:

```
cargo run --release
``` 

This builds the local package and executes the binary. However, you can also run these steps seperately. 

First build:

```
cargo build --release
```

Then run:
```
./target/release/chinese
```

Note that in both cases, we're using the --release flag. This prompts the compiler to perform optimisations which substantially improve performance of the tokenizer.

## NB

This was written quite quickly to solve a specific problem and is still essentially work-in-progress. It will work for any collection of Chinese texts, as long as the corpus structured in the format outlined above. However, I hope at some point to return to this and make it more flexible, as well as offering the user the chance to set certain flags. 


## Author
Author:		[rdkm89](https://github.com/rdkm89)
Date:		2020-01-13

## Built with

This tokenizer pipeline is dependent on _jieba-rs_ by Github user [messense](https://github.com/messense). The original repo for that project can be found [here](https://github.com/messense/jieba-rs)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

 
