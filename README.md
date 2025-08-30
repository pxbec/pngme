# PNGme

This is my own implementation of [PNGme](https://jrdngr.github.io/pngme_book/introduction); a CLI-program for hiding secret messages in PNG images.
It shouldn't be used for anything serious, though.

## Installation

locally:
```shell
cargo install --path .
```

or remotely:
```shell
cargo install --git https://github.com/pxbec/pngme/
```

## Usage

To add a secret message to an image:
```shell
pngme encode -i ./my_image.png RuST "This is a secret message!"
```

To decode a secret message from an image:
```shell
pngme decode -i ./my_image.png RuST
```

To remove a secret message from an image:
```shell
pngme remove -i ./my_image.png RuST
```

To print an overview of the chunks embedded in an image:
```shell
pngme print -i ./my_image.png
```