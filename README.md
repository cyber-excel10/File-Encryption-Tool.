# Project Name: Excel File Encryption Tool.

** A command-line tool for encrypting and decrypting files using AES encryption (AES-128 and AES-256). This tool allows users to either provide a key manually or generate a random 128-bit key for encryption/decryption purposes.

# Basic Features Of This Tool Are:
* Reads a File to Encrypt: The tool takes an input file (e.g., input.txt) that you want to encrypt.
* Uses AES Encryption: It applies the AES (Advanced Encryption Standard) encryption algorithm to the content of the file. AES is a widely-used encryption method that secures data.
. AES-128 or AES-256: The tool supports two key lengths for AES encryption:
. AES-128: Requires a 16-byte key.
. AES-256: Requires a 32-byte key.
* Random Initialization Vector (IV): It uses a random IV (Initialization Vector) to ensure the encryption is unique each time, even if the same key is used. This adds an extra layer of security, especially for CBC (Cipher Block Chaining) mode.
* Encrypts the File: It encrypts the content of the input file and generates an encrypted version, which it saves to a new file (e.g., output.enc).
* Supports Command-Line Interface (CLI): The tool allows you to specify the input file, output file, and encryption key through the command line. The encryption key is expected to be provided in hexadecimal format.
* You Have Option To Generate a Random 128-bit key for encryption or decryption.

# Requirements
* Please Ensure You Have Rust Programming Language Installed...
* Installation
# Clone this repository:
* git clone https://github.com/yourusername/excel_file_encryption_tool.git
* cd excel_file_encryption_tool
* 
# To Build the project:
* Run cargo build
* If you want to run the tool without building the project manually, use cargo run directly.(optional)..

# Purpose Of This Tool..
* Encrypt a file:To encrypt a file, you need to specify the input file, output file, action (encrypt), and either provide a key using the --key option or generate a random key using --generate-key.

cargo run -- <input_file> <output_file> --action encrypt --key <encryption_key>

* The tool reads the input file and processes the data in chunks of 16 bytes (block size for AES).
* It uses AES encryption to encrypt the data with the provided key.
* The encrypted data is then written to the output file.
Decrypt a file:The tool reads the encrypted file and processes the data in chunks of 16 bytes.
* It uses AES decryption to decrypt the data with the provided key.
* The decrypted data is then written to the output file.

* Encrypt a File: Once you have the key (either generated or provided), you can encrypt a file with the --action encrypt option.

* Decrypt a File: After encryption, use the same key to decrypt the file with the --action decrypt option.

# Error Handling
* The tool will display an error message if:
1. The key length is invalid (not 16 or 32 bytes).
2. The --key or --generate-key options are not used correctly.
3. Any file operation (read/write) fails.
* For now currently working on updating the Code... Feel Free to Add more features to the code and update the code fully....

