use std::collections::HashMap;

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