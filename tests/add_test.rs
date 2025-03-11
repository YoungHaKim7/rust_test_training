use rust_test_training::two_add;

#[test]
fn add_fn() {
    let x_input = 10;
    println!("input + input = {}", two_add(x_input));
}
