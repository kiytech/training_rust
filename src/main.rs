use rand::Rng;

const ARRAY_SIZE: usize = 10;
fn main() {
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let mut count = 0;
    while count < ARRAY_SIZE-1 {
        array[count] = rand::thread_rng().gen_range(0..100);
        count+=1;
    }
    heap_sort(array.to_vec());
    println!("Generate rand array: {:?}", array);
    println!("Max: {:?}", ind_max(array.to_vec()));
}

fn heap_sort(v:Vec<i32>) -> Vec<i32> {
    return v
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
