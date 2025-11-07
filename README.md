# Firewire's Image encoding
FWIE is a simple command line based tool for encoding and decoding data in and out of image files. It was originally created as a small CTF style challenge for a few friends, however I have found a few niche cases for it outside of its original use case.

## Installation
1. Make sure you have a working rust toolchain, checkout rustup if you need one.
2. Clone and cd into this repo
3. Run `cargo build --release`

After compiling, you can use the binary located in target/release/image-encoding as is, or copy into a directory in your path

## Usage
Encoding and decoding are both supported.

### Encoding data
To encode information into an image, you can run the following command. Replacing `<input>` with the path to your input file and `<output>` with the path you want your image saved as.

```image-encoding encode -i <input> -o <output> encode```

For example,

```image-encoding -i bee-movie-script.txt -o bees.png encode```

Will encode all information in the `bee-movie-script.txt` file into `bees.png`

## Decoding
Decoding is much the same, just with an image encoded with FWIE as the input and your decoded file as the output.

```image-encoding -i <input> -o <output> decode```

For example,

```image-encoding -i bees.png -o bee-movie-script.txt decode```

Will decode all information in the `bees.png` image into `bee-movie-script.txt`


## Features
- Compression: GZ compression provided by the flate2 crate allows for a very noticable size reduction in encoded image files. You likely will not have a reduction in file size compared to the original, but without it encoded images like to be rather large.
- Error correction: There is a fair bit of error correction built in through the use of reed solomon error correction, as well as some trickery with how pixel values are read in during decoding.
- Data input: Theoretically any type of file can be read in, I've managed to encode everything from the bee movie script and simple a "hello" in a txt file to large multi-gigabyte 3d scans (probably best not to ask)

## License
This project is licensed under GNU GPLv3. See the LICENSE file for more details.

## Contributions
All contributions are welcome. Feel free to open an issue or submit a pull request if you find any bugs or have any suggestions.
