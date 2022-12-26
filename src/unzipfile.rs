// 自定义一下文件格式
// | 压缩数据0填充个数 |  u8_path_map的大小x | u8_path_map  | 压缩数据 |
// |    3bit          |   13bit            |    x个bit    |      --  |

use std::{fs::{self, OpenOptions}, io::{Read, Write, Seek}, collections::HashMap, path::Path};

pub fn huffman_unzip(filename: &str) -> Result<String,String> {
    // 生成新文件名
    let pos = filename[..filename.len()-6].rfind(".").unwrap();
    let (name, suffix) = filename[..filename.len()-6].split_at(pos);
    let new_file_name = format!("{}-after{}", name,suffix);

    if Path::new(&new_file_name).exists() {
        let msg = format!("解压文件已存在：{}",new_file_name);
        return Err(msg);
    }
    let mut new_file = OpenOptions::new()
                            .create_new(true)
                            .write(true)
                            .open(&new_file_name)
                            .unwrap();

    const READ_BUFFER_SIZE: usize = 8 * 1024;
    const WRITE_BUFFER_SIZE: usize = 8 * 1024;

    let mut read_buf = [0u8; READ_BUFFER_SIZE];
    let mut write_buf = [0u8; WRITE_BUFFER_SIZE];

    let mut file = fs::File::open(filename).unwrap();
    file.read(&mut read_buf[..2]).unwrap();

    let data_zero_fill = (read_buf[0] >> 5) & 0b111;
    let map_len = (((read_buf[0] as u16) << 8) | (read_buf[1] as u16)) << 3 >> 3;


    // 将格式化后的map生成map
    // 形式：u8:len path,u8:len path,u8:len path, len占两个byte
    let mut map_string = vec![];
    let mut map_len = map_len as usize;
    let map_str_len = map_len;
    while map_len != 0 {
        if map_len >= READ_BUFFER_SIZE {
            file.read(&mut read_buf).unwrap();
            map_string.extend_from_slice(&read_buf);
            map_len -= READ_BUFFER_SIZE;
        } else {
            file.read(&mut read_buf[..map_len]).unwrap();
            map_string.extend_from_slice(&read_buf[..map_len]);
            map_len = 0;
        }
    }
    //println!("{}", map_string.len());

    let mut i = 0;
    let mut u8_path_map:HashMap<Vec<bool>, u8> = HashMap::new();
    while i < map_str_len {
        let u8_data = map_string[i];
        let len_part1 = map_string[i+2];
        let len_part2 = map_string[i+3];
        
        let mut path_len = (len_part1 as usize) << 8 | (len_part2 as usize);
        let source_path_len = path_len;
        if path_len % 8 != 0 {
            path_len += 8-path_len%8;
        } 
        let mut path_vec_rev = vec![];
        for j in 0..path_len/8 {
            let mut dd = map_string[i+4+j];
            for _ in 0..8 {
                if dd & 1 == 1 {
                    path_vec_rev.push(true);
                } else {
                    path_vec_rev.push(false);
                }
                dd >>= 1;
            }
        }

        // 此处bug好久才找出来
        for k in 0..(path_vec_rev.len()+7)/8 {
            path_vec_rev[k*8..k*8+8].reverse();
        }
        
        let key = &path_vec_rev[(path_len-source_path_len)..];
        u8_path_map.entry(key.to_vec()).or_insert(u8_data);

        i += 5 + path_len/8;
        // println!("{} {}",u8_data,path_len);
    }

    let mut write_byte_index = 0;

    let mut max_map_path_len = 0;
    for (k, _) in &u8_path_map {
        if k.len() > max_map_path_len {
            max_map_path_len = k.len();
        }
    }

    let mut data = vec![];
    loop {
        let readlen = file.read(&mut read_buf);
        match readlen {
            Ok(readlen) => {
                match readlen {
                    0 => {
                        // 肯定map都能匹配到，只是最后填充零会误写入一个bit
                        new_file.write_all(&write_buf[..write_byte_index]).unwrap();
                        break;
                    },
                    _ => {
                        for i_u8 in &read_buf[..readlen] {
                            for k in (0..8).rev() {
                                let mut cur_bit = false;
                                if *i_u8 & (1<<k) == 1<<k {
                                    cur_bit = true;
                                }                            
                                data.push(cur_bit);
                                if !cur_bit || data.len() == max_map_path_len {
                                    write_buf[write_byte_index] = u8_path_map[&data];
                                    write_byte_index += 1;
                                    if write_byte_index == WRITE_BUFFER_SIZE {
                                        new_file.write_all(&write_buf).unwrap();
                                        write_byte_index = 0;
                                    }
                                    data.clear();
                                }
                            }
                        }
                    }
                }
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }
    // 零的占位符是肯定能匹配上字符的，所以最后只要根据零占位符个数进行截断即可。

    let sz = new_file.stream_position().unwrap()-data_zero_fill as u64;

    new_file.set_len(sz).unwrap();

    Ok(new_file_name)
}

