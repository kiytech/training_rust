use rand::Rng;

struct NodesIdx {
    parent: usize,
    child: usize,
    node_count: usize
}
impl NodesIdx {
    fn set_parent(&mut self, idx: usize) {
        self.parent = idx; 
        self.calc_child_idx();
    }
    fn set_child(&mut self, idx: usize) {
        self.child = idx; 
    }
    fn calc_last_parent_idx(&mut self) {
        self.set_parent((self.node_count / 2) -1);
    }
    fn calc_child_idx(&mut self) {
        self.set_child(self.parent * 2 + 1);
    }
    fn has_child(&self)  -> bool {
        (self.child * 2 + 1) < self.node_count - 1
    }
}

fn main() {
    const ARRAY_SIZE: usize = 20;
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let mut count = 0;
    while count < ARRAY_SIZE {
        array[count] = rand::thread_rng().gen_range(0..100);
        count+=1;
    }

    println!("Generate rand array: {:?}", array);
    println!("Swaped array       : {:?}", heap_sort(array.to_vec()));
}

fn heap_sort(mut v: Vec<i32>) -> Vec<i32> {
    for i in 0..v.len()-1 {
        let mut nodes_idx = NodesIdx {parent: 0, child: 0, node_count: v.len()-i};
        nodes_idx.calc_last_parent_idx();
        nodes_swap(&mut v, &mut nodes_idx);
        v.swap(0, nodes_idx.node_count-1);
    };
    v
}

fn nodes_swap(v: &mut Vec<i32>, nodes_idx: &mut NodesIdx) {
    for _j in 0..nodes_idx.parent+1 {
        comp_and_swap(v, nodes_idx);
        if nodes_idx.parent != 0 {
            nodes_idx.set_parent(nodes_idx.parent - 1);
        }
    }
}

fn comp_and_swap(v: &mut Vec<i32>, nodes_idx: &mut NodesIdx) {
    for _i in 0..2 {
        if nodes_idx.child > nodes_idx.node_count-1 {
            continue;
        }
        if v[nodes_idx.parent] < v[nodes_idx.child] {
            v.swap(nodes_idx.parent, nodes_idx.child);
            if nodes_idx.has_child() {
                recheck_child(v, &nodes_idx);
            }
        }
        nodes_idx.child = nodes_idx.child + 1;
    }
}

fn recheck_child(v: &mut Vec<i32>, nodes_idx: &NodesIdx) {
    let mut nodes_idx_down = NodesIdx {
        parent: nodes_idx.child,
        child: 0,
        node_count: nodes_idx.node_count
    };
    nodes_idx_down.calc_child_idx();
    comp_and_swap(v, &mut nodes_idx_down)
}
