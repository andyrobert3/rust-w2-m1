// create a function is_this_cash which takes in string and returns Result of String, String
// if input of string equals to "cash", return Ok of "Yes this is cash"
// else return Err of "No, this is not cash"

fn is_this_cash(input: String) -> Result<String, String> {
    if input == "cash" {
        Ok("Yes this is cash".to_string())
    } else {
        Err("No, this is not cash".to_string())
    }
}

fn main() {
    // create variable cash of type Option<String>, assign "cash"
    let cash: Option<String> = Some("cash".to_owned());

    // create variable credit of type Option<String>, assign "credit"
    let credit: Option<String> = Some("credit".to_owned());

    // using if let on cash, bind variable within Some
    if let Some(payment_type) = cash {
        match is_this_cash(payment_type) {
            Ok(msg) => println!("Output {}", msg),
            Err(msg) => println!("Output {}", msg)
        }
    } 

    // pass the binded variable into is_this_cash function and use match

    // if matches on OK, print "Output X", where X the value within OK

    // if matches on Err, print "Output X", where X the value within Err

    // do the same as above on credit
    if let Some(payment_type) = credit {
        match is_this_cash(payment_type) {
            Ok(msg) => println!("Output {}", msg),
            Err(msg) => println!("Output {}", msg)
        }
    } 
}
