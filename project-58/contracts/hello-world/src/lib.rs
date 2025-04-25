#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, String, log, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Song {
    pub id: u64,
    pub title: String,
    pub artist: String,
    pub lyrics: String,
    pub submitter: Address,
}

const SONG_COUNT : soroban_sdk::Symbol= symbol_short!("S_COUNT");

#[contract]
pub struct LyricsCollector;

#[contractimpl]
impl LyricsCollector {
    // Add a new song (immutable once submitted)
    pub fn add_song(
        env: Env,
        submitter: Address,
        title: String,
        artist: String,
        lyrics: String,
    ) -> u64 {
        submitter.require_auth();

        let mut count: u64 = env.storage().instance().get(&SONG_COUNT).unwrap_or(0);
        count += 1;

        let song = Song {
            id: count,
            title,
            artist,
            lyrics,
            submitter: submitter.clone(),
        };

        env.storage().instance().set(&count, &song);
        env.storage().instance().set(&SONG_COUNT, &count);

        log!(&env, "Song {} added by {}", count, submitter);

        count
    }

    // Get song by ID
    pub fn get_song(env: Env, song_id: u64) -> Song {
        env.storage().instance().get(&song_id).unwrap()
    }

    // Get total number of songs
    pub fn get_song_count(env: Env) -> u64 {
        env.storage().instance().get(&SONG_COUNT).unwrap_or(0)
    }
}
