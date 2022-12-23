use std::{collections::HashMap, rc::Rc, cell::RefCell};
use std::fmt::Debug;

pub struct  HuffmanTree {
    pub root: Rc<RefCell<HuffmanNode>>,
    pub u8_path_map: HashMap<u8,Vec<bool>>,
}
pub struct HuffmanNode {
    is_leaf: bool,
    u8_data: u8,       // u8字符
    path: Vec<bool>,   // 字符对应的路径
    left_child: Option<Rc<RefCell<HuffmanNode>>>,
    right_child: Option<Rc<RefCell<HuffmanNode>>>,
}


impl HuffmanNode {
    fn default(u8_data: u8) -> HuffmanNode {
        HuffmanNode {is_leaf: false, u8_data, path: vec![],  left_child: None, right_child: None }
    }
    pub fn build(u8_count_map: &HashMap<u8, u64>) -> HuffmanTree {
        // 对u8 count map进行排序
        let u8_count_map = u8_count_map.clone();
        let mut u8_count_vec: Vec<_> = u8_count_map.iter().collect();
        u8_count_vec.sort_by(|a,b| a.1.cmp(b.1));
        // println!("{:?}", u8_count_vec);

        // 在哈夫曼树中，只有一个节点是不允许的，因为没有路径
        // 这里只是简单过滤下，应有更高级的处理方法
        assert!(u8_count_vec.len()>1);
        

        // 创建第一个节点，如果还有其他节点，则开始合并
        let mut ret = Rc::new(RefCell::new(Self::default(u8_count_vec[0].0.clone())));
        ret.borrow_mut().is_leaf = true;

        u8_count_vec.remove(0);

        while u8_count_vec.len()>0 {
            // 新加入的节点
            let node = Rc::new(RefCell::new(Self::default(u8_count_vec[0].0.clone())));
            node.borrow_mut().is_leaf = true;

            let father = Rc::new(RefCell::new(Self::default(0)));
            father.borrow_mut().is_leaf = false;
            father.borrow_mut().left_child = Some(Rc::clone(&node));
            father.borrow_mut().right_child = Some(Rc::clone(&ret));

            ret = Rc::clone(&father);
            u8_count_vec.remove(0);
        }
        
        // 到此，树已经成型了，但是path还没分配，通过深度遍历进行路径确定。
        let mut u8_path_map = HashMap::new();
        dfs(Rc::clone(&ret),vec![], &mut u8_path_map);
        // println!("{:?}",u8_path_map);
        HuffmanTree { root: ret, u8_path_map }
    }
}

impl Debug for HuffmanNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Huffman Node - u8_data:{}, path: {:?}, is_leaf: {}, left: {:?}, right: {:?}", self.u8_data, self.path,self.is_leaf, self.left_child,self.right_child)
    }
}

fn dfs(root: Rc<RefCell<HuffmanNode>>, path: Vec<bool>, u8_path_map: &mut HashMap<u8, Vec<bool>>) {
    if root.borrow().is_leaf {
        root.borrow_mut().path = path.clone();
        u8_path_map.entry(root.borrow().u8_data).or_insert(path.clone());
    }
    match &root.borrow().left_child {
        None => (),
        Some(child) => {
            let mut p = path.clone();
            p.push(false);
            dfs(Rc::clone(child), p, u8_path_map);
        }
    }
    match &root.borrow().right_child {
        None => (),
        Some(child) => {
            let mut p = path.clone();
            p.push(true);
            dfs(Rc::clone(child), p, u8_path_map);
        }
    }
}