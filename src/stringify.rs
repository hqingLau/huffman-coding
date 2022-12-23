use std::{collections::HashMap};

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
        
        let mut vb = 0u128;
        assert!(map.len()<127); // 太长不搞
        for b in v {
            vb <<= 1;
            if *b {
                vb |= 1;
            }
        }
        let mut tmp_vec = vec![];
        while len_size!=0 {
            tmp_vec.push((vb & 0b1111_1111) as u8);
            vb >>= 8;
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