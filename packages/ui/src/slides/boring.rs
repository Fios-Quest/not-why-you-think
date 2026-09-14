use crate::common::notes::Notes;
use dioxus::prelude::*;

const TYPESCRIPT: &str = r#"
const getPokemon = async (id: number): Promise<Result<Pokemon, Error>> => {
    const response = await fetch(`https://pokeapi.co/api/v2/pokemon/${id}`);
    const pokemon = await response.json();
    return pokemon.id === id
        ? Result.ok(pokemon)
        : Result.error(new Error('Incorrect Pokémon returned'));
}
"#;

const RUST: &str = r#"
async fn get_pokemon(id: u16) -> Result<User, GetPokemonError> {
    let pokemon: Pokemon = get(format!("https://pokeapi.co/api/v2/pokemon/{id}")).await?
        .json().await?;
    if pokemon.id == id {
        Ok(pokemon)
    } else {
        Err(GetPokemonError::IncorrectPokemonReturned)
    }
}
"#;

#[component]
pub fn RustVsTypeScript() -> Element {
    rsx! {
        section {
            "TypeScript"
            pre {
                code {
                    class: "language-typescript",
                    "data-trim": true,
                    "data-line-numbers": "|1|2|3|4|",
                    {TYPESCRIPT}
                }
            }
            div { class: "fragment",
                "Rust"
                pre {
                    code {
                        class: "fragment language-rust",
                        "data-trim": true,
                        "data-fragment-index": "0",
                        "data-line-numbers": true,
                        {RUST}
                    }
                }
            }

            Notes {
                notes: vec![
                    "Picking on TS again".into(),
                    "Perfectly good TypeScript".into(),
                    "3 errors".into(),
                    "4 if you include a duplicate".into(),
                    "Maybe more".into(),
                    "number, exceptions, parsing".into(),
                    "Rust for comparison".into(),
                ],
            }
        }
    }
}

#[component]
pub fn ItsBoring() -> Element {
    rsx! {
        section {
            section {
                h2 { "It's Boring" }

                Notes {
                    notes: vec![
                        "Rust is boring".into(),
                        "Or unsurprising".into(),
                        "Let's pick on TypeScript again".into(),
                    ],
                }
            }

            RustVsTypeScript {}
        }
    }
}
