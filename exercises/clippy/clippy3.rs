// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_none() {
    //     my_option.unwrap();
    // }
    // 使用 `match` 或 `if let` 来处理 `Option`
    if my_option.is_none() {
        // 这里不需要 `unwrap`，因为我们知道它是 `None`
        println!("Option is None");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
