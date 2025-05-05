#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct Donation {
    pub donor: String,
    pub amount: u64,
    pub message: String,
}

#[contracttype]
#[derive(Clone)]
pub struct MealService {
    pub meals_served: u64,
    pub funds_collected: u64,
}

const DONATION_LOG: Symbol = symbol_short!("DON_LOG");
const SERVICE_STATUS: Symbol = symbol_short!("SERVICE");

#[contract]
pub struct DonationMealServiceContract;

#[contractimpl]
impl DonationMealServiceContract {
    // Donate funds with optional message
    pub fn donate(env: Env, donor: String, amount: u64, message: String) {
        let mut service = Self::get_service_status(env.clone());
        let donation = Donation { donor: donor.clone(), amount, message };

        service.funds_collected += amount;

        env.storage().instance().set(&DONATION_LOG, &donation);
        env.storage().instance().set(&SERVICE_STATUS, &service);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Donation received from {} of amount {}", donor, amount);
    }

    // Admin uses funds to serve meals
    pub fn serve_meals(env: Env, count: u64) {
        let mut service = Self::get_service_status(env.clone());
        service.meals_served += count;

        env.storage().instance().set(&SERVICE_STATUS, &service);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "{} meals served!", count);
    }

    // View current stats
    pub fn get_service_status(env: Env) -> MealService {
        env.storage().instance().get(&SERVICE_STATUS).unwrap_or(MealService {
            meals_served: 0,
            funds_collected: 0,
        })
    }
}

