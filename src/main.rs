use std::error::Error;
use std::fs::{self};
use std::io::{Read, Write};
use std::process::exit;
use anyhow::Result;
use aes_gcm::aead::Aead;
use base64::{decode, encode};
use cryptokeep_cli::{ui, dao::Dao};
use console::style;


fn main() -> Result<()> {
    // let dao: Dao;
    // let db_response = Dao::init();
    //
    // match db_response {
    //     Ok(d) => { dao = d }
    //     Err(e) => {
    //         println!("Error occurred while initializing database, err: {:?}", e);
    //         exit(0)
    //     }
    // }
    //
    // loop {
    //     ui::start();
    //
    //     println!("{}", style("\nDo you want to continue ? Y / n\n").cyan());
    //     let mut buff = String::new();
    //     std::io::stdin().read_line(&mut buff)?;
    //
    //     if buff == "N\n" || buff == "n\n" {
    //         break;
    //     }
    // }

    use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`
    use aes_gcm::aead::{Aead, NewAead};

    let msg = "Hello World!";

    let key = Key::from_slice(b"an example very very secret key.");
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(b"unique nonce");

    let enc_data = cipher.encrypt(nonce, msg.as_ref()).unwrap();
    // println!("{:?}", enc_data);

    let encoded = encode(enc_data);

    let _c = fs::write("test.key", encoded);

    // let mut fd = File::create("test.key")?;
    // let d = fd.write(enc_data.as_slice())?;
    // println!("{}", d);

    let mut m = fs::read("test.key")?;

    println!("{}", String::from_utf8_lossy(m.as_slice()));
    let decoded = decode(m)?;

    let d_data = cipher.decrypt(nonce, decoded.as_slice()).unwrap();
    //
    // // println!("{}", String::from_utf8_lossy(d_data.as_slice()));
    //
    println!("{}", String::from_utf8_lossy(d_data.as_slice()));

    Ok(())
}
