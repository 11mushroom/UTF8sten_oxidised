/*
*   UTF8sten_oxidised gives very basic CLI tools to work with u8s(UTF8sten) encoding
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

use core::{slice};
use std::io::{Write, Read, IsTerminal, BufWriter, BufReader};

// buffer size must be dividible by 4 to support v2
const BUFF_SIZE:usize=512;

fn main() {
    let args:Vec<String>=std::env::args().collect();
    let mut stdout=BufWriter::new(std::io::stdout().lock());


    if args.len()<2 {
      let mut stdin=BufReader::new(std::io::stdin().lock());

      let mut buff:[u8;BUFF_SIZE]=[0;BUFF_SIZE];
      let mut codepoints:Vec<u32> = vec![0;BUFF_SIZE];
      'denc_l:loop{
        let mut read_len:usize = 0;
        while read_len<BUFF_SIZE {
          match stdin.read(&mut buff[read_len..]){
            Ok(0) if read_len==0 => break 'denc_l,
            Ok(0) => break,
            Ok(n) => read_len+=n,
            Err(e) => panic!("what happened? {}", e)
          };
        }
        //eprintln!("read {} bytes", read_len);


        let cp_len:usize=cfg_select! {
          feature = "skip-checks" => unsafe {
              str::from_utf8_unchecked(&buff[..read_len]).chars()
                .fold(0, |l, c| {
                  codepoints[l]=c as u32;
                  l+1
                })
          },
          _ => {{
            let ret=match str::from_utf8(&buff[..read_len]) {
              Ok(s) => s.chars().fold(0, |l, c| {
                    codepoints[l]=c as u32;
                    l+1
                }),
              Err(e) => { 
                eprintln!("failed to convert decode utf8 bytes");
                eprintln!("you can try to bypass validation checks by compiling it with \"skip-validations\" feature");
                panic!("{}", e);
              }
            };
            if utf8sten::valid_en_v2(&codepoints[..ret]){
              ret
            } else {
              panic!("invalid u8s v2 input data");
            }
          }}
          };

        // reuse buffer
        // safe to reuse buff because
        // decoded data is always smaller than encoded
        let result_len = unsafe { utf8sten::deSten2_to_raw_unchecked(codepoints.as_ptr(), cp_len, codepoints.as_mut_ptr() as *mut u8)};
        let _ = stdout.write_all(unsafe { slice::from_raw_parts(codepoints.as_ptr() as *const u8, result_len)} );

        if read_len<BUFF_SIZE {
          break
        }

      }
      let _ = stdout.flush();

    } else {
      let destenned:Vec<u8>=utf8sten::deSten2(&args[1].chars().map(|c| c as u32).collect::<Vec<u32>>()).unwrap();
      let _ = stdout.write_all(&destenned);
      let _ = stdout.flush();
    }

    //don't pipe \n when output piped/redirected
    if std::io::stdout().is_terminal() {
      println!("");
    }
}
