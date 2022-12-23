use std::{fs, io::Read, collections::HashMap, ops::{AddAssign}};

pub fn get_u8_count_map(filename: &str) -> Result<HashMap<u8, u64>,String> {
    let mut buffer = [0u8; 8];
    let mut u8_count_map:HashMap<u8, u64> = HashMap::new();
    let mut file = fs::File::open(filename).unwrap();
    
    loop {
        let sz = file.read(&mut buffer);
        match sz {
            Ok(sz) => {
                match sz {
                    0 => {
                        println!("文件统计结束 ...");
                        break;
                    }
                    _ => {
                        // println!("{:?}", &buffer[..sz]);
                        for i in 0..sz {
                            u8_count_map.entry(buffer[i])
                                        .or_insert(0)
                                        .add_assign(1);
                                        
                        }
                    }
                }
            },
            Err(e) => {
                println!("文件解析失败：{}", e.to_string());
                return Err(e.to_string());
            }
        }
    }
    Ok(u8_count_map)
}