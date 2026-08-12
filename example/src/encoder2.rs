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
use core::slice;
use std::io::{Write, Read, IsTerminal, BufWriter, BufReader};
use UTF8::{char_slice_to_utf8_unchecked};
use UTF8::Block;

// buffer size must be 2*x for best efficiency with v2
const BUFF_SIZE:usize=512;

fn main() {
    //let start=Instant::now();
    let args:Vec<String>=std::env::args().collect();

    if args.len()<2 {
      let mut stdin=BufReader::new(std::io::stdin().lock());
      let mut stdout=BufWriter::new(std::io::stdout().lock());

      let mut buff:[u8;BUFF_SIZE]=[0;BUFF_SIZE];
      let mut out_buff:Vec<char> = vec![0 as char; BUFF_SIZE*2];
      let mut read_len:usize;
      let mut write_len:usize;

      'enc_l:loop{
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

        //checks if data can be encoded in a valid output
        if !Block::v2_encode_valid(&buff[..read_len]) {
          eprintln!("cannot be encoded with second version");
          eprintln!("you should probably remove non ascii characters and unicode or UTF8 encoded characters for v2 to work correctly");
          eprintln!("or use v1 encoder");
          return;
        }

        write_len = unsafe { UTF8::enSten2_to(&buff[..read_len], &mut out_buff) };
        let _ = stdout.write_all(unsafe {
                slice::from_raw_parts(
                    out_buff.as_ptr() as *const u8,
                    //returns length of written utf8 bytes
                    char_slice_to_utf8_unchecked(
                        out_buff.as_ptr(), write_len,
                        out_buff.as_mut_ptr() as *mut u8
                    )
                )
            });

        //quits loop after reaching last chunk of data
        if read_len<BUFF_SIZE {
          break
        }

      }
      let _ = stdout.flush();

    } else {
      //checks if data can be encoded in a valid output
      if !Block::v2_encode_valid(args[1].as_bytes()) {
        eprintln!("cannot be encoded with second version");
        eprintln!("you should probably remove non ascii characters and unicode or UTF8 encoded characters for v2");
        eprintln!("or use v1 encoder");
        return;
      }

      let enstenned:Vec<char>=UTF8::enSten2(args[1].as_bytes());
      print!("{}", enstenned.iter().collect::<String>());

    }

    //don't pipe \n when output piped/redirected
    if std::io::stdout().is_terminal() {
      println!("");
    }
    //eprintln!("main function were running {:?}", start.elapsed());
}
