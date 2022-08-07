use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct W3InfoReplay {
    pub id: i64,
    pub filetype: String,
    pub uploader_id: u32,
    pub stats_event_id: Option<u32>,
    pub map: String,
    pub version: String,
    pub duration: u32,
    pub r#type: String,
    pub winner: Value,
    pub detected_winner: Value,
    pub is_highlight: u8,
    pub downloads: u32,
    pub created_at: DateTime<Utc>,
    pub replaypack_id: Value,
    pub random_hero_flag: u8,
    pub youtube: Value,
    pub youtube_time: Value,
    pub map_checksum: String,
    pub map_checksum_sha1: String,
    pub map_alias_id: Option<u32>,
    pub parsed_id: String,
    pub origin: String,
    pub likes_count: u32,
    pub comments_count: u32,
    pub notable_for: Vec<Value>,
    pub players: Vec<PlayerInfo>,
    pub likes: Vec<Value>,
    pub user_like: Value,
    pub map_alias: Option<MapAlias>,
    pub event: Option<Event>,
    pub uploader: W3InfoAccount,
    pub notable_for_stats_players: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: u32,
    pub replay_id: u32,
    pub team: u8,
    pub player: String,
    pub color: String,
    pub race: String,
    pub stats_player_id: Option<u32>,
    pub stats_player: Option<StatsPlayer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatsPlayer {
    pub id: u32,
    pub name: String,
    pub main_race: Option<String>,
    pub country: Option<String>,
    pub liquipedia: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapAlias {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub map_id: u32,
    pub map_checksum_sha1: String,
    pub map: MapInfo,
    pub map_file: Option<MapFile>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapInfo {
    pub id: i64,
    pub name: String,
    pub short: String,
    pub tags: Vec<MapTag>,
    pub image: MapImage,
    pub aliases: Vec<Alias>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapTag {
    pub id: u32,
    pub map_id: u32,
    pub mode: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapImage {
    pub file_name: String,
    pub width: u32,
    pub height: u32,
    pub thumbnails: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Alias {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub map_id: u32,
    pub map_checksum_sha1: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapFile {
    pub id: u32,
    pub original_name: String,
    pub file_name: String,
    pub file_size: u32,
    pub label: Value,
    pub created_at: String,
    pub updated_at: String,
    pub checksum_sha1: String,
    pub downloads: u32,
    pub map_alias_id: Option<u32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct W3InfoAccount {
    pub id: u32,
    pub name: String,
    pub discord: String,
    pub twitter: Value,
    pub deleted_at: Option<String>,
    pub last_visit: Option<String>,
    pub country: Option<String>,
    pub race: Option<String>,
    pub battle_net: Option<String>,
    pub twitch_stream_id: Option<u32>,
    // pub gold: f32,
    pub country_detail: Value,
    pub image: Value,
}

#[cfg(test)]
mod tests {
    use crate::w3info_replay::{
        Alias, Event, MapAlias, MapImage, MapInfo, MapTag, PlayerInfo, StatsPlayer, W3InfoAccount,
        W3InfoReplay,
    };
    use chrono::{DateTime, Utc};
    use serde_json::Value;

    fn utc(iso: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(iso)
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn parse_example() {
        let sample = serde_json::json!({
            "id":72603,
            "filetype":"w3g",
            "uploader_id":3523,
            "stats_event_id":61676,
            "map":"w3c_ConcealedHill.w3x",
            "version":"32",
            "duration":1778,
            "type":"1on1",
            "winner":null,
            "detected_winner":null,
            "is_highlight":0,
            "downloads":4,
            "created_at":"2022-08-05T21:12:26.000000Z",
            "replaypack_id":null,
            "random_hero_flag":0,
            "youtube":null,
            "youtube_time":null,
            "map_checksum":"71303ed9",
            "map_checksum_sha1":"f1bb681a2e1cafe2bb3550f48e2a5a770a80fded",
            "map_alias_id":811,
            "parsed_id":"b6db280487efd78ccdc0f73e5aec89cef8aee049006e0d43f4d0e4b85aee0320",
            "origin":"Battle.net",
            "likes_count":0,
            "comments_count":0,
            "notable_for":[],
            "players":[
                {
                    "id":162042,
                    "replay_id":72603,
                    "team":1,
                    "player":"blockkopf#2721",
                    "color":"#0042ff",
                    "race":"Human",
                    "stats_player_id":18057,
                    "stats_player":{
                        "id":18057,
                        "name":"Blockkopf",
                        "main_race":"Human",
                        "country":"de",
                        "liquipedia":null
                    }
                },
                {
                    "id":162043,
                    "replay_id":72603,
                    "team":2,
                    "player":"philo#2512",
                    "color":"#fffc00",
                    "race":"Human",
                    "stats_player_id":11902,
                    "stats_player":{
                        "id":11902,
                        "name":"Philo",
                        "main_race":"Human",
                        "country":"fr",
                        "liquipedia":"Philo"
                    }
                }
            ],
            "likes":[],
            "user_like":null,
            "map_alias":{
                "id":811,
                "created_at":"2022-07-13T15:14:09.000000Z",
                "updated_at":"2022-07-13T15:14:09.000000Z",
                "map_id":72,
                "map_checksum_sha1":"f1bb681a2e1cafe2bb3550f48e2a5a770a80fded",
                "map":{
                    "id":72,
                    "name":"Concealed Hill",
                    "short":"CH",
                    "tags":[
                        {"id":68,"map_id":72,"mode":"1vs1"}
                    ],
                    "image":{
                        "file_name":"tHDvD64xfUvxQPMmykJLkhUv3ZOHEEjerioFfYP3.jpeg",
                        "width":225,
                        "height":225,
                        "thumbnails":[]
                    },
                    "aliases":[
                        {
                            "id":4,
                            "created_at":"2020-06-24T15:30:59.000000Z",
                            "updated_at":"2020-06-24T15:30:59.000000Z",
                            "map_id":72,
                            "map_checksum_sha1":"4339540cbcaf63e958a428b9287c2439015cd0d5"
                        },
                    ]
                },
                "map_file":null
            },
            "event":{
                "id":61676,
                "name":"B2W Weekly Cup 29","link":"B2W_Weekly_Cup/29"
            },
            "uploader":{
                "id":3523,
                "name":"kover2177",
                "discord":"kover2177#6163",
                "twitter":null,
                "deleted_at":null,
                "last_visit":"2022-08-07 10:52:32",
                "country":null,
                "race":null,
                "battle_net":null,
                "twitch_stream_id":46848396,
                "gold":16.069999999999993,
                "country_detail":null,
                "image":null
            },
            "notable_for_stats_players":[]
        });
        let parsed = serde_json::from_value::<W3InfoReplay>(sample);
        assert!(parsed.is_ok());
        let replay = parsed.unwrap();
        assert_eq!(
            replay,
            W3InfoReplay {
                id: 72603,
                filetype: "w3g".to_string(),
                uploader_id: 3523,
                stats_event_id: Some(61676),
                map: "w3c_ConcealedHill.w3x".to_string(),
                version: "32".to_string(),
                duration: 1778,
                r#type: "1on1".to_string(),
                winner: Value::Null,
                detected_winner: Value::Null,
                is_highlight: 0,
                downloads: 4,
                created_at: utc("2022-08-05T21:12:26.000000Z"),
                replaypack_id: Value::Null,
                random_hero_flag: 0,
                youtube: Value::Null,
                youtube_time: Value::Null,
                map_checksum: "71303ed9".to_string(),
                map_checksum_sha1: "f1bb681a2e1cafe2bb3550f48e2a5a770a80fded".to_string(),
                map_alias_id: Some(811),
                parsed_id: "b6db280487efd78ccdc0f73e5aec89cef8aee049006e0d43f4d0e4b85aee0320"
                    .to_string(),
                origin: "Battle.net".to_string(),
                likes_count: 0,
                comments_count: 0,
                notable_for: vec![],
                players: vec![
                    PlayerInfo {
                        id: 162042,
                        replay_id: 72603,
                        team: 1,
                        player: "blockkopf#2721".to_string(),
                        color: "#0042ff".to_string(),
                        race: "Human".to_string(),
                        stats_player_id: Some(18057),
                        stats_player: Some(StatsPlayer {
                            id: 18057,
                            name: "Blockkopf".to_string(),
                            main_race: Some("Human".to_string()),
                            country: Some("de".to_string()),
                            liquipedia: None
                        })
                    },
                    PlayerInfo {
                        id: 162043,
                        replay_id: 72603,
                        team: 2,
                        player: "philo#2512".to_string(),
                        color: "#fffc00".to_string(),
                        race: "Human".to_string(),
                        stats_player_id: Some(11902),
                        stats_player: Some(StatsPlayer {
                            id: 11902,
                            name: "Philo".to_string(),
                            main_race: Some("Human".to_string()),
                            country: Some("fr".to_string()),
                            liquipedia: Some("Philo".to_string())
                        })
                    }
                ],
                likes: vec![],
                user_like: Value::Null,
                map_alias: Some(MapAlias {
                    id: 811,
                    created_at: utc("2022-07-13T15:14:09.000000Z"),
                    updated_at: utc("2022-07-13T15:14:09.000000Z"),
                    map_id: 72,
                    map_checksum_sha1: "f1bb681a2e1cafe2bb3550f48e2a5a770a80fded".to_string(),
                    map: MapInfo {
                        id: 72,
                        name: "Concealed Hill".to_string(),
                        short: "CH".to_string(),
                        tags: vec![MapTag {
                            id: 68,
                            map_id: 72,
                            mode: "1vs1".to_string()
                        }],
                        image: MapImage {
                            file_name: "tHDvD64xfUvxQPMmykJLkhUv3ZOHEEjerioFfYP3.jpeg".to_string(),
                            width: 225,
                            height: 225,
                            thumbnails: vec![]
                        },
                        aliases: vec![Alias {
                            id: 4,
                            created_at: utc("2020-06-24T15:30:59.000000Z"),
                            updated_at: utc("2020-06-24T15:30:59.000000Z"),
                            map_id: 72,
                            map_checksum_sha1: "4339540cbcaf63e958a428b9287c2439015cd0d5"
                                .to_string()
                        }]
                    },
                    map_file: None
                }),
                event: Some(Event {
                    id: 61676,
                    name: "B2W Weekly Cup 29".to_string(),
                    link: "B2W_Weekly_Cup/29".to_string()
                }),
                uploader: W3InfoAccount {
                    id: 3523,
                    name: "kover2177".to_string(),
                    discord: "kover2177#6163".to_string(),
                    twitter: Value::Null,
                    deleted_at: None,
                    last_visit: Some("2022-08-07 10:52:32".to_string()),
                    country: None,
                    race: None,
                    battle_net: None,
                    twitch_stream_id: Some(46848396),
                    // gold: 16.07,
                    country_detail: Default::default(),
                    image: Default::default()
                },
                notable_for_stats_players: vec![]
            }
        );
    }
}
