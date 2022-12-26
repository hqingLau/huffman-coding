use std::{collections::HashMap, ops::{Shr, Shl}};

#[derive(Debug)]
struct List {
    head: Option<Box<List>>,
    num: i32,
}

impl List {
    fn new() -> List {
        List { head: None, num: 0 }
    }

    fn prepend(self, elem: i32) -> List {
        List {
            head: Some(Box::new(self)),
            num: elem,
        }
    }

    fn len(&self) -> u32 {
        match &self.head {
            Some(list) => list.len() + 1,
            None => 0,
        }
    }

    fn stringify(&self) -> String {
        match &self.head {
            Some(list) => format!("{}, {}", self.num, list.stringify()),
            None => format!("Nil"),
        }
    }
}

#[test]
fn test_owner() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

#[test]
fn test_extend_vec() {
    let mut a = Vec::new();
    let b = vec![1,2,3];
    let c = vec![4,5];
    a.extend_from_slice(&b);
    a.extend_from_slice(&c);
    a[4] = 999;
    println!("{:?}", a);
    println!("{:?}",b);
    println!("{:?}",c);
}

#[test]
fn test_str_find() {
    let filename = String::from("./a.txt.huuuf");
    
    let pos = filename[..filename.len()-6].rfind(".").unwrap();
    let (name, suffix) = filename[..filename.len()-6].split_at(pos);
    let new_file_name = format!("{}-after{}", name,suffix);
    println!("{}", new_file_name);
}

#[test]
fn bit_and() {
    let i = 32u8;
    let b = 1<<5;
    println!("{}, {}",i,b);
    let c = i & b;
    println!("{}",c);
}

#[test]
fn test_map_iter() {
    let mut scores = HashMap::new();
    scores.entry(8u8).or_insert(vec![true,false,false,false]);
    scores.entry(18u8).or_insert(vec![false,true,false]);
    for (k, v) in &scores{
        println!("{}:{:?}", k, v);
    }
}



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
        // AB,CD,EF,GH
        // D0,F0,H0,00
        // DE,FG,H0,00
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

        // AB,CD,EF,GH
        // 00,0A,0C,0E
        // DE,FG,H0,00
        u512(ret)
    }
}

#[test]
fn test_u512() {
    let mut a = u512([0u64;8]);
    
    a = a<<1;
    a.0[7] |= 1;
    println!("{:?}",a);
    a = a<<1;
    a.0[7] |= 1;
    println!("{:?}",a);
    a = a<<1;
    a.0[7] |= 1;
    println!("{:?}",a);
    a = a<<1;
    a.0[7] |= 1;
    println!("{:?}",a);
    a = a>>62;
    println!("{:?}",a);
}