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

const FFU32 :u32=0xffffffff;
const OCTPR :u32=0b10000000;
const PB2   :u32=0b11000000;
const PB3   :u32=0b11100000;
const PB4   :u32=0b11110000;
const MASK3 :u32=0b00000111;
const MASK4 :u32=0b00001111;
const MASK5 :u32=0b00011111;
const MASK6 :u32=0b00111111;
const MASK8 :u32=0b11111111;
const RMASK2:u32=0b11000000;
const RMASK3:u32=0b11100000;
const RMASK4:u32=0b11110000;
const RMASK5:u32=0b11111000;

//function to calculate amount of encoded data will take in bytes

pub fn getEnLen(len: u32) -> usize {
  let mut res:usize;
  res=((len/3)*6) as usize;
  res+=((len%3)*3) as usize;
  return res;
}

//function to calculate amount of decoded data will take in bytes

pub fn getStenLen(arr: &[u32]) -> usize {
  let mut res:usize=0;
  let mut bits:u32=0;

  for i in 0..arr.len() {
    if arr[i]<=0x8fff && arr[i]>=0x8000 {
      bits+=12;
    } else if arr[i]<=0xff {
      bits+=8;
    }

    //res+=bits/8;
    //bits=bits%8;
    res+=(bits>>3) as usize;
    bits&=7;
  }

  if bits>0 { 
    res+=1;
  }

  return res;
}

//functions to get value of specific bit in number

/*fn gBit(num:u8, ind:u32) -> u32{
  (num>>ind)&1
}*/

fn gBit(num:u32, ind:u32) -> u32{
  (num>>ind)&1
}

//function to encode single code point into UTF-8
//it recives unsigned code and returns structure of array of bytes

/*fn UTF8_enc(code:u32) -> Vec<u8> {
  let len:u8 = 
    if code<0x10000 {
        if code<0x0800 {
            if code<0x0080 {1} else {2}
        } else {3}
    } else {4};

  let mut bytes:Vec<u8>=vec![0;len as usize];
 
  match len {
    1_u8 => {
      bytes[0]=code as u8;
    },

    2_u8 => {
      bytes[0]=(PB2|((code>>6)&MASK5)) as u8;
      bytes[1]=(OCTPR|(code&MASK6)) as u8;
    },

    3_u8 => {
      bytes[0]=(PB3|((code>>12)&MASK4)) as u8;
      bytes[1]=(OCTPR|((code>>6)&MASK6)) as u8;
      bytes[2]=(OCTPR|(code&MASK6)) as u8;
    },

    4_u8 => {
      bytes[0]=(PB4|((code>>18)&MASK3)) as u8;
      bytes[1]=(OCTPR|((code>>12)&MASK6)) as u8;
      bytes[2]=(OCTPR|((code>>6)&MASK6)) as u8;
      bytes[3]=(OCTPR|(code&MASK6)) as u8;
    },
    _ => {
    }
  }
  
  return bytes;
}*/

//function to calculate length of string not by bytes but by characters, including UTF-8 characters

fn calcLen(str:&String) -> usize {
  str.chars().count()
}

//function to deencode string that contains UTF-8 characters and returns Vector with codepoints of characters

pub fn UTF8_den( string:&String ) -> Vec<u32> {
  string.chars().map(|c| c as u32).collect::<Vec<u32>>()
}

//function to encode bytes in UTF-8 characters, recives array of bytes and length of that array, and returns vector with codepoints with data stored in it

pub fn enSten(arr: &[u8]) -> Vec<char> {
  let len:usize=arr.len();
  let enLen:usize=(len*2)/3+(if(len*2)%3>0 {1} else {0});//getEnLen(len);

  let mut res:Vec<char>=vec![0 as char;enLen];

  let mut codePoint:u32=0x8000;
  let mut subB:u8=0;
  let mut bits:u8=0;
  let mut cary:u8;
  let mut shift:u8;
  let mut bitsPass:u8=0;
  let mut dataI:usize=0;
  let mut i:usize=0;

  while i < len {
    if bits<=0 {
      bits=8;
    }
    cary=12-subB;
    shift=subB;


    if bits<=cary {
      subB+=bits;
      //codePoint|=((arr[i]>>bitsPass)&((1<<bits)-1))<<shift;
      codePoint|=(((arr[i] as u32)>>bitsPass)&(!(FFU32<<bits)))<<shift;
      bits=0;
      bitsPass=0;
      i += 1;

    } else if bits>cary {
      subB=12;
      //codePoint|=((arr[i]>>bitsPass)&((1<<cary)-1))<<shift;
      codePoint|=(((arr[i] as u32)>>bitsPass)&(!(FFU32<<cary)))<<shift;
      bits-=cary;
      bitsPass=cary;

    }


    if subB>=12||(i>=len) {
      res[dataI]=char::from_u32(codePoint).expect("[enSten()]: failed to convert codepoint into char");
      dataI += 1;
      codePoint=0x8000;
    }

    subB%=12;
  }

  return res;
}

//function to decode data from codepoints
//decodes result of enSten function

pub fn deSten(arr:&[u32]) -> Vec<u8> {
  let len:usize=arr.len();
  

  let mut res:Vec<u8>=vec![0;(len*3)/2 + if (len*3)%2>0 {1} else {0}];

  let mut dataI:usize=0;
  let mut bits:u8;
  let mut bitsPass:u8;
  let mut subB:u8=0;
  let mut cary:u8;
  let mut shift:u8;
  let mut proc:bool;


  for i in 0..len {
    proc=false;
    bitsPass=0;
    bits=0;

    if arr[i]<=0x8fff && arr[i]>=0x8000 {
      bits=12;
      proc=true;

    } else if arr[i]<=0x8ff && arr[i]>=0x800 {
      bits=8;
      proc=true;
      
    } else if arr[i]<=0xff {
      bits=8;
      proc=true;
    }
    
    //proccess data
    while bits>0 && proc {
      cary=8-subB;
      shift=subB;

      if bits<=cary {
        subB+=bits;
        //res.bytes[dataI] |= ((arr[i]>>bitsPass)&((1<<bits)-1))<<shift;
        res[dataI] |= (((arr[i]>>bitsPass)&(!(FFU32<<bits)))<<shift) as u8;
        bits=0;

      } else if bits>cary {
        subB=8;
        //res.bytes[dataI] |= ((arr[i]>>bitsPass)&((1<<cary)-1))<<shift;
        res[dataI] |= (((arr[i]>>bitsPass)&(!(FFU32<<cary)))<<shift) as u8;
        bits-=cary;
        bitsPass+=cary;
        
      }
      //dataI+=subB/8;
      //subB%=8;
      dataI+=(subB>>3) as usize;
      subB&=7;
    }

  }

  return res;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enstenning_works() {
        eprintln!("\nSTART enSten");

        let result = enSten("hello!".as_bytes());
        eprintln!("result: {}", result.iter().collect::<String>());

        let buff=[34152_u32, 34502_u32, 36716_u32, 33302_u32];
        let need:Vec<char>=buff.iter()
            .map(|code| char::from_u32(*code).expect("err"))
            .collect();

        eprintln!("need:   {}", need.iter().collect::<String>());

        assert_eq!(result, need);
    }

    #[test]
    fn destenning_works() {
        eprintln!("\nSTART deSten");

        let buff=[34152_u32, 34502_u32, 36716_u32, 33302_u32];
        let need="hello!".as_bytes();

        let result = deSten(&buff);

        eprintln!("result: {}", result.iter().map(|c| *c as char).collect::<String>());
        eprintln!("need:   {}", need.iter().map(|c| *c as char).collect::<String>());

        assert_eq!(result, need);
    }
}
