fn main() {
    mutable_refarence_x2();
}

fn copy_semantics() {
    let s = 1;
    let t = s;
    println!("s = {}", s);
    println!("t = {}", t);
}

fn move_semantics_f() {
    /*
    let s = "Hello".to_string();
    let t = s;
    println!("s = {}", s);
    println!("t = {}", t);
    */
}

fn move_ownership_by_function_aug_primitive() {
    let s = 1;
    myprint(s);
    myprint(s);

    fn myprint<T: std::fmt::Display>(msg: T) {
        println!("{}", msg);
    }
}

fn move_ownership_by_function_aug_f() {
    /*
    let s = "Hello".to_string();
    myprint(s);
    myprint(s);

    fn myprint<T: std::fmt::Display>(msg: T) {
        println!("{}", msg);
    }
    */
}

fn move_ownership_by_function_aug_clone() {
    let s = "Hello".to_string();
    let ss = s.clone();
    myprint(s);
    myprint(ss);

    fn myprint<T: std::fmt::Display>(msg: T) {
        println!("{}", msg);
    }
}

fn move_ownership_by_function_aug_reference() {
    let s = "Hello".to_string();
    myprint(&s);
    myprint(&s);

    fn myprint<T: std::fmt::Display>(msg: &T) {
        println!("{}", *msg);
    }
}

fn mutable_refarence() {
    let mut s = "Hello".to_string();
    println!("s = {}", s);

    // s.clear();
    myclear(&mut s);
    println!("s = {}", s);

    fn myclear(x: &mut String) {
        x.clear();
    }
}

fn mutable_refarence_x2_f() {
    /*
    let mut s = "Hello".to_string();
    println!("s = {}", s);

    let s_ref = &mut s;
    let s_ref2 = &mut s;
    myclear(s_ref);
    println!("s = {}", s);

    fn myclear(x: &mut String) {
        x.clear();
    }
    */
}

fn show_mem_addr(){
    let x = 2;
    println!("x addr = {:p}", &x);
}

fn mutable_refarence_x2() {
    let mut s = "Hello".to_string();
    println!("s = {}", s);

    let s_ref = &mut s;
    myclear(s_ref);
    println!("s = {}", s);

    let s_ref2 = &mut s;
    println!("s = {}", s);

    fn myclear(x: &mut String) {
        x.clear();
    }
}

fn change_mut_reference_f() {
    let mut x = 1;
    let x_ref = &x;

    x = 2;
    println!("x = {}", x_ref);
}
