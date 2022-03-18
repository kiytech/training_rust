/*
 ** 挿入ソート
 ** -1000 以上 1000 以下のランダムな整数を100回発生させ、その整数を昇順に並び替える。
*/
use rand::Rng;

const ARRAY_SIZE: usize = 100;
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
    print!("\nResult: ");
    for target_ind in 1..ARRAY_SIZE {
        for comp_ind in 0..target_ind {
            if array[target_ind] < array[comp_ind] {
                array.swap(target_ind, comp_ind);
            }
        }
    }
    for i in 0..ARRAY_SIZE {
        print!("[{}] ", array[i]);
    }
    println!();
}
