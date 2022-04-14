use rand::Rng;

const ARRAY_SIZE: usize = 10;
fn main() {
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let mut count = 0;
    loop {
        if count > ARRAY_SIZE-1 {
            break;
        }
        array[count] = rand::thread_rng().gen_range(0..100);
        count+=1;
    }
    heap_sort(array.to_vec());
    println!("Generate rand array: {:?}", array);
}

fn heap_sort(v:Vec<i32>) -> Vec<i32> {
    return v
}

#[allow(dead_code)]
fn min_ind(v:Vec<i32>) -> (usize, i32) {
    v.iter()
        .enumerate()
        .fold((usize::MAX, i32::MAX), |(i_a, a), (i_b, &b)| {
            if b < a {
                (i_b, b)
            } else {
                (i_a, a)
            }
        })
}
