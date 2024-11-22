use ic_cdk_macros::*;
use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
struct Wallet {
    balance: u64,
    owner: String,
}

thread_local! {
    static WALLET: std::cell::RefCell<Wallet> = std::cell::RefCell::new(Wallet {
        balance: 0,
        owner: String::new(),
    });
}

#[init]
fn init() {
    let caller = ic_cdk::caller().to_string();
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        w.owner = caller;
    });
}

fn set_balance_internal(new_balance: u64, caller: String) -> Result<(), String> {
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        if caller != w.owner {
            return Err("Unauthorized".to_string());
        }
        w.balance = new_balance;
        Ok(())
    })
}

#[update]
fn set_balance(new_balance: u64) -> Result<(), String> {
    set_balance_internal(new_balance, ic_cdk::caller().to_string())
}

fn send_tokens_internal(amount: u64, to: Option<String>, caller: String) -> Result<(), String> {
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        if w.owner != caller {
            return Err("Unauthorized".to_string());
        }
        if w.balance < amount {
            return Err("Insufficient balance".to_string());
        }
        w.balance -= amount;
        ic_cdk::println!("Remaining balance after sending: {}", w.balance);
        if let Some(to_address) = to {
            // Implement token transfer logic here
            println!("Tokens sent to: {}", to_address);
        }
        Ok(())
    })
}

#[update]
fn send_tokens(amount: u64, to: Option<String>) -> Result<(), String> {
    send_tokens_internal(amount, to, ic_cdk::caller().to_string())
}

fn receive_tokens_internal(amount: u64, caller: String) -> Result<(), String> {
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        if caller != w.owner {
            return Err("Unauthorized".to_string());
        }
        w.balance += amount;
        ic_cdk::println!("The balance after recieving: {}", w.balance);
        Ok(())
    })
}

#[update]
fn receive_tokens(amount: u64) -> Result<(), String> {
    receive_tokens_internal(amount, ic_cdk::caller().to_string())
}

#[query]
fn get_balance() -> u64 {
    WALLET.with(|wallet| wallet.borrow().balance)
}

#[update]
fn set_owner(new_owner: String) {
    WALLET.with(|wallet| {
        let mut w = wallet.borrow_mut();
        w.owner = new_owner.clone();
        ic_cdk::println!("New owner set to: {}", new_owner);
    });
}

#[query]
fn get_owner() -> String {
    WALLET.with(|wallet| wallet.borrow().owner.clone())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> String {
        let owner = "test_owner".to_string();
        WALLET.with(|wallet| {
            let mut w = wallet.borrow_mut();
            w.owner = owner.clone();
            w.balance = 0;
        });
        owner
    }

    #[test]
    fn test_get_balance() {
        setup();
        assert_eq!(get_balance(), 0, "Initial balance should be 0");
    }

    #[test]
    fn test_set_and_get_balance() {
        let owner = setup();
        set_balance_internal(100, owner).unwrap();
        assert_eq!(get_balance(), 100, "Balance should be updated to 100");
    }

    #[test]
    fn test_get_owner() {
        let owner = setup();
        assert_eq!(get_owner(), owner, "Owner should be set to test_owner");
    }

    #[test]
    fn test_set_owner() {
        setup();
        set_owner("new_owner".to_string());
        assert_eq!(get_owner(), "new_owner", "Owner should be updated to new_owner");
    }

    #[test]
    fn test_send_tokens_sufficient_balance() {
        let owner = setup();
        set_balance_internal(100, owner.clone()).unwrap();
        let result = send_tokens_internal(50, Some("recipient".to_string()), owner);
        assert!(result.is_ok(), "Sending tokens should succeed with sufficient balance");
        assert_eq!(get_balance(), 50, "Balance should be reduced after sending tokens");
    }

    #[test]
    fn test_send_tokens_insufficient_balance() {
        let owner = setup();
        set_balance_internal(100, owner.clone()).unwrap();
        let result = send_tokens_internal(150, Some("recipient".to_string()), owner);
        assert!(result.is_err(), "Sending tokens should fail with insufficient balance");
        assert_eq!(get_balance(), 100, "Balance should remain unchanged");
    }

    #[test]
    fn test_receive_tokens() {
        let owner = setup();
        let result = receive_tokens_internal(50, owner);
        assert!(result.is_ok(), "Receiving tokens should succeed");
        assert_eq!(get_balance(), 50, "Balance should be increased after receiving tokens");
    }

    #[test]
    fn test_unauthorized_access() {
        let owner = setup();
        set_balance_internal(100, owner).unwrap();
        let unauthorized = "unauthorized".to_string();
        
        let result = set_balance_internal(200, unauthorized.clone());
        assert!(result.is_err(), "Unauthorized set_balance should fail");
        
        let result = send_tokens_internal(50, Some("recipient".to_string()), unauthorized.clone());
        assert!(result.is_err(), "Unauthorized send_tokens should fail");
        
        let result = receive_tokens_internal(50, unauthorized);
        assert!(result.is_err(), "Unauthorized receive_tokens should fail");
        
        assert_eq!(get_balance(), 100, "Balance should remain unchanged after unauthorized attempts");
    }
}