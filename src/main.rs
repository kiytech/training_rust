/*
バブルソートアルゴリズムを Rust で実装してください。
仕様: -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
*/
use rand::Rng;

fn main() {
    const ARRAY_SIZE: usize = 5;
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE]; 
    let mut count = 0;
    loop {
        if count > ARRAY_SIZE-1 {
            println!();
            break;
        }
        array[count] = rand::thread_rng().gen_range(0, 1001);
        print!("[{}] ", array[count]);
        count+=1;
    }

    for (i, &element) in array.iter().enumerate() {
        if i > ARRAY_SIZE-2 { break; }
        if element >= array[i+1] {
            array.swap(i, i+1);
        }
        println!("i: {}, elm: {}, next: {}", i, element, array[i+1]);
    }
}
