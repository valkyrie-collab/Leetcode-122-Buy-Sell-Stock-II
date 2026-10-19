use std::io::{self, Write};

fn buy_sell(stocks: &[i32]) -> i32 {
    let mut min: i32 = stocks[0];
    let mut max_price: i32 = 0;
    let mut cur_price: i32 = 0;

    for &xs in stocks {

        if xs < min {
            min = xs;
        }

        let temp_price: i32 = xs - min;

        if temp_price >= cur_price {
            cur_price = temp_price;
        } else if temp_price < cur_price {
            max_price = max_price + cur_price;
            cur_price = 0;
            min = xs;
        }

        println!("min {} | max {} | cur_price {} | max_price {} ", min, xs, cur_price, max_price);
    }

    if cur_price != 0 {
        return max_price + cur_price;
    }

    max_price
}

fn main() {
    let mut buf: String = String::new();

    print!("Enter the quantity of stocks ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let size: usize = buf.trim().parse().unwrap();
    let mut stocks: Vec<i32> = Vec::with_capacity(size);

    println!("Enter the value of the stock array below");

    for xi in 0..size {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        stocks[xi] = buf.trim().parse().unwrap();
    }

    let val: i32 = buy_sell(&stocks);
    
    println!("value of the stock: {}", val);
}
