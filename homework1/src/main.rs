const FREZZING: f64 = 32.0;

fn ftoc(f: f64) -> f64{
        //converts fahrenheit to celsius
        (f - 32.0) * 5.0 / 9.0
    }

fn ass1() {
    println!("Problem #1");
    let mut f = FREZZING;

    let c = ftoc(f);
    println!("{f}°F = {c:.2}°C");

    for _ in 0..5 {
        f += 1.0;
        let c = ftoc(f);
        println!("{f}°F = {c:.2}°C");
        
    }

}

fn is_even(n: i32) -> bool{
    n % 2 == 0
}

fn ass2(){
    println!("\nProblem #2");
    let numbers = [3,10,15,25,21,5,38,30,7,9];

    for &num in numbers.iter(){
        if num % 3 == 0 && num % 5 == 0 {
            println!("{num}: FizzBuzz");
        }
        else if num % 3 == 0 {
            println!("{num}: Fizz");
        }
        else if num % 5 == 0 {
            println!("{num} Buzz");
        }
        else{
            if is_even(num){
                println!("{num}: Even");
            }
            else {
                println!("{num}: Odd");
            }
        }
    }//end of for loop

    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len(){
        sum += numbers[index];
        index += 1;
    }

    println!("\nSum of all numbers: {sum}");

    let mut largest = numbers[0];

    for &num in numbers.iter(){
        if num > largest {
            largest = num;
        }
    }

    println!("Largest Number: {largest}");
}

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        return 0;
    }
    else if guess > secret{
        return 1;
    }
    else {
        return -1;
    }
}

fn ass3(){
    println!("\nProblem #3");
    let secret: i32 = 42;

    let guesses: [i32; 7] = [25,50,30,40,45,42,60];
    let mut guess_count: i32 = 0;

    for i in 0..guesses.len(){
        guess_count += 1;

        let current_guess: i32 = guesses[i];
        let result: i32 = check_guess(current_guess, secret);

        if result == 0{
            println!("Guess {}: {} is correct!",guess_count, guesses[i]);
            break;
        } else if result == 1 {
            println!("Guess {}: {} is too high.", guess_count, guesses[i]);
        }
        else {
            println!("Guess {}: {} is too low.", guess_count, guesses[i]);
        }
    }

    println!("Number of guesses: {}", guess_count);
}

fn main() {
    ass1();
     ass2();
      ass3();
}
