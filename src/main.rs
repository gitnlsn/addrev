use std::io;

fn reverse(mut number: usize) -> usize {
    let mut reversed = 0;
    while number > 0 {
        let digit = number % 10;
        reversed = reversed * 10 + digit;
        number = number / 10;
    }
    return reversed;
}

fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed at read_line");
    return input.trim().parse().expect("Failed to parse");
}

fn read_tuple() -> (usize, usize) {
    let input: Vec<usize> = get_input()
        .split(" ")
        .map(|x| x.parse().expect("Failed to parse interger"))
        .collect();

    return (input[0], input[1]);
}

#[test]
fn test_reverse() {
    assert!(reverse(123) == 321);
    assert!(reverse(1324) == 4231);
    assert!(reverse(1000) == 1);
}

#[test]
fn foo() {}

fn main() {
    let total_test: u128 = get_input().parse()
        .expect("Failed to convert integer");
        
    for _i in 0..total_test {
        let (a, b) = read_tuple();
        let reversed_a = reverse(a);
        let reversed_b = reverse(b);
        let reversed_sum = reversed_a + reversed_b;
        let sum = reverse(reversed_sum);
        println!("{}", sum);
    }
}
