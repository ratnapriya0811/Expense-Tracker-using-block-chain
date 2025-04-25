#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, String, Symbol, symbol_short};

// Unique symbol key for tracking expense count
const EXPENSE_COUNT: Symbol = symbol_short!("EX_COUNT");

// Expense struct to store each expense entry
#[contracttype]
#[derive(Clone)]
pub struct Expense {
    pub id: u64,
    pub title: String,
    pub amount: u64,
    pub timestamp: u64,
}

// Enum key to store expenses in storage
#[contracttype]
pub enum ExpenseBook {
    Entry(u64),
}

// Main contract struct
#[contract]
pub struct ExpenseTrackerContract;

#[contractimpl]
impl ExpenseTrackerContract {
    // Create a new expense entry
    pub fn add_expense(env: Env, title: String, amount: u64) -> u64 {
        let mut count: u64 = env.storage().instance().get(&EXPENSE_COUNT).unwrap_or(0);
        count += 1;

        let expense = Expense {
            id: count,
            title,
            amount,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&ExpenseBook::Entry(count), &expense);
        env.storage().instance().set(&EXPENSE_COUNT, &count);

        count
    }

    // View a specific expense entry by ID
    pub fn view_expense(env: Env, id: u64) -> Expense {
        env.storage()
            .instance()
            .get(&ExpenseBook::Entry(id))
            .unwrap_or(Expense {
                id: 0,
                title: String::from_str(&env, "Not Found"),
                amount: 0,
                timestamp: 0,
            })
    }

    // Get the total number of expenses
    pub fn get_total_expenses(env: Env) -> u64 {
        env.storage().instance().get(&EXPENSE_COUNT).unwrap_or(0)
    }
}
