/*
バブルソートアルゴリズムを Rust で実装してください。
仕様: -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
*/
use rand::Rng;

const ARRAY_SIZE: usize = 100;

fn main() {
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE]; 
    let mut count = 0;
    print!("Init: ");
    loop {
        if count > ARRAY_SIZE-1 {
            println!();
            break;
        }
        array[count] = rand::thread_rng().gen_range(0, 1001);
        print!("[{}] ", array[count]);
        count+=1;
    }

    for j in 0..ARRAY_SIZE-1 { 
        let mut is_swapped = false;
        print!(" ({}): ", j);
        for i in 0..ARRAY_SIZE {
            if i < ARRAY_SIZE-1 {
                if array[i] >= array[i+1] {
                    array.swap(i, i+1);
                    is_swapped = true;
                }
            }
            print!("[{}] ", array[i]);
        }
        println!();
        if !is_swapped { 
            break;
        };
    }
}
