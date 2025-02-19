use aes::Aes128;
use aes::cipher::{ BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,generic_array::GenericArray,};
use hex;
use rand::{Rng, rngs::ThreadRng};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use clap::{Arg, Command};
fn main() {
    let matches = Command::new("Excel File Encryption Tool.")
        .version("1.0")
        .author("Joseph <cyber_excel10>")
        .about("Encrypts files using AES encryption")
        .arg(
            Arg::new("input")
                .help("Input file to encrypt")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("Output file to save encrypted content")
                .required(true)
                .index(2),
        )
        .arg(
            Arg::new("key")
                .help("Encryption key (32 bytes for AES-256, 16 bytes for AES-128)")
                .required(false)
                .long("key")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("action")
                .help("Action: 'encrypt' or 'decrypt'")
                .required(true)
                .index(3),
        )
        .arg(
            Arg::new("generate-key")
                .help("Generate a random 128-bit key")
                .long("generate-key")  
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

        let file_inputed = matches.get_one::<String>("input").unwrap();
        let output_file = matches.get_one::<String>("output").unwrap();
    

        let key_hex = if matches.get_flag("generate-key"){
            let random = random_key_generator();
            println!("Generated Key: {}",random);
            random
        } else {
            match matches.get_one::<String>("key"){
                Some(key) => key.to_string(),
                None => {
                    eprintln!("Please provide a key using --key or --generate-key");
                    return;
                }
            }
        };
        let key = hex::decode(key_hex).expect("Failed to decode hex key");

        if key.len() != 16 && key.len() != 32 {
            eprintln!("Invalid Key Length...Please Ensure It must be 16 bytes or 32 bytes");
            return;
        } 

        let action = matches.get_one::<String>("action").unwrap();
        let iv = random_generator();

        if action == "encrypt" {
            if let Err(e) = file_encryption(file_inputed, output_file, &key, &iv) {
                eprintln!("Error: {}", e);
            } else {
                println!("File encrypted successfully!");
            }
        } else if action == "decrypt" {
            if let Err(e) = file_decryption(file_inputed, output_file, &key, &iv) {
                eprintln!("Error: {}", e);
            } else {
                println!("File decrypted successfully!");
            }
        } else {
            eprintln!("Invalid action. Please specify 'encrypt' or 'decrypt'.");
        }
    }
    fn random_generator () -> Vec<u8> {
            let mut random = rand::rngs::ThreadRng::default();
            let iv: Vec<u8> = (0..16).map(|_| random.gen()).collect(); // 128-bit IV
            iv
        }
    fn file_encryption(input_path: &str,output_path: &str, key:&[u8], _iv:&[u8]) -> Result < (), Box<dyn Error>> {

        let mut file_inputed = File::open(input_path)?; // This Section Open The User Inputed File............
        let mut fact = Vec::new();
        file_inputed.read_to_end(&mut fact)?;

        let aes_cipher = Aes128::new(&GenericArray::from_slice(key)); // This Involves The Creation Of The  AES Cipher...
        let mut data_encrypted = Vec::new(); // I created This Bufferv  To Hold The Encrypted Data......

        // let mut aes_block_size = Vec::new(); // This Section Processess The Data Into 16-byte...
        for chunk in fact.chunks(16){

            let mut aes_block = GenericArray::clone_from_slice(chunk); //  This Clones  the chunk into a 16-byte block.
            aes_cipher.encrypt_block(&mut aes_block); // This Performs Encrypting the blocks.
            data_encrypted.extend_from_slice(&aes_block); // It then adds the encrypted block to the list.
        }

        let mut output_file = OpenOptions::new().create(true).write(true).open(output_path)?;
            output_file.write_all(&data_encrypted)?;
        
        Ok(())
    }
fn file_decryption(input_path: &str, output_path: &str, key: &[u8], _iv: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut file_inputed = File::open(input_path)?;
    let mut fact = Vec::new();
    file_inputed.read_to_end(&mut fact)?; 

    let aes_cipher = Aes128::new(&GenericArray::from_slice(key));
    let mut data_decrypted = Vec::new(); // This Is A Buffer To Hold The Decrypted Data Inputed..

 // This Section Processess The Data Into 16-byte...
        for chunk in fact.chunks(16){
            let mut aes_block = GenericArray::clone_from_slice(chunk); //  This Clones  the chunk into a 16-byte block.
            aes_cipher.decrypt_block(&mut aes_block); // This Performs Decrypting the blocks.
            data_decrypted.extend_from_slice(&aes_block) // It then adds the Decrypted block to the list.
        }

  // This Section Performs Writing The Decrypted Data To The Output File....
        let mut output_file = OpenOptions::new().create(true).write(true).open(output_path)?;
            output_file.write_all(&data_decrypted)?;
        Ok(())

    }
    fn random_key_generator() -> String {
        let mut rng = rand::rngs::ThreadRng::default();
        let key: Vec<u8> = (0..16).map(|_| rng.gen()).collect(); // 128-bit key (16 bytes)
        hex::encode(key) // Convert to hexadecimal string
    }