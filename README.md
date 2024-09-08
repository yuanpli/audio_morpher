# AudioMorpher - Audio Conversion Tool
AudioMorpher is a command-line tool written in Rust for converting audio files in `m4a` format to `mp3` format. It supports specifying input and output directories and provides a progress bar to show the conversion progress. After the conversion is completed, the program will prompt the user to check the files in the output directory.
## Installation
1. Ensure that your system has the Rust compiler installed. If not, visit [Rust official website](https://www.rust-lang.org/) and download and install the Rust toolchain according to your operating system.
2. Clone the AudioMorpher project locally and enter the project directory.
3. In the command line, run `cargo build --release` to compile the project. After successful compilation, an executable file named `audiomorpher` will be generated in the `target\release` directory.
## Usage
1. **Batch conversion of files**: If you need to convert all m4a files in a directory, you can enter in the command line: `audiomorpher input directory`. AudioMorpher will automatically scan all m4a files in the specified directory and convert them to mp3 format. The output directory defaults to the `output` folder in the current directory.
2. **View help**: If you need to view help information, you can enter in the command line: `audiomorpher -h`, which will display the help information of the tool, including usage, options, and descriptions.
3. **Example**:
```bash
(base)  ~/ audiomorpher "~/folder"
[00:00:10] [########################################] 21/21 (100%)
Conversion completed. Please open /Users/output to view the converted files.
```
## Contribution
Welcome to contribute code or provide feedback. If you have any questions or suggestions, please submit an Issue on the GitHub project page or contact me by email.
Welcome to contribute! Please follow these steps:
1. Fork this project
2. Create your feature branch (`git checkout -b feature/YourFeature`)
3. Commit your changes (`git commit -m 'Add some feature'`)
4. Push to the branch (`git push origin feature/YourFeature`)
5. Create a new Pull Request
## License
This project is licensed under the MIT license.
## Copyright notice
Copyright (c) 2024 All rights reserved.