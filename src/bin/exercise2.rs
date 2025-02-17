use std::collections::HashMap;
fn main() { 
  let vec = vec![1,2,3,4,5,6,7,8,9,10];
  // todo: Create an iterator from the vector above.
  let iter = vec.iter();

  // todo: Using filter() to filter for numbers > 5, and use map() to multiply their values by 2.
  // todo: Save the values into a new vector called vec_even\
  let vec_even: Vec<_> = iter.filter(|x| **x > 5).map(|x| x * 2).collect();

  // todo: print vec_even
  println!("vec_even: {:?}", vec_even);
 
  let mut stock_prices = HashMap::new();
  stock_prices.insert("TSLA", 20);
  // todo: insert a few more stock_prices, less or more than 50 dollars
  stock_prices.insert("APPL", 50);
  stock_prices.insert("JPM", 70);
  stock_prices.insert("DOGE", 30);
  stock_prices.insert("AMC", 40);
  stock_prices.insert("HOOD", 10);
  stock_prices.insert("GME", 0);

  // todo: use a for loop to print out the name of each stock that costs less than 50 dollars
  for (name, price) in stock_prices.iter().filter(|stock| *(stock.1) < 50) {
    println!("Stock price name: {}", name);
  }
   
}
