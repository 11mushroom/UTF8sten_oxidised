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

    if args.len()<2 {
      let mut stdin=std::io::stdin();
      let mut stdout=std::io::stdout();

      let mut buff:[u8;BUFF_SIZE]=[0;BUFF_SIZE];

      loop{
        let mut read_len:usize = 0;

        //this loop ensures that buffer is full except last chunk of data
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
        /*eprintln!("read {} bytes", read_len);*/

        if read_len==0 {
          break
        }

        let result:String = UTF8::enSten(&buff[..read_len]).iter().collect();
        let _ = stdout.write_all(result.as_bytes());

        //quits loop after reaching last chunk of data
        if read_len<BUFF_SIZE {
          break
        }

      }
      println!("");

    } else {
      let enstenned:Vec<char>=UTF8::enSten(args[1].as_bytes());
      println!("{}", enstenned.iter().collect::<String>());

    }
}
