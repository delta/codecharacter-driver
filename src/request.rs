use std::os::fd::RawFd;

use serde::de;
use serde::Deserialize;
use serde::Deserializer;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Attacker {
    pub id: u32,
    pub hp: u32,
    pub range: u32,
    pub attack_power: u32,
    pub speed: u32,
    pub price: u32,
    pub is_aerial: u32,
    pub weight: u32,
    pub num_ability_turns: u32,
    pub ability_activation_cost: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Defender {
    pub id: u32,
    pub hp: u32,
    pub range: u32,
    pub attack_power: u32,
    pub price: u32,
    pub is_aerial: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct GameParameters {
    pub attackers: Vec<Attacker>,
    pub defenders: Vec<Defender>,
    pub no_of_turns: u32,
    pub no_of_coins: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PvPGameParameters {
    pub attackers: Vec<Attacker>,
    pub defenders: Vec<Defender>,
    pub no_of_turns: u32,
    pub no_of_coins: u32, // no of coins per turn
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Language {
    CPP,
    JAVA,
    PYTHON,
}

pub enum GameRequest {
    NormalGame(NormalGameRequest),
    PvPGame(PvPGameRequest),
}

impl From<NormalGameRequest> for GameRequest {
    fn from(request: NormalGameRequest) -> Self {
        GameRequest::NormalGame(request)
    }
}

impl From<PvPGameRequest> for GameRequest {
    fn from(request: PvPGameRequest) -> Self {
        GameRequest::PvPGame(request)
    }
}

impl GameRequest {
    pub fn game_id(&self) -> &String {
        match self {
            GameRequest::NormalGame(req) => &req.game_id,
            GameRequest::PvPGame(req) => &req.game_id,
        }
    }
}

pub struct PvPPipeFds {
    pub p1_in: RawFd,
    pub p2_in: RawFd,
    pub p1_out: RawFd,
    pub p2_out: RawFd,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct NormalGameRequest {
    pub game_id: String,
    pub parameters: GameParameters,
    pub player_code: PlayerCode,
    #[serde(deserialize_with = "deserialize_from_str")]
    pub map: Vec<Vec<u8>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PlayerCode {
    pub source_code: String,
    pub language: Language,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PvPGameRequest {
    pub game_id: String,
    pub parameters: PvPGameParameters,
    pub player1: PlayerCode,
    pub player2: PlayerCode,
}

// Reference: https://serde.rs/attr-bound.html
fn deserialize_from_str<'de, D>(deserializer: D) -> Result<Vec<Vec<u8>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(de::Error::custom)
}

#[cfg(test)]
mod tests {

    use crate::request::PlayerCode;

    // TODO: Test the pvp desearialization
    use super::{
        Attacker, Defender, GameParameters, NormalGameRequest, PvPGameParameters, PvPGameRequest,
    };
    #[test]
    pub fn deserealization_test() {
        // An example request that we might get from backend for a normal game
        let example_request_normal_game = r#"{"game_id":"0fa0f12d-d472-42d5-94b4-011e0c916023","parameters":{"attackers":[{"id":1,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1,"is_aerial":0,"weight":1,"num_ability_turns":2,"ability_activation_cost":2},{"id":2,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1,"is_aerial":1,"weight":2,"num_ability_turns":2,"ability_activation_cost":3}],"defenders":[{"id":1,"hp":10,"range":4,"attack_power":5,"price":1,"is_aerial":1},{"id":2,"hp":10,"range":6,"attack_power":5,"price":1,"is_aerial":1}],"no_of_turns":500,"no_of_coins":1000},"player_code":{"source_code":"print(x)","language":"PYTHON"},"map":"[[1,0],[0,2]]"}"#;

        let expected_deserealized_struct = NormalGameRequest {
            game_id: "0fa0f12d-d472-42d5-94b4-011e0c916023".to_owned(),
            parameters: GameParameters {
                attackers: vec![
                    Attacker {
                        id: 1,
                        hp: 10,
                        range: 3,
                        attack_power: 3,
                        speed: 3,
                        price: 1,
                        is_aerial: 0,
                        weight: 1,
                        num_ability_turns: 2,
                        ability_activation_cost: 2,
                    },
                    Attacker {
                        id: 2,
                        hp: 10,
                        range: 3,
                        attack_power: 3,
                        speed: 3,
                        price: 1,
                        is_aerial: 1,
                        weight: 2,
                        num_ability_turns: 2,
                        ability_activation_cost: 3,
                    },
                ],
                defenders: vec![
                    Defender {
                        id: 1,
                        hp: 10,
                        range: 4,
                        attack_power: 5,
                        price: 1,
                        is_aerial: 1,
                    },
                    Defender {
                        id: 2,
                        hp: 10,
                        range: 6,
                        attack_power: 5,
                        price: 1,
                        is_aerial: 1,
                    },
                ],
                no_of_turns: 500,
                no_of_coins: 1000,
            },
            map: vec![vec![1, 0], vec![0, 2]],
            player_code: PlayerCode {
                language: super::Language::PYTHON,
                source_code: r#"print(x)"#.to_owned(),
            },
        };
        let deserealized_example_request: NormalGameRequest =
            serde_json::from_str(example_request_normal_game).unwrap();
        assert_eq!(deserealized_example_request, expected_deserealized_struct);

        // An example request that we might get from backend for a pvp game
        let example_request_pvp_game = r#"{"game_id":"0fa0f12d-d472-42d5-94b4-011e0c916023","parameters":{"attackers":[{"id":1,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1,"is_aerial":0,"weight":1,"num_ability_turns":2,"ability_activation_cost":2},{"id":2,"hp":10,"range":3,"attack_power":3,"speed":3,"price":1,"is_aerial":1,"weight":2,"num_ability_turns":2,"ability_activation_cost":3}],"defenders":[{"id":1,"hp":10,"range":4,"attack_power":5,"price":1,"is_aerial":1},{"id":2,"hp":10,"range":6,"attack_power":5,"price":1,"is_aerial":1}],"no_of_turns":500,"no_of_coins":10},"player1":{"source_code":"print(x)","language":"PYTHON"},"player2":{"source_code":"print(x)","language":"PYTHON"}}"#;

        let expected_deserealized_struct = PvPGameRequest {
            game_id: "0fa0f12d-d472-42d5-94b4-011e0c916023".to_owned(),
            parameters: PvPGameParameters {
                attackers: vec![
                    Attacker {
                        id: 1,
                        hp: 10,
                        range: 3,
                        attack_power: 3,
                        speed: 3,
                        price: 1,
                        is_aerial: 0,
                        weight: 1,
                        num_ability_turns: 2,
                        ability_activation_cost: 2,
                    },
                    Attacker {
                        id: 2,
                        hp: 10,
                        range: 3,
                        attack_power: 3,
                        speed: 3,
                        price: 1,
                        is_aerial: 1,
                        weight: 2,
                        num_ability_turns: 2,
                        ability_activation_cost: 3,
                    },
                ],
                defenders: vec![
                    Defender {
                        id: 1,
                        hp: 10,
                        range: 4,
                        attack_power: 5,
                        price: 1,
                        is_aerial: 1,
                    },
                    Defender {
                        id: 2,
                        hp: 10,
                        range: 6,
                        attack_power: 5,
                        price: 1,
                        is_aerial: 1,
                    },
                ],
                no_of_turns: 500,
                no_of_coins: 10,
            },
            player1: PlayerCode {
                language: super::Language::PYTHON,
                source_code: r#"print(x)"#.to_owned(),
            },
            player2: PlayerCode {
                language: super::Language::PYTHON,
                source_code: r#"print(x)"#.to_owned(),
            },
        };
        let deserealized_example_request: PvPGameRequest =
            serde_json::from_str(example_request_pvp_game).unwrap();
        assert_eq!(deserealized_example_request, expected_deserealized_struct);
    }
}
