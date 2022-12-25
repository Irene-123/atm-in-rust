use std::io::{stdin, stdout, BufRead, Error, Write};
use std::fmt::{self, Display};
fn main() {
    let mut account= Account::new(); 
    println!("Enter your bank account name: \n");
    stdin().read_line(&mut account.name);

    // Answer to the query requested by user. 
    println!("Depositing or withdrawing? \n1: Deposit\n2: Withdraw\n3: Exit\n"); 
    let mut ans= String::new(); 

    loop {
        // Submit a request for a bank action
        match BankAction::request(&mut ans, "Depositing or withdrawing? (1 = Deposit; 2 = Withdraw; 3 = Exit): ") {
            // Ask for a deposit amount and deposit that amount into the account, if the amount is valid.
            Ok(BankAction::Deposit) => {
                match Currency::request(&mut ans, "Enter amount to deposit: ") {
                    Ok(amount) => account.deposit(amount),
                    Err(why) => eprintln!("{}", why)
                }
            },
            // Ask for a withdrawal amount and attempt to withdraw that amount from the account, if it's available.
            Ok(BankAction::Withdraw) => {
                match Currency::request(&mut ans, "Enter amount to withdraw: ") {
                    Ok(amount) => account.Withdraw(amount),
                    Err(why) => eprintln!("{}", why)
                };
            },
            // Exit the program
            Ok(BankAction::Exit) => break,
            // An error occurred when reading the bank action.
            Err(why) => eprintln!("Error occurred: {}", why)
        }
    }

}


struct Account{
    name: String, 
    balance: Currency
}

enum  BankAction {
    Deposit, Withdraw, Exit
}
enum BankError {
    IO(Error),
    InvalidString(String),
    InvalidAction(usize)
}
#[derive(Clone, Copy)]
struct Currency(usize);

/// A possible error that may be returned by a `BankAction::request()`.
enum CurrencyError {
    IO(Error),
    Invalid(String)
}

impl Account{
    fn new()-> Account{
        Account { name: String::new() , balance: Currency(0) }
    }

    fn deposit(&mut self, amount: Currency) {
        self.balance.0+= amount.0; 
        println!("Deposited {} into account. (Balance: {})", amount, self.balance);
    }
    fn Withdraw(&mut self, amount: Currency){
        match self.balance.0.checked_sub(amount.0) {
            Some(new_balance) => {
                self.balance.0= new_balance; 
            }, 
            None => eprintln!("Error: Funds not available in account. (Balance: {})", self.balance), 
        }
    }
   
    
}
trait ATMRequest {
    type Success;
    type Error;
    fn request(ans: &mut String, msg: &str)-> Result<Self::Success, Self::Error>; 
}

impl ATMRequest for BankAction {
    type Success = BankAction;
    type Error = BankError;
    fn request(buffer: &mut String, msg: &str)-> Result<BankAction, BankError> {
        use BankError::*;

        // Print the supplied dialogue.
        print!("{}", msg);
        let stdout = stdout();
        let _ = stdout.lock().flush();

        // Clear the buffer first
        buffer.clear();
        // Read the value from standard input.
        let stdin = stdin();
        stdin.lock().read_line(buffer);
        // Remove the newline read at the end.
        buffer.pop().unwrap();

        // Match the parsed input to it's corresponding action.
        
        buffer.parse::<u8>().map_err(|_| InvalidString(buffer.clone()))
            .and_then(|action| match action {
                1 => Ok(BankAction::Deposit),
                2 => Ok(BankAction::Withdraw),
                3 => Ok(BankAction::Exit),
                answer => Err(InvalidAction(answer as usize))
            })
    }
}


impl ATMRequest for Currency {
    type Success = Currency;
    type Error = CurrencyError;
    fn request(buffer: &mut String, msg: &str) -> Result<Currency, CurrencyError> {
        use CurrencyError::*;

        // Print the supplied dialogue.
        print!("{}", msg);
        let stdout = stdout();
        let _ = stdout.lock().flush();

        // Clear the buffer first
        buffer.clear();
        // Read the value from standard input.
        let stdin = stdin();
        stdin.lock().read_line(buffer)?;
        // Remove the newline read at the end.
        buffer.pop().unwrap();

        // Match the buffer to it's corresponding Currency result. Because currencies may contain
        // decimal places, special handling needs to happen in order to parse the value.
        match buffer.find('.') {
            Some(position) => {
                // Split the buffer at the position where the decimal point was found.
                let (major, mut minor_str) = buffer.split_at(position);
                // Remove the decimal point from the minor string.
                minor_str = &minor_str[1..];
                // Parse the major value, mapping the error value accordingly
                major.parse::<usize>().map_err(|_| Invalid(buffer.clone()))
                    // If the parse succeeded, we will next parse the minor string
                    .and_then(|major| minor_str.parse::<usize>().map_err(|_| Invalid(buffer.clone())).and_then(|minor| {
                        // If there's only one character, it's because that number is a tenth.
                        let minor = if minor_str.len() == 1 { minor * 10 } else { minor };
                        // Don't allow anyone to cheat the machine.
                        if minor > 99 {
                            Err(Invalid(buffer.clone()))
                        } else {
                            // A major value is 100 times the value of a minor, so add them up accordingly.
                            Ok(Currency(minor + (major * 100)))
                        }
                    }).map_err(|_| Invalid(buffer.clone())))
            },
            None => buffer.parse::<usize>().map(|v| Currency(v * 100))
                .map_err(|_| Invalid(buffer.clone()))
        }
    }
}

impl From<Error> for BankError {
    fn from(error: Error) -> BankError { BankError::IO(error) }
}

impl From<Error> for CurrencyError {
    fn from(e: Error) -> CurrencyError { CurrencyError::IO(e) }
}

impl Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BankError::IO(ref err) => write!(f, "error: standard input error: {}", err),
            BankError::InvalidString(ref string) => write!(f, "error: invalid input supplied: {}", string),
            BankError::InvalidAction(ref action) => write!(f, "error: no operation is mapped to {}", action)
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${}.{:02}", self.0 / 100, self.0 % 100)
    }
}

impl Display for CurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CurrencyError::IO(ref error) => write!(f, "error: standard input error: {}", error),
            CurrencyError::Invalid(ref string) => write!(f, "error: {} is not a valid amount", string),
        }
    }
}