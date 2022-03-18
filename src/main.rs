/*
 ** 挿入ソート
 ** -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
*/
use rand::Rng;

const ARRAY_SIZE: usize = 5;
fn main() {
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
    let mut count = 0;
    print!("Init:   ");
    loop {
        if count > ARRAY_SIZE-1 {
            println!();
            break;
        }
        array[count] = rand::thread_rng().gen_range(0, 1001);
        print!("[{}] ", array[count]);
        count+=1;
    }
    print!("Result: ");
    for i in 0..ARRAY_SIZE-1 {
        let (min_index, _min) = min_ind(array[i ..].to_vec());
        array.swap(i, i+min_index);
        print!("[{}] ", array[i]);
    }
    println!("[{}]", array[ARRAY_SIZE-1]);
}

fn min_ind (v: Vec<i32>) -> (usize, i32) {
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
