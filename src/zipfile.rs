use std::{fs::{self, OpenOptions}, io::{Read, Write, Seek, SeekFrom}, path::Path};

use crate::{huffman_tree::HuffmanTree, stringify};

pub fn huffman_zip(huffman_tree: HuffmanTree, filename: &str) -> Result<String, String> {
    // println!("{:?}", huffman_tree.u8_path_map);
    // {104: [true, true], 10: [false], 105: [true, false]}

    const READ_BUFFER_SIZE: usize = 8 * 1024;
    const WRITE_BUFFER_SIZE: usize = 8 * 1024;

    assert!(WRITE_BUFFER_SIZE>=2); // 写缓冲不能太小，后面有些细节直接填充区缓冲区前几个u8

    let mut read_buffer = [0u8; READ_BUFFER_SIZE];
    let mut write_buffer = [0u8; WRITE_BUFFER_SIZE];
    let mut file = fs::File::open(filename).unwrap();
    
    // 自定义一下文件格式
    // | 压缩数据0填充个数 |  u8_path_map的大小x | u8_path_map  | 压缩数据 |
    // |    3bit          |   13bit            |    x个bit    |      --  |

    let new_file_name = filename.to_string() + ".huuuf";
    if Path::new(&new_file_name).exists() {
        let msg = format!("压缩文件已存在：{}",new_file_name);
        return Err(msg);
    }
    let mut new_file = OpenOptions::new()
                            .create_new(true)
                            .write(true)
                            .open(&new_file_name)
                            .unwrap();

    let mut cur_byte = 0u8;  // 当前写byte
    let mut cur_count = 0; // 每8位写一下缓冲
    let mut cur_buf_u8_count = 0;

    let mut after_zip_data_u8_count = 0;

    // 数据零填充个数
    let mut data_zero_fill = 0;

    
    // let u8_path_map_string = format!("{:?}", huffman_tree.u8_path_map);

    let u8_path_map_string = stringify::stringify(&huffman_tree.u8_path_map);
    let u8_path_map_size = u8_path_map_string.len() as u16;
    // println!("{}", u8_path_map_size);
    assert!(u8_path_map_size < 1<<13);

    // 先零填充前两个byte
    {
        cur_buf_u8_count += 2;
    }

    // 填充 u8_path_map
     for d in &u8_path_map_string {
        if cur_buf_u8_count == WRITE_BUFFER_SIZE {
            new_file.write_all(&write_buffer).unwrap();
            cur_buf_u8_count = 0;
        }
        write_buffer[cur_buf_u8_count] = *d;
        cur_buf_u8_count += 1;
    }

    // 填充压缩数据
    loop {
        let sz = file.read(&mut read_buffer);
        match sz {
            Ok(sz) => {
                match sz {
                    0 => {
                        println!("文件压缩结束 ...");
                        // 也需考虑未完成的字符和写缓冲区
                        if cur_count !=0 {
                            data_zero_fill = 8-cur_count;
                            cur_byte <<= data_zero_fill;
                            write_buffer[cur_buf_u8_count] = cur_byte;
                            cur_buf_u8_count += 1;
                        }

                        if cur_buf_u8_count != 0 {
                            new_file.write_all(&write_buffer[..cur_buf_u8_count]).unwrap();
                            after_zip_data_u8_count += cur_buf_u8_count;
                        }

                        break;
                    }
                    _ => {
                        for i in 0..sz {
                            let chr = read_buffer[i];
                            for v in &huffman_tree.u8_path_map[&chr] {
                                // 判断边界
                                if cur_count == 8 {
                                    write_buffer[cur_buf_u8_count] = cur_byte;
                                    cur_byte = 0;
                                    cur_buf_u8_count += 1;
                                    cur_count = 0;
                                }

                                if cur_buf_u8_count == WRITE_BUFFER_SIZE {
                                    new_file.write_all(&write_buffer).unwrap();
                                    cur_buf_u8_count = 0;
                                    after_zip_data_u8_count += WRITE_BUFFER_SIZE;
                                }

                                // 编码字符
                                cur_byte = cur_byte<<1;
                                if *v {
                                    cur_byte |= 1;
                                }
                                cur_count += 1;
                            }
                        }
                    }
                }
            },
            Err(e) => {
                println!("文件压缩解析失败：{}", e.to_string());
                return Err(e.to_string());
            }
        }
    }

    println!("压缩后文件的大小：{}, map的大小： {}", after_zip_data_u8_count, u8_path_map_size);

    // 修正前两个byte
    {
        // 占位压缩数据0填充个数3个bit
        let a1 = (data_zero_fill<<5) | (u8_path_map_size>>8 & 0b11111) as u8;
        let a2 = (u8_path_map_size & 0b1111_1111) as u8;
        write_buffer[0] = a1;
        write_buffer[1] = a2;


        new_file.seek(SeekFrom::Start(0)).unwrap();
        new_file.write_all(&write_buffer[..2]).unwrap();
    }

    Ok(new_file_name)
}