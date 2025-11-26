/*
*   UTF8sten_osidised gives tools to store data in unicode symbols
*   Copyright (C) 2025  11mushroom
*
*   This program is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   This program is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::io::{Write,Read};

const BUFF_SIZE:usize=510;

fn main() {
    let args:Vec<String>=std::env::args().collect();
    let mut stdout=std::io::stdout();

    let force_lossy_decode=false;

    if args.len()<2 {
      let mut stdin=std::io::stdin();

      let mut buff:[u8;BUFF_SIZE]=[0;BUFF_SIZE];
      loop{
        let mut read_len:usize = 0;
        loop { 
          read_len += match stdin.read(&mut buff[read_len..]){
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => panic!("wtf {}", e)
          };
          if read_len>=BUFF_SIZE {
            break
          }
        }
        //eprintln!("read {} bytes", read_len);


        let codepoints:Vec<u32>=match String::from_utf8(Vec::from(&buff[..read_len])) {
            Ok(s) => s.chars().map(|c| c as u32).collect::<Vec<u32>>(),
            Err(e) => if force_lossy_decode {
                        String::from_utf8_lossy(&buff[..read_len]).chars().map(|c| c as u32).collect::<Vec<u32>>()
                      } else {
                        println!("failed to convert raw bytes into ecceptable for decoder format");
                        println!("you can try to enable forcing convertion,\nwhich may result in some DATA LOSSES OR CORRUPTIONS");
                        println!("to enable it, change value of `force_lossy_decode` variable to `true`");
                        panic!("{}", e);
                      }
          };

        let result:Vec<u8> = UTF8::deSten(&codepoints);
        let _ = stdout.write_all(&result);

        if read_len<BUFF_SIZE {
          break
        }

      }
      println!("");

    } else {
      let destenned:Vec<u8>=UTF8::deSten(&args[1].chars().map(|c| c as u32).collect::<Vec<u32>>());
      let _ = stdout.write_all(&destenned);
      println!("");
    }
}
