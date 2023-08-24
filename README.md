# pngme
This is the full implementation for https://picklenerd.github.io/pngme_book/introduction.html. A CLI program that let's you hide secret messages in PNG files.

# Usage Example

# Encode a message
```console
$ cargo run -- encode <file path> <key> <message> [output file]
Message encoded successfully!
```
# Decode hidden message
```console
$ cargo run -- decode <file path> <key>
Message: <message>
```
# Print the PNG file
```console
$ cargo run -- print <file path>
header: [137, 80, 78, 71, 13, 10, 26, 10]  chunks: [Chunk { length: 13, chunk_type: [73, 72, 68, 82], chunk_data: [0, 0, 3, 189, 0, 0, 2, 88, 8, 6, 0, 0, 0], crc: 2921562409 }, ...
```
# Remove hidden message
```console
$ cargo run -- remove <file path> <key>
Message has been removed successfully!
```
