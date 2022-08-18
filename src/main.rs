
mod lib;

use crate::lib::Files::list_files;
use crate::lib::Crypto::{decrypt, encrypt};
use std::error::Error;
use std::path::PathBuf;
use std::fs;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let key:Vec<u8> = Vec::from("fiaoltirhadftolp");
    let path = PathBuf::from("C:\\Users\\Administrator\\Desktop\\testcode\\test");

    let files_path = list_files(path);

    for i in files_path{
        if i.is_file(){
            let mut temp = fs::read(i.clone()).expect("文件讀取失敗");
            // temp = encrypt(&key,&temp).expect("加密失敗");
            temp = decrypt(&key,&temp).expect(" ");
            fs::write(i.clone(), &mut temp).expect("");
        };
    }

    Ok(())
}
