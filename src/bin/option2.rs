struct Stock {
    name: String,
    qty: i32,
 }
 
 // find_quantity finds a stock by its string name, and returns its quantity in our investment portfolio. It returns Option::None if no stock item matches the provided name
 fn find_quantity(name: &str) -> Option<i32> {
    let portfolio = vec![
        Stock {name: "TSLA".to_owned(), qty: 12, },
        Stock {name: "GME".to_owned(), qty: 1, },
    ];
    for item in portfolio { // iterate thru each stock in portfolio
        if item.name == name {
            // this returns Option::Some with a value in it
            return Some(item.qty);
        }
    }
    None    // this returns Option::None,
 }
 
 fn main() {

    match find_quantity("MSFT") {
        Some(x) => println!("Quantity is {}", x),
        None => println!("No such stock in portfolio"),
    };
    // The above will print "12
    // Changing "TSLA" to "MSFT" prints "No such stock in portfolio"
 }
 
 