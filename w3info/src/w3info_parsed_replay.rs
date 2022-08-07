use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct W3ParsedReplay {
    pub id: String,
    pub gamename: String,
    pub randomseed: u32,
    pub start_spots: u8,
    pub observers: Vec<String>,
    pub players: Vec<Player>,
    pub matchup: String,
    pub creator: String,
    pub r#type: String,
    pub chat: Vec<Chat>,
    pub apm: Apm,
    pub map: Map,
    pub version: String,
    pub build_number: i64,
    pub duration: i64,
    pub expansion: bool,
    pub settings: Settings,
    pub parse_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub actions: Actions,
    pub group_hotkeys: HashMap<String, GroupHotKey>,
    pub buildings: OrderSummary,
    pub items: OrderSummary,
    pub units: OrderSummary,
    pub upgrades: OrderSummary,
    pub color: String,
    pub heroes: Vec<Hero>,
    pub name: String,
    pub race: String,
    pub race_detected: String,
    pub teamid: i64,
    pub apm: i64,
    pub id: i64,
    pub resource_transfers: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Actions {
    pub timed: Vec<i64>,
    pub assigngroup: i64,
    pub rightclick: i64,
    pub basic: i64,
    pub buildtrain: i64,
    pub ability: i64,
    pub item: i64,
    pub select: i64,
    pub removeunit: i64,
    pub subgroup: i64,
    pub selecthotkey: i64,
    pub esc: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct GroupHotKey {
    pub assigned: u32,
    pub used: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct OrderSummary {
    pub summary: Vec<Summary>,
    pub order: Vec<Order>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Summary {
    pub name: String,
    pub count: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Order {
    pub id: String,
    pub ms: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Hero {
    pub name: String,
    pub level: u8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub player_name: String,
    pub player_id: u8,
    pub message: String,
    pub mode: String,
    #[serde(rename = "timeMS")]
    pub time_ms: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Apm {
    pub tracking_interval: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub path: String,
    pub file: String,
    pub checksum: String,
    pub checksum_sha1: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub referees: bool,
    pub observer_mode: String,
    pub fixed_teams: bool,
    pub full_shared_unit_control: bool,
    pub always_visible: bool,
    pub hide_terrain: bool,
    pub map_explored: bool,
    pub teams_together: bool,
    pub random_hero: bool,
    pub random_races: bool,
    pub speed: u32,
}

#[cfg(test)]
mod tests {
    use crate::w3info_parsed_replay::{
        Actions, Apm, Chat, GroupHotKey, Hero, Map, Order, OrderSummary, Player, Settings, Summary,
        W3ParsedReplay,
    };
    use std::collections::HashMap;

    #[test]
    fn parse_example() {
        let sample = serde_json::json!({
            "id":"8f1416d8ab81a5ad6726d38eb820c42f65e2e121f346d48f741e03e736cefcaa",
            "gamename":"FLO-STREAM",
            "randomseed":1164946044,
            "startSpots":2,
            "observers":["Kover00#2421"],
            "players":[
                {
                    "actions":{
                        "timed":[315,273,238,200,153,191,191,172,185,246,263,306,245,245,43],
                        "assigngroup":60,
                        "rightclick":1193,
                        "basic":174,
                        "buildtrain":102,
                        "ability":65,
                        "item":9,
                        "select":483,
                        "removeunit":0,
                        "subgroup":0,
                        "selecthotkey":1132,
                        "esc":4
                    },
                    "groupHotkeys":{
                        "0":{"assigned":0,"used":0},
                        "1":{"assigned":28,"used":293},
                        "2":{"assigned":22,"used":286},
                        "3":{"assigned":4,"used":38},
                        "4":{"assigned":2,"used":204},
                        "5":{"assigned":1,"used":219},
                        "6":{"assigned":2,"used":83},
                        "7":{"assigned":1,"used":9},
                        "8":{"assigned":0,"used":0},
                        "9":{"assigned":0,"used":0}
                    },
                    "buildings":{
                        "summary":[
                            {"name":"Altar of Elders","count":1},
                            {"name":"Moon Well","count":8},
                            {"name":"Ancient of War","count":1},
                            {"name":"Ancient of Wonders","count":2},
                            {"name":"Tree of Ages","count":1},
                            {"name":"Tree of Life","count":1},
                            {"name":"Ancient of Wind","count":2},
                            {"name":"Hunter's Hall","count":1},
                            {"name":"Tree of Eternity","count":1}
                        ],
                        "order":[
                            {"id":"eate","ms":3449},
                            {"id":"emow","ms":17035},
                            {"id":"eaom","ms":21703},
                            {"id":"emow","ms":91104},
                            {"id":"eden","ms":129294},
                            {"id":"etoa","ms":159781},
                            {"id":"emow","ms":191556},
                            {"id":"etol","ms":228262},
                            {"id":"eaow","ms":303143},
                            {"id":"eaow","ms":303542},
                            {"id":"eden","ms":363288},
                            {"id":"emow","ms":408934},
                            {"id":"emow","ms":409943},
                            {"id":"edob","ms":483994},
                            {"id":"emow","ms":495614},
                            {"id":"etoe","ms":635841},
                            {"id":"emow","ms":655758},
                            {"id":"emow","ms":657479}
                        ]
                    },
                    "items":{
                        "summary":[
                            {"name":"Moonstone","count":2},
                            {"name":"Lesser Clarity Potion","count":1},
                            {"name":"Staff of Preservation","count":1},
                            {"name":"Scroll of Healing","count":2}
                        ],
                        "order":[
                            {"id":"moon","ms":499732},
                            {"id":"plcl","ms":499762},
                            {"id":"spre","ms":502039},
                            {"id":"shea","ms":564172},
                            {"id":"moon","ms":692608},
                            {"id":"shea","ms":768471}
                        ]
                    },
                    "units":{
                        "summary":[
                            {"name":"Wisp","count":29},
                            {"name":"Archer","count":4},
                            {"name":"Hippogryph","count":3},
                            {"name":"Faerie Dragon","count":12},
                            {"name":"Goblin Sapper","count":3}
                        ],
                        "order":[
                            {"id":"ewsp","ms":1169},
                            {"id":"ewsp","ms":4218},
                            {"id":"ewsp","ms":29863},
                            {"id":"ewsp","ms":29894},
                            {"id":"ewsp","ms":29925},
                            {"id":"ewsp","ms":29955},
                            {"id":"ewsp","ms":29985},
                            {"id":"ewsp","ms":38574},
                            {"id":"ewsp","ms":45068},
                            {"id":"ewsp","ms":68268},
                            {"id":"ewsp","ms":76932},
                            {"id":"earc","ms":82006},
                            {"id":"ewsp","ms":95684},
                            {"id":"ewsp","ms":99834},
                            {"id":"ewsp","ms":99865},
                            {"id":"ewsp","ms":99896},
                            {"id":"ewsp","ms":99926},
                            {"id":"ewsp","ms":114397},
                            {"id":"ewsp","ms":143053},
                            {"id":"earc","ms":150407},
                            {"id":"earc","ms":172379},
                            {"id":"earc","ms":244497},
                            {"id":"ewsp","ms":304616},
                            {"id":"ewsp","ms":335463},
                            {"id":"ewsp","ms":335616},
                            {"id":"ewsp","ms":359473},
                            {"id":"ewsp","ms":359628},
                            {"id":"ewsp","ms":359781},
                            {"id":"ewsp","ms":359903},
                            {"id":"ehip","ms":364213},
                            {"id":"ehip","ms":365444},
                            {"id":"ewsp","ms":384426},
                            {"id":"ehip","ms":390580},
                            {"id":"efdr","ms":425852},
                            {"id":"efdr","ms":429850},
                            {"id":"efdr","ms":451282},
                            {"id":"ewsp","ms":454823},
                            {"id":"ewsp","ms":454977},
                            {"id":"efdr","ms":483284},
                            {"id":"efdr","ms":508160},
                            {"id":"efdr","ms":555598},
                            {"id":"efdr","ms":576079},
                            {"id":"efdr","ms":591996},
                            {"id":"efdr","ms":613334},
                            {"id":"ewsp","ms":630160},
                            {"id":"efdr","ms":646828},
                            {"id":"efdr","ms":705255},
                            {"id":"efdr","ms":743188},
                            {"id":"ngsp","ms":775335},
                            {"id":"ngsp","ms":775489},
                            {"id":"ngsp","ms":781495}
                        ]
                    },
                    "upgrades":{
                        "summary":[
                            {"name":"Improved Bows","count":1},
                            {"name":"Strength of the Wild","count":2},
                            {"name":"Backpack","count":1},
                            {"name":"Reinforced Hides","count":1},
                            {"name":"Ultravision","count":1}
                        ],
                        "order":[
                            {"id":"Reib","ms":320328},
                            {"id":"Resw","ms":548290},
                            {"id":"Resw","ms":614073},
                            {"id":"Repm","ms":637100},
                            {"id":"Rerh","ms":706306},
                            {"id":"Reuv","ms":755826}
                        ]
                    },
                    "color":"#0042ff",
                    "heroes":[
                        {"name":"Keeper of the Grove","level":4},
                        {"name":"Pandaren Brewmaster","level":6}
                    ],
                    "name":"DiSe22#2860",
                    "race":"N",
                    "raceDetected":"Nightelf",
                    "teamid":0,
                    "apm":230,
                    "id":1,
                    "resourceTransfers":[]
                },
                {
                    "actions":{
                        "timed":[141,121,130,141,145,79,120,82,145,150,227,187,183,106,12],
                        "assigngroup":42,
                        "rightclick":859,
                        "basic":14,
                        "buildtrain":62,
                        "ability":30,
                        "item":2,
                        "select":275,
                        "removeunit":0,
                        "subgroup":0,
                        "selecthotkey":655,
                        "esc":0
                    },
                    "groupHotkeys":{
                        "0":{"assigned":0,"used":0},
                        "1":{"assigned":29,"used":355},
                        "2":{"assigned":7,"used":183},
                        "3":{"assigned":3,"used":69},
                        "4":{"assigned":1,"used":27},
                        "5":{"assigned":2,"used":21},
                        "6":{"assigned":0,"used":0},
                        "7":{"assigned":0,"used":0},
                        "8":{"assigned":0,"used":0},
                        "9":{"assigned":0,"used":0}
                    },
                    "buildings":{
                        "summary":[
                            {"name":"Crypt","count":1},
                            {"name":"Altar of Darkness","count":1},
                            {"name":"Ziggurat","count":6},
                            {"name":"Tomb of Relics","count":1},
                            {"name":"Graveyard","count":1},
                            {"name":"Nerubian Tower","count":1},
                            {"name":"Halls of the Dead","count":1},
                            {"name":"Black Citadel","count":1},
                            {"name":"Slaughterhouse","count":1}
                        ],
                        "order":[
                            {"id":"usep","ms":6546},
                            {"id":"uaod","ms":7652},
                            {"id":"uzig","ms":33047},
                            {"id":"utom","ms":47336},
                            {"id":"ugrv","ms":101863},
                            {"id":"uzg2","ms":165009},
                            {"id":"unp1","ms":166237},
                            {"id":"uzig","ms":190486},
                            {"id":"unp2","ms":311672},
                            {"id":"uslh","ms":354069},
                            {"id":"uzig","ms":363503},
                            {"id":"uzig","ms":395817},
                            {"id":"uzig","ms":725151},
                            {"id":"uzig","ms":750598}
                        ]
                    },
                    "items":{
                        "summary":[
                            {"name":"Rod of Necromancy","count":3},
                            {"name":"Dust of Appearance","count":1},
                            {"name":"Orb of Corruption","count":1},
                            {"name":"Scroll of Healing","count":1},
                            {"name":"Potion of Mana","count":1},
                            {"name":"Scroll of Town Portal","count":2}
                        ],
                        "order":[
                            {"id":"rnec","ms":140832},
                            {"id":"dust","ms":142185},
                            {"id":"rnec","ms":318487},
                            {"id":"rnec","ms":379332},
                            {"id":"ocor","ms":467867},
                            {"id":"shea","ms":571224},
                            {"id":"pman","ms":572358},
                            {"id":"stwp","ms":641667},
                            {"id":"stwp","ms":695343}
                        ]
                    },
                    "units":{
                        "summary":[
                            {"name":"Acolyte","count":3},
                            {"name":"Ghoul","count":3},
                            {"name":"Crypt Fiend","count":7},
                            {"name":"Obsidian Statue","count":2}
                        ],
                        "order":[
                            {"id":"uaco","ms":1385},
                            {"id":"uaco","ms":17065},
                            {"id":"uaco","ms":59809},
                            {"id":"ugho","ms":68453},
                            {"id":"ugho","ms":84278},
                            {"id":"ucry","ms":226909},
                            {"id":"ucry","ms":247603},
                            {"id":"uobs","ms":421354},
                            {"id":"ucry","ms":421941},
                            {"id":"ucry","ms":495923},
                            {"id":"ucry","ms":528089},
                            {"id":"uobs","ms":537375},
                            {"id":"ugho","ms":577955},
                            {"id":"ucry","ms":687387},
                            {"id":"ucry","ms":751955}
                        ]
                    },
                    "upgrades":{
                        "summary":[
                            {"name":"Web","count":1},
                            {"name":"Destroyer Form","count":1}
                        ],
                        "order":[
                            {"id":"Ruwb","ms":441936},
                            {"id":"Rusp","ms":484332}
                        ]
                    },
                    "color":"#fffc00",
                    "heroes":[
                        {"name":"Death Knight","level":3},
                        {"name":"Lich","level":3},
                        {"name":"Naga Sea Witch","level":2}
                    ],
                    "name":"Manu#25648",
                    "race":"U",
                    "raceDetected":"Undead",
                    "teamid":1,
                    "apm":139,
                    "id":2,
                    "resourceTransfers":[]
                }
            ],
            "matchup":"NvU",
            "creator":"FLO",
            "type":"1on1",
            "chat":[
                {"playerName":"Kover00#2421","playerId":24,"message":"[FLO] Fast Forwarding...","mode":"Team","timeMS":0},
                {"playerName":"Kover00#2421","playerId":24,"message":"[FLO] Fast Forwarding complete: Synced with 4336s delay.","mode":"Team","timeMS":850190}
            ],
            "apm":{"trackingInterval":60000},
            "map":{
                "path":"Maps/W3Champions\\w3c_LastRefuge_v1.5.w3x",
                "file":"5.w3x",
                "checksum":"520a0666",
                "checksumSha1":"2292a9ef84e67b187c286e36d6a077077c849029"
            },
            "version":"32",
            "buildNumber":6114,
            "duration":852,
            "expansion":true,
            "settings":{
                "referees":false,
                "observerMode":"FULL",
                "fixedTeams":true,
                "fullSharedUnitControl":false,
                "alwaysVisible":false,
                "hideTerrain":false,
                "mapExplored":false,
                "teamsTogether":true,
                "randomHero":false,
                "randomRaces":false,
                "speed":2
            },
            "parseTime":275
        });
        let parsed = serde_json::from_value::<W3ParsedReplay>(sample);
        println!("{parsed:?}");
        assert!(parsed.is_ok());
        let replay = parsed.unwrap();
        assert_eq!(
            replay,
            W3ParsedReplay {
                id: "8f1416d8ab81a5ad6726d38eb820c42f65e2e121f346d48f741e03e736cefcaa".to_string(),
                gamename: "FLO-STREAM".to_string(),
                randomseed: 1164946044,
                start_spots: 2,
                observers: vec!["Kover00#2421".to_string()],
                players: vec![
                    Player {
                        actions: Actions {
                            timed: vec![
                                315, 273, 238, 200, 153, 191, 191, 172, 185, 246, 263, 306, 245,
                                245, 43
                            ],
                            assigngroup: 60,
                            rightclick: 1193,
                            basic: 174,
                            buildtrain: 102,
                            ability: 65,
                            item: 9,
                            select: 483,
                            removeunit: 0,
                            subgroup: 0,
                            selecthotkey: 1132,
                            esc: 4
                        },
                        group_hotkeys: HashMap::from([
                            (
                                "0".to_string(),
                                GroupHotKey {
                                    assigned: 0,
                                    used: 0
                                }
                            ),
                            (
                                "1".to_string(),
                                GroupHotKey {
                                    assigned: 28,
                                    used: 293
                                }
                            ),
                            (
                                "2".to_string(),
                                GroupHotKey {
                                    assigned: 22,
                                    used: 286
                                }
                            ),
                            (
                                "3".to_string(),
                                GroupHotKey {
                                    assigned: 4,
                                    used: 38
                                }
                            ),
                            (
                                "4".to_string(),
                                GroupHotKey {
                                    assigned: 2,
                                    used: 204
                                }
                            ),
                            (
                                "5".to_string(),
                                GroupHotKey {
                                    assigned: 1,
                                    used: 219
                                }
                            ),
                            (
                                "6".to_string(),
                                GroupHotKey {
                                    assigned: 2,
                                    used: 83
                                }
                            ),
                            (
                                "7".to_string(),
                                GroupHotKey {
                                    assigned: 1,
                                    used: 9
                                }
                            ),
                            (
                                "8".to_string(),
                                GroupHotKey {
                                    assigned: 0,
                                    used: 0
                                }
                            ),
                            (
                                "9".to_string(),
                                GroupHotKey {
                                    assigned: 0,
                                    used: 0
                                }
                            )
                        ]),
                        buildings: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Altar of Elders".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Moon Well".to_string(),
                                    count: 8
                                },
                                Summary {
                                    name: "Ancient of War".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Ancient of Wonders".to_string(),
                                    count: 2
                                },
                                Summary {
                                    name: "Tree of Ages".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Tree of Life".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Ancient of Wind".to_string(),
                                    count: 2
                                },
                                Summary {
                                    name: "Hunter's Hall".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Tree of Eternity".to_string(),
                                    count: 1
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "eate".to_string(),
                                    ms: 3449
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 17035
                                },
                                Order {
                                    id: "eaom".to_string(),
                                    ms: 21703
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 91104
                                },
                                Order {
                                    id: "eden".to_string(),
                                    ms: 129294
                                },
                                Order {
                                    id: "etoa".to_string(),
                                    ms: 159781
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 191556
                                },
                                Order {
                                    id: "etol".to_string(),
                                    ms: 228262
                                },
                                Order {
                                    id: "eaow".to_string(),
                                    ms: 303143
                                },
                                Order {
                                    id: "eaow".to_string(),
                                    ms: 303542
                                },
                                Order {
                                    id: "eden".to_string(),
                                    ms: 363288
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 408934
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 409943
                                },
                                Order {
                                    id: "edob".to_string(),
                                    ms: 483994
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 495614
                                },
                                Order {
                                    id: "etoe".to_string(),
                                    ms: 635841
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 655758
                                },
                                Order {
                                    id: "emow".to_string(),
                                    ms: 657479
                                }
                            ]
                        },
                        items: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Moonstone".to_string(),
                                    count: 2
                                },
                                Summary {
                                    name: "Lesser Clarity Potion".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Staff of Preservation".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Scroll of Healing".to_string(),
                                    count: 2
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "moon".to_string(),
                                    ms: 499732
                                },
                                Order {
                                    id: "plcl".to_string(),
                                    ms: 499762
                                },
                                Order {
                                    id: "spre".to_string(),
                                    ms: 502039
                                },
                                Order {
                                    id: "shea".to_string(),
                                    ms: 564172
                                },
                                Order {
                                    id: "moon".to_string(),
                                    ms: 692608
                                },
                                Order {
                                    id: "shea".to_string(),
                                    ms: 768471
                                }
                            ]
                        },
                        units: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Wisp".to_string(),
                                    count: 29
                                },
                                Summary {
                                    name: "Archer".to_string(),
                                    count: 4
                                },
                                Summary {
                                    name: "Hippogryph".to_string(),
                                    count: 3
                                },
                                Summary {
                                    name: "Faerie Dragon".to_string(),
                                    count: 12
                                },
                                Summary {
                                    name: "Goblin Sapper".to_string(),
                                    count: 3
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 1169
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 4218
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 29863
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 29894
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 29925
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 29955
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 29985
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 38574
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 45068
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 68268
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 76932
                                },
                                Order {
                                    id: "earc".to_string(),
                                    ms: 82006
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 95684
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 99834
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 99865
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 99896
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 99926
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 114397
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 143053
                                },
                                Order {
                                    id: "earc".to_string(),
                                    ms: 150407
                                },
                                Order {
                                    id: "earc".to_string(),
                                    ms: 172379
                                },
                                Order {
                                    id: "earc".to_string(),
                                    ms: 244497
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 304616
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 335463
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 335616
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 359473
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 359628
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 359781
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 359903
                                },
                                Order {
                                    id: "ehip".to_string(),
                                    ms: 364213
                                },
                                Order {
                                    id: "ehip".to_string(),
                                    ms: 365444
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 384426
                                },
                                Order {
                                    id: "ehip".to_string(),
                                    ms: 390580
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 425852
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 429850
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 451282
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 454823
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 454977
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 483284
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 508160
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 555598
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 576079
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 591996
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 613334
                                },
                                Order {
                                    id: "ewsp".to_string(),
                                    ms: 630160
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 646828
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 705255
                                },
                                Order {
                                    id: "efdr".to_string(),
                                    ms: 743188
                                },
                                Order {
                                    id: "ngsp".to_string(),
                                    ms: 775335
                                },
                                Order {
                                    id: "ngsp".to_string(),
                                    ms: 775489
                                },
                                Order {
                                    id: "ngsp".to_string(),
                                    ms: 781495
                                }
                            ]
                        },
                        upgrades: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Improved Bows".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Strength of the Wild".to_string(),
                                    count: 2
                                },
                                Summary {
                                    name: "Backpack".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Reinforced Hides".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Ultravision".to_string(),
                                    count: 1
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "Reib".to_string(),
                                    ms: 320328
                                },
                                Order {
                                    id: "Resw".to_string(),
                                    ms: 548290
                                },
                                Order {
                                    id: "Resw".to_string(),
                                    ms: 614073
                                },
                                Order {
                                    id: "Repm".to_string(),
                                    ms: 637100
                                },
                                Order {
                                    id: "Rerh".to_string(),
                                    ms: 706306
                                },
                                Order {
                                    id: "Reuv".to_string(),
                                    ms: 755826
                                }
                            ]
                        },
                        color: "#0042ff".to_string(),
                        heroes: vec![
                            Hero {
                                name: "Keeper of the Grove".to_string(),
                                level: 4
                            },
                            Hero {
                                name: "Pandaren Brewmaster".to_string(),
                                level: 6
                            },
                        ],
                        name: "DiSe22#2860".to_string(),
                        race: "N".to_string(),
                        race_detected: "Nightelf".to_string(),
                        teamid: 0,
                        apm: 230,
                        id: 1,
                        resource_transfers: vec![]
                    },
                    Player {
                        actions: Actions {
                            timed: vec![
                                141, 121, 130, 141, 145, 79, 120, 82, 145, 150, 227, 187, 183, 106,
                                12
                            ],
                            assigngroup: 42,
                            rightclick: 859,
                            basic: 14,
                            buildtrain: 62,
                            ability: 30,
                            item: 2,
                            select: 275,
                            removeunit: 0,
                            subgroup: 0,
                            selecthotkey: 655,
                            esc: 0
                        },
                        group_hotkeys: HashMap::from([
                            (
                                "0".to_string(),
                                GroupHotKey {
                                    assigned: 0,
                                    used: 0
                                }
                            ),
                            (
                                "1".to_string(),
                                GroupHotKey {
                                    assigned: 29,
                                    used: 355
                                }
                            ),
                            (
                                "2".to_string(),
                                GroupHotKey {
                                    assigned: 7,
                                    used: 183
                                }
                            ),
                            (
                                "3".to_string(),
                                GroupHotKey {
                                    assigned: 3,
                                    used: 69
                                }
                            ),
                            (
                                "4".to_string(),
                                GroupHotKey {
                                    assigned: 1,
                                    used: 27
                                }
                            ),
                            (
                                "5".to_string(),
                                GroupHotKey {
                                    assigned: 2,
                                    used: 21
                                }
                            ),
                            ("6".to_string(), Default::default()),
                            ("7".to_string(), Default::default()),
                            ("8".to_string(), Default::default()),
                            ("9".to_string(), Default::default()),
                        ]),
                        buildings: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Crypt".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Altar of Darkness".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Ziggurat".to_string(),
                                    count: 6
                                },
                                Summary {
                                    name: "Tomb of Relics".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Graveyard".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Nerubian Tower".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Halls of the Dead".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Black Citadel".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Slaughterhouse".to_string(),
                                    count: 1
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "usep".to_string(),
                                    ms: 6546
                                },
                                Order {
                                    id: "uaod".to_string(),
                                    ms: 7652
                                },
                                Order {
                                    id: "uzig".to_string(),
                                    ms: 33047
                                },
                                Order {
                                    id: "utom".to_string(),
                                    ms: 47336
                                },
                                Order {
                                    id: "ugrv".to_string(),
                                    ms: 101863
                                },
                                Order {
                                    id: "uzg2".to_string(),
                                    ms: 165009
                                },
                                Order {
                                    id: "unp1".to_string(),
                                    ms: 166237
                                },
                                Order {
                                    id: "uzig".to_string(),
                                    ms: 190486
                                },
                                Order {
                                    id: "unp2".to_string(),
                                    ms: 311672
                                },
                                Order {
                                    id: "uslh".to_string(),
                                    ms: 354069
                                },
                                Order {
                                    id: "uzig".to_string(),
                                    ms: 363503
                                },
                                Order {
                                    id: "uzig".to_string(),
                                    ms: 395817
                                },
                                Order {
                                    id: "uzig".to_string(),
                                    ms: 725151
                                },
                                Order {
                                    id: "uzig".to_string(),
                                    ms: 750598
                                }
                            ]
                        },
                        items: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Rod of Necromancy".to_string(),
                                    count: 3
                                },
                                Summary {
                                    name: "Dust of Appearance".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Orb of Corruption".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Scroll of Healing".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Potion of Mana".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Scroll of Town Portal".to_string(),
                                    count: 2
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "rnec".to_string(),
                                    ms: 140832
                                },
                                Order {
                                    id: "dust".to_string(),
                                    ms: 142185
                                },
                                Order {
                                    id: "rnec".to_string(),
                                    ms: 318487
                                },
                                Order {
                                    id: "rnec".to_string(),
                                    ms: 379332
                                },
                                Order {
                                    id: "ocor".to_string(),
                                    ms: 467867
                                },
                                Order {
                                    id: "shea".to_string(),
                                    ms: 571224
                                },
                                Order {
                                    id: "pman".to_string(),
                                    ms: 572358
                                },
                                Order {
                                    id: "stwp".to_string(),
                                    ms: 641667
                                },
                                Order {
                                    id: "stwp".to_string(),
                                    ms: 695343
                                }
                            ]
                        },
                        units: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Acolyte".to_string(),
                                    count: 3
                                },
                                Summary {
                                    name: "Ghoul".to_string(),
                                    count: 3
                                },
                                Summary {
                                    name: "Crypt Fiend".to_string(),
                                    count: 7
                                },
                                Summary {
                                    name: "Obsidian Statue".to_string(),
                                    count: 2
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "uaco".to_string(),
                                    ms: 1385
                                },
                                Order {
                                    id: "uaco".to_string(),
                                    ms: 17065
                                },
                                Order {
                                    id: "uaco".to_string(),
                                    ms: 59809
                                },
                                Order {
                                    id: "ugho".to_string(),
                                    ms: 68453
                                },
                                Order {
                                    id: "ugho".to_string(),
                                    ms: 84278
                                },
                                Order {
                                    id: "ucry".to_string(),
                                    ms: 226909
                                },
                                Order {
                                    id: "ucry".to_string(),
                                    ms: 247603
                                },
                                Order {
                                    id: "uobs".to_string(),
                                    ms: 421354
                                },
                                Order {
                                    id: "ucry".to_string(),
                                    ms: 421941
                                },
                                Order {
                                    id: "ucry".to_string(),
                                    ms: 495923
                                },
                                Order {
                                    id: "ucry".to_string(),
                                    ms: 528089
                                },
                                Order {
                                    id: "uobs".to_string(),
                                    ms: 537375
                                },
                                Order {
                                    id: "ugho".to_string(),
                                    ms: 577955
                                },
                                Order {
                                    id: "ucry".to_string(),
                                    ms: 687387
                                },
                                Order {
                                    id: "ucry".to_string(),
                                    ms: 751955
                                }
                            ]
                        },
                        upgrades: OrderSummary {
                            summary: vec![
                                Summary {
                                    name: "Web".to_string(),
                                    count: 1
                                },
                                Summary {
                                    name: "Destroyer Form".to_string(),
                                    count: 1
                                }
                            ],
                            order: vec![
                                Order {
                                    id: "Ruwb".to_string(),
                                    ms: 441936
                                },
                                Order {
                                    id: "Rusp".to_string(),
                                    ms: 484332
                                }
                            ]
                        },
                        color: "#fffc00".to_string(),
                        heroes: vec![
                            Hero {
                                name: "Death Knight".to_string(),
                                level: 3
                            },
                            Hero {
                                name: "Lich".to_string(),
                                level: 3
                            },
                            Hero {
                                name: "Naga Sea Witch".to_string(),
                                level: 2
                            },
                        ],
                        name: "Manu#25648".to_string(),
                        race: "U".to_string(),
                        race_detected: "Undead".to_string(),
                        teamid: 1,
                        apm: 139,
                        id: 2,
                        resource_transfers: vec![]
                    },
                ],
                matchup: "NvU".to_string(),
                creator: "FLO".to_string(),
                r#type: "1on1".to_string(),
                chat: vec![
                    Chat {
                        player_name: "Kover00#2421".to_string(),
                        player_id: 24,
                        message: "[FLO] Fast Forwarding...".to_string(),
                        mode: "Team".to_string(),
                        time_ms: 0
                    },
                    Chat {
                        player_name: "Kover00#2421".to_string(),
                        player_id: 24,
                        message: "[FLO] Fast Forwarding complete: Synced with 4336s delay."
                            .to_string(),
                        mode: "Team".to_string(),
                        time_ms: 850190
                    }
                ],
                apm: Apm {
                    tracking_interval: 60000
                },
                map: Map {
                    path: "Maps/W3Champions\\w3c_LastRefuge_v1.5.w3x".to_string(),
                    file: "5.w3x".to_string(),
                    checksum: "520a0666".to_string(),
                    checksum_sha1: "2292a9ef84e67b187c286e36d6a077077c849029".to_string()
                },
                version: "32".to_string(),
                build_number: 6114,
                duration: 852,
                expansion: true,
                settings: Settings {
                    referees: false,
                    observer_mode: "FULL".to_string(),
                    fixed_teams: true,
                    full_shared_unit_control: false,
                    always_visible: false,
                    hide_terrain: false,
                    map_explored: false,
                    teams_together: true,
                    random_hero: false,
                    random_races: false,
                    speed: 2
                },
                parse_time: 275
            }
        );
    }
}
