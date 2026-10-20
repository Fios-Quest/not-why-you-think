use reqwest::get;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    id: u16,
    name: String,
    base_experience: u16,
    height: u16,
    is_default: bool,
    order: u16,
    weight: u16,
}

#[derive(Debug, Error)]
enum GetPokemonError {
    #[error("Incorrect Pokemon Returned by API")]
    IncorrectPokemonReturned,
    #[error("Reqwest error {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Parsing error {0}")]
    JsonError(#[from] serde_json::Error),
}

async fn get_pokemon(id: u16) -> Result<Pokemon, GetPokemonError> {
    let pokemon: Pokemon = get(format!("https://pokeapi.co/api/v2/pokemon/{id}"))
        .await?
        .json()
        .await?;
    if pokemon.id == id {
        Ok(pokemon)
    } else {
        Err(GetPokemonError::IncorrectPokemonReturned)
    }
}

#[tokio::main]
async fn main() {
    let pokemon = get_pokemon(1000).await.unwrap();
    dbg!(pokemon);
}
