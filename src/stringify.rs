use std::{collections::HashMap, ops::{Shl, Shr}};

#[derive(Debug)]
pub struct u512([u64; 8]);

impl Shl<usize> for u512 {
    type Output = Self;

    fn shl(self, shift: usize) -> Self::Output {
        let u512(ref original) = self;
        let mut ret = [0u64; 8];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;

        // shift
        for i in word_shift..8 {
            ret[i  - word_shift] = original[i] << bit_shift;
        }
        // carry
        if bit_shift > 0 {
            for i in word_shift+1..8 {
                ret[i - 1 - word_shift] += original[i] >> (64 - bit_shift);
            }
        }
        u512(ret)
    }
}

impl Shr<usize> for u512 {
    type Output = u512;

    fn shr(self, shift: usize) -> u512 {
        let u512(ref original) = self;
        let mut ret = [0u64; 8];
        let word_shift = shift / 64;
        let bit_shift = shift % 64;

        // shift
        for i in word_shift..8 {
            ret[i] = original[i - word_shift] >> bit_shift;
        }

        // Carry
        if bit_shift > 0 {
            for i in word_shift+1..8 {
                ret[i] += original[i - word_shift - 1] << (64 - bit_shift);
            }
        }

        u512(ret)
    }
}

pub fn stringify(map: &HashMap<u8,Vec<bool>>) -> Vec<u8> {
    let mut ret = vec![];
    //println!("{:?}", map);
    for (k,v) in map {
        // println!("{:?}",v);
        ret.push(*k);
        ret.push(':' as u8);

        // u8:len path, len占两个byte
        let vlen = v.len() as u16;
        ret.push((vlen >> 8 & 0b1111_1111) as u8);
        ret.push((vlen & 0b1111_1111) as u8);

        let mut len_size = (vlen+7)/8; // vlen应该占用几个字节
        
        let mut vb = u512([0u64; 8]);
        // let mut vb = 0u128;
        for b in v {
            vb = vb << 1;
            if *b {
                vb.0[7] |= 1;
            }
        }

        let mut tmp_vec = vec![];
        while len_size!=0 {
            tmp_vec.push((vb.0[7] & 0b1111_1111) as u8);
            vb = vb >> 8;
            len_size -= 1;
        }
        while tmp_vec.len() != 0 {
            ret.push(tmp_vec.pop().unwrap());
        }
        ret.push(',' as u8);
    }
    ret
}

#[test]
fn test_stringify() {
    let mut map:HashMap<u8, Vec<bool>> = HashMap::new();
    map.entry(32u8).or_insert(vec![true,false,false,true,true,false,false,true,false,false,true,true,false,false,true,false,false,true,true,false,false,true,false,false,true,true,false,false,]);
    map.entry(1u8).or_insert(vec![false,true,false,false,true,true,false,false,true,false,false,true,true,false,false,]);
 
    unsafe {

        println!("{}",String::from_utf8_unchecked(stringify(&map)));
    }
    
}