mod u8_count;
mod huffman_tree;
mod zipfile;
// mod test;
mod stringify;
mod unzipfile;

fn main() {
    // 要压缩的文件名
    let filename = "./bbe.txt";

    // 统计u8频率
    let u8_count_map = u8_count::get_u8_count_map(filename);
    if let Err(e) =  u8_count_map {
        println!("统计u8失败：{:?}", e);
        return;
    }

    let u8_count_map = u8_count_map.unwrap();

    // 构建Huffman Tree
    let hf_tree = huffman_tree::HuffmanNode::build(&u8_count_map);

    // 根据Huffman Tree和源文件生成压缩文件
    let newfilename = zipfile::huffman_zip(hf_tree, filename);
    match newfilename {
        Ok(newfilename) => {
            println!("压缩成功，压缩文件：{}", newfilename);
        },
        Err(e) => {
            println!("压缩失败：{}", e.to_owned())
        }
    }

    // 要解压的文件名

    let unzipfilename = filename.to_string() + ".huuuf";
    let afterzip_filename = unzipfile::huffman_unzip(&unzipfilename);
    match afterzip_filename {
        Ok(afterzip_filename) => {
            println!("解压成功，生成文件：{}", afterzip_filename);
        },
        Err(e) => {
            println!("解压失败：{}", e.to_owned())
        }
    }
}

#[test]
fn testzip() {
    // 要压缩的文件名
    let filename = "./a.txt";

    // 统计u8频率
    let u8_count_map = u8_count::get_u8_count_map(filename);
    if let Err(e) =  u8_count_map {
        println!("统计u8失败：{:?}", e);
        return;
    }

    let u8_count_map = u8_count_map.unwrap();

    // 构建Huffman Tree
    let hf_tree = huffman_tree::HuffmanNode::build(&u8_count_map);

    // 根据Huffman Tree和源文件生成压缩文件
    let filename = zipfile::huffman_zip(hf_tree, filename);
    match filename {
        Ok(filename) => {
            println!("压缩成功，压缩文件：{}", filename);
        },
        Err(e) => {
            println!("压缩失败：{}", e.to_owned())
        }
    }
}

#[test]
fn testunzip() {
    // 要解压的文件名
    let filename = "./a.txt";
    let unzipfilename = filename.to_string() + ".huuuf";
    let afterzip_filename = unzipfile::huffman_unzip(&unzipfilename);
    match afterzip_filename {
        Ok(afterzip_filename) => {
            println!("解压成功，生成文件：{}", afterzip_filename);
        },
        Err(e) => {
            println!("解压失败：{}", e.to_owned())
        }
    }
}
