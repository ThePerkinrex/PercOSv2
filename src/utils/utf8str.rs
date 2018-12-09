//TODO IMPLEMENT THIS TRAITS
use core::cmp::Eq;
use core::convert::From;
use core::convert::Into;
use core::fmt::Display;
use core::fmt::Debug;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Add;
use core::ops::AddAssign;


const MAX_LEN: usize = 256;
const EMPTY_STR_ARRAY: [u8; MAX_LEN] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
pub struct UTF8StrErrors {}
impl UTF8StrErrors {
    const STRING_FULL_ERROR: u8 = 0;
    const STRING_EMPTY_ERROR: u8 = 1;
    const INDEX_TOO_BIG_ERROR: u8 = 2;
    const INDEX_TOO_SMALL_ERROR: u8 = 3;
    const INDEX_OUT_OF_BOUNDS_ERROR: u8 = 4;
}

pub struct UTF8Str {
    array: [u8; MAX_LEN],
    str_len: usize
}

impl UTF8Str {
    pub fn new() -> UTF8Str {
        UTF8Str {
            array: EMPTY_STR_ARRAY,
            str_len: 0
        }
    }

    pub fn push(&mut self, chr: u8) -> Result<(),u8> {
        if self.str_len + 1 < MAX_LEN {
            self.array[self.str_len] = chr;
            self.str_len += 1;
            Ok(())
        } else {
            
            Err(UTF8StrErrors::STRING_FULL_ERROR)
        }
    }

    pub fn pop(&mut self) -> Result<u8,u8> {
        if self.str_len != 0 {
            self.str_len -= 1;
            Ok(self.array[self.str_len])
        } else {
            Err(UTF8StrErrors::STRING_EMPTY_ERROR)
        }
    }

    pub fn remove(&mut self, index: usize) -> Result<u8,u8>{
        if self.str_len == 0 {
            Err(UTF8StrErrors::STRING_EMPTY_ERROR)
        } else if index < 0 {
            Err(UTF8StrErrors::INDEX_TOO_SMALL_ERROR)
        } else if index >= MAX_LEN {
            Err(UTF8StrErrors::INDEX_TOO_BIG_ERROR)
        } else if index >= self.str_len {
            Err(UTF8StrErrors::INDEX_OUT_OF_BOUNDS_ERROR)
        } else if self.str_len-1 == index {
            self.str_len -= 1;
            Ok(self.array[index])
        } else {
            let res = self.array[index];
            self.str_len -= 1;
            for i in (index+1)..(self.str_len) {
                self.array[i-1] = self.array[i];
            }
            Ok(res)
        }
    }
}