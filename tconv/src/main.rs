use std::io;

fn main() {

    const RL_ERR : &str = "Unable to read line";

    loop {
        
        println!("Choose conv procedure ( 1 or 2 ):");
        println!("1) C to F");
        println!("2) F to C");
        println!("3) Exit program");
        
        let mut procedure = String::new();

        io::stdin().read_line(&mut procedure).expect(&RL_ERR);
    
        let procedure: u8 = match procedure.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("You must enter a positive number");
                continue;
            }
        };

        if procedure < 1 || procedure > 3 {
            println!("The number must be between 1 and 3");
            continue;
        }

        if procedure == 3 { 
            break;
        }

        println!("Please enter a temperature");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp).expect(&RL_ERR);

        let mut temp : f64 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Error converting temperature to f64");
                continue;
            }
        };
       
        if procedure == 1 {
            temp = temp * 1.8 + 32.0;
        } else {
            temp = (temp - 32.0) * 0.5556;
        }


        println!("The temperature is {}", temp);
    }

    println!("Bye bye");
    
}
