/*
バブルソートアルゴリズムを Rust で実装してください。
仕様: -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
*/
use rand::Rng;

fn main() {
    const ARRAY_SIZE: usize = 100;
    let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE]; 
    let mut count = 0;
    loop {
        if count > ARRAY_SIZE-1 { break; }
        array[count] = rand::thread_rng().gen_range(0, 1001);
        println!("配列{}: {}", count, array[count]);
        count+=1;
    }
}
