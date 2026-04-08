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

//use std::time::Instant;
use std::io::{Read, Write};

// buffer size must be 3*x for best efficiency with v1
const BUFF_SIZE:usize=510;

fn main() {
    //let start=Instant::now();
    let args:Vec<String>=std::env::args().collect();

    if args.len()<2 {
      let mut stdin=std::io::stdin().lock();
      let mut stdout=std::io::stdout().lock();
      
      let mut buff:[u8;BUFF_SIZE]=[0;BUFF_SIZE];
      let mut read_len:usize;

      'enc_l:loop{
        //let mut start=Instant::now();
        read_len=0;

        //this loop ensures that buffer is full except last chunk of data
        while read_len<BUFF_SIZE {
          match stdin.read(&mut buff[read_len..]){
            Ok(0) if read_len==0 => break 'enc_l,
            Ok(0) => break,
            Ok(n) => read_len+=n,
            Err(e) => panic!("what happened? {}", e)
          };
        }
        /*eprintln!("read {} bytes", read_len);*/

        //eprintln!("reading: {:?}", start.elapsed());

        //start=Instant::now();
        let result:String = UTF8::enSten(&buff[..read_len]).iter().collect();
        let _ = stdout.write_all(result.as_bytes());
        //eprintln!("encoding: {:?}", start.elapsed());

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
    //eprintln!("main function were running {:?}", start.elapsed());
}
