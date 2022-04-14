use rand::Rng;

const ARRAY_SIZE: usize = 10;
fn main() {
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let mut count = 0;
    while count < ARRAY_SIZE {
        array[count] = rand::thread_rng().gen_range(0..100);
        count+=1;
    }

    println!("Generate rand array: {:?}", array);
    println!("Swaped array       : {:?}", heap_sort(array.to_vec()));
}

struct NodesIdx {
    parent: usize,
    child: usize
}
impl NodesIdx {
    fn set_parent(&mut self, idx: usize) {
        self.parent = idx; 
        self.calc_child_idx();
    }
    fn set_child(&mut self, idx: usize) {
        self.child = idx; 
    }
    fn calc_last_parent_idx(&mut self, len:usize) {
        self.set_parent((len / 2) -1);
    }
    fn calc_child_idx(&mut self) {
        self.set_child(self.parent * 2 + 1);
    }
}

fn heap_sort(v:Vec<i32>) -> Vec<i32> {
    let mut nodes_idx = NodesIdx {parent: 0, child: 0};
    nodes_idx.calc_last_parent_idx(v.len());
    println!("parent_idx: {}, child_idx: {}", nodes_idx.parent, nodes_idx.child);
    nodes_swap(v, nodes_idx)
}

fn nodes_swap(mut v:Vec<i32>, mut nodes_idx:NodesIdx) -> Vec<i32> {
    for _i in 0..1 {
        if v[nodes_idx.parent] < v[nodes_idx.child] {
            v.swap(nodes_idx.parent, nodes_idx.child);
        }
        nodes_idx.child = nodes_idx.child + 1;
    }
    v
}

#[allow(dead_code)]
fn ind_max(v:Vec<i32>) -> (usize, i32) {
    v.iter()
        .enumerate()
        .fold((usize::MIN, i32::MIN), |(i_a, a), (i_b, &b)| {
            if b < a {
                (i_a, a)
            } else {
                (i_b, b)
            }
        })
}
