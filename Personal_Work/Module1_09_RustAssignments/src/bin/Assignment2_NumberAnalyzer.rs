fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [12, 7, 9, 20, 15, 2, 33, 10, 8, 25];

    // FOR LOOP
    for n in numbers {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{n}: FizzBuzz");
        } else if n % 3 == 0 {
            println!("{n}: Fizz");
        } else if n % 5 == 0 {
            println!("{n}: Buzz");
        } else if is_even(n) {
            println!("{n}: even");
        } else {
            println!("{n}: odd");
        }
    }

    // WHILE LOOP (sum)
    let mut i = 0;
    let mut sum = 0;

    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }

    println!("Sum = {sum}");

    // LOOP (largest number)
    let mut index = 0;
    let mut largest = numbers[0];

    loop {
        if numbers[index] > largest {
            largest = numbers[index];
        }

        index += 1;

        if index >= numbers.len() {
            break;
        }
    }

    println!("Largest = {largest}");
}
