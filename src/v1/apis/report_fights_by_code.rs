//! API: GET `/report/fights/{code}`
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub lang: String,
    pub fights: Vec<Fight>,
    pub friendlies: Vec<Friendly>,
    pub enemies: Vec<Enemy>,
    pub friendly_pets: Vec<FriendlyPet>,
    pub enemy_pets: Vec<EnemyPet>,

    pub log_version: u64,
    pub game_version: u64,
    pub phases: Vec<Phase>,
    pub title: String,
    pub owner: String,
    pub start: u64,
    pub end: u64,
    pub zone: u64,
    pub exported_characters: Vec<ExportedCharacter>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Fight {
    pub id: u64,
    pub boss: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub name: String,
    #[serde(rename = "zoneID")]
    pub zone_id: u64,
    pub zone_name: String,
    pub zone_counter: u64,
    pub size: u64,
    pub difficulty: u64,
    pub kill: bool,
    pub partial: u64,
    pub in_progress: bool,
    pub standard_composition: bool,
    pub has_echo: bool,
    #[serde(rename = "hasTrustNPCs")]
    pub has_trust_npcs: bool,
    pub combat_time: u64,
    pub boss_percentage: u64,
    pub fight_percentage: u64,
    pub last_phase_as_absolute_index: u64,
    pub last_phase_for_percentage_display: u64,
    pub maps: Vec<Map>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Map {
    #[serde(rename = "mapID")]
    pub map_id: u64,
    pub map_name: String,
    pub map_file: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Friendly {
    pub name: String,
    pub id: u64,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: String,
    pub server: String,
    pub icon: String,
    pub fights: Vec<FriendlyFight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendlyFight {
    pub id: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Enemy {
    pub name: String,
    pub id: u64,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: String,
    pub icon: String,
    pub fights: Vec<EnemyFight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnemyFight {
    pub id: u64,
    pub instances: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendlyPet {
    pub name: String,
    pub id: u64,
    pub guid: u64,
    #[serde(rename = "type")]
    pub ty: String,
    pub icon: String,
    pub pet_owner: u64,
    pub fights: Vec<FriendlyPetFight>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FriendlyPetFight {
    pub id: u64,
    pub instances: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EnemyPet {
    // todo
}

#[derive(Debug, Clone, Deserialize)]
pub struct Phase {
    // todo
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExportedCharacter {
    pub id: u64,
    pub name: String,
    pub server: String,
    pub region: String,
}

// {
//     "lang": "cn",
//     "fights": [
//         {
//             "id": 1,
//             "boss": 93,
//             "start_time": 55246,
//             "end_time": 543328,
//             "name": "黑猫",
//             "zoneID": 1226,
//             "zoneName": "Scratching Ring",
//             "zoneCounter": 2,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 487679,
//             "bossPercentage": 1,
//             "fightPercentage": 1,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 927,
//                     "mapName": "狩猎格斗场",
//                     "mapFile": "m-x6r1-x6r1.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 2,
//             "boss": 94,
//             "start_time": 704855,
//             "end_time": 1232561,
//             "name": "蜂蜂小甜心",
//             "zoneID": 1228,
//             "zoneName": "Lovely Lovering",
//             "zoneCounter": 6,
//             "size": 8,
//             "difficulty": 101,
//             "kill": false,
//             "partial": 0,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 526635,
//             "bossPercentage": 327,
//             "fightPercentage": 327,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 923,
//                     "mapName": "爱意格斗场",
//                     "mapFile": "m-x6r2-x6r2.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 3,
//             "boss": 94,
//             "start_time": 1265340,
//             "end_time": 1808067,
//             "name": "蜂蜂小甜心",
//             "zoneID": 1228,
//             "zoneName": "Lovely Lovering",
//             "zoneCounter": 6,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 541425,
//             "bossPercentage": 1,
//             "fightPercentage": 1,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 923,
//                     "mapName": "爱意格斗场",
//                     "mapFile": "m-x6r2-x6r2.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 4,
//             "boss": 95,
//             "start_time": 1945250,
//             "end_time": 2594541,
//             "name": "野蛮爆弹",
//             "zoneID": 1230,
//             "zoneName": "Blasting Ring",
//             "zoneCounter": 8,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 648933,
//             "bossPercentage": 1,
//             "fightPercentage": 1,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 926,
//                     "mapName": "爆破格斗场",
//                     "mapFile": "m-x6r3-x6r3.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 5,
//             "boss": 96,
//             "start_time": 3716276,
//             "end_time": 3792683,
//             "name": "狡雷",
//             "zoneID": 1232,
//             "zoneName": "The Thundering",
//             "zoneCounter": 11,
//             "size": 8,
//             "difficulty": 101,
//             "kill": false,
//             "partial": 0,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 75510,
//             "bossPercentage": 9040,
//             "fightPercentage": 9040,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 924,
//                     "mapName": "闪雷格斗场",
//                     "mapFile": "m-x6r4-x6r4.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 6,
//             "boss": 96,
//             "start_time": 3830323,
//             "end_time": 4603529,
//             "name": "狡雷",
//             "zoneID": 1232,
//             "zoneName": "The Thundering",
//             "zoneCounter": 11,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 772360,
//             "bossPercentage": 1,
//             "fightPercentage": 1,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 924,
//                     "mapName": "闪雷格斗场",
//                     "mapFile": "m-x6r4-x6r4.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 7,
//             "boss": 93,
//             "start_time": 18960212,
//             "end_time": 19431716,
//             "name": "黑猫",
//             "zoneID": 1226,
//             "zoneName": "Scratching Ring",
//             "zoneCounter": 31,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 470701,
//             "bossPercentage": 1,
//             "fightPercentage": 1,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 927,
//                     "mapName": "狩猎格斗场",
//                     "mapFile": "m-x6r1-x6r1.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 8,
//             "boss": 94,
//             "start_time": 19542882,
//             "end_time": 20071930,
//             "name": "蜂蜂小甜心",
//             "zoneID": 1228,
//             "zoneName": "Lovely Lovering",
//             "zoneCounter": 34,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 528107,
//             "bossPercentage": 96,
//             "fightPercentage": 96,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 923,
//                     "mapName": "爱意格斗场",
//                     "mapFile": "m-x6r2-x6r2.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 9,
//             "boss": 95,
//             "start_time": 20235886,
//             "end_time": 20852028,
//             "name": "野蛮爆弹",
//             "zoneID": 1230,
//             "zoneName": "Blasting Ring",
//             "zoneCounter": 36,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 615125,
//             "bossPercentage": 1,
//             "fightPercentage": 1,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 926,
//                     "mapName": "爆破格斗场",
//                     "mapFile": "m-x6r3-x6r3.00.jpg"
//                 }
//             ]
//         },
//         {
//             "id": 10,
//             "boss": 96,
//             "start_time": 21439963,
//             "end_time": 22181525,
//             "name": "狡雷",
//             "zoneID": 1232,
//             "zoneName": "The Thundering",
//             "zoneCounter": 38,
//             "size": 8,
//             "difficulty": 101,
//             "kill": true,
//             "partial": 2,
//             "inProgress": false,
//             "standardComposition": true,
//             "hasEcho": false,
//             "hasTrustNPCs": false,
//             "combatTime": 740493,
//             "bossPercentage": 1,
//             "fightPercentage": 1,
//             "lastPhaseAsAbsoluteIndex": 0,
//             "lastPhaseForPercentageDisplay": 0,
//             "maps": [
//                 {
//                     "mapID": 924,
//                     "mapName": "闪雷格斗场",
//                     "mapFile": "m-x6r4-x6r4.00.jpg"
//                 }
//             ]
//         }
//     ],
//     "friendlies": [
//         {
//             "name": "马里奥丶黑叔",
//             "id": 87,
//             "guid": 1000087,
//             "type": "Samurai",
//             "server": "幻影群岛",
//             "icon": "Samurai",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "冰一",
//             "id": 90,
//             "guid": 1000090,
//             "type": "Dancer",
//             "server": "幻影群岛",
//             "icon": "Dancer",
//             "fights": [
//                 {
//                     "id": 9
//                 }
//             ]
//         },
//         {
//             "name": "焱阳",
//             "id": 102,
//             "guid": 1000102,
//             "type": "Machinist",
//             "server": "宇宙和音",
//             "icon": "Machinist",
//             "fights": [
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "咚哒哒",
//             "id": 5,
//             "guid": 1000005,
//             "type": "Viper",
//             "server": "伊修加德",
//             "icon": "Viper",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 },
//                 {
//                     "id": 4
//                 }
//             ]
//         },
//         {
//             "name": "王离",
//             "id": 79,
//             "guid": 1000079,
//             "type": "Sage",
//             "server": "红玉海",
//             "icon": "Sage",
//             "fights": [
//                 {
//                     "id": 9
//                 }
//             ]
//         },
//         {
//             "name": "奥尔什方·f",
//             "id": 55,
//             "guid": 1000055,
//             "type": "Warrior",
//             "server": "水晶塔",
//             "icon": "Warrior",
//             "fights": [
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         },
//         {
//             "name": "云野絮",
//             "id": 81,
//             "guid": 1000081,
//             "type": "Bard",
//             "server": "萌芽池",
//             "icon": "Bard",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "王离",
//             "id": 2,
//             "guid": 1000002,
//             "type": "Sage",
//             "server": "太阳海岸",
//             "icon": "Sage",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 }
//             ]
//         },
//         {
//             "name": "乌拉呀哈呀哈",
//             "id": 95,
//             "guid": 1000095,
//             "type": "WhiteMage",
//             "server": "神意之地",
//             "icon": "WhiteMage",
//             "fights": [
//                 {
//                     "id": 9
//                 },
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "乌尔达哈蛋挞",
//             "id": 93,
//             "guid": 1000093,
//             "type": "Paladin",
//             "server": "神意之地",
//             "icon": "Paladin",
//             "fights": [
//                 {
//                     "id": 9
//                 },
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "Ribey",
//             "id": 53,
//             "guid": 1000053,
//             "type": "Summoner",
//             "server": "太阳海岸",
//             "icon": "Summoner",
//             "fights": [
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         },
//         {
//             "name": "王离",
//             "id": 100,
//             "guid": 1000100,
//             "type": "Scholar",
//             "server": "红玉海",
//             "icon": "Scholar",
//             "fights": [
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "王离",
//             "id": 36,
//             "guid": 1000036,
//             "type": "WhiteMage",
//             "server": "太阳海岸",
//             "icon": "WhiteMage",
//             "fights": [
//                 {
//                     "id": 4
//                 },
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         },
//         {
//             "name": "锋面雨",
//             "id": 4,
//             "guid": 1000004,
//             "type": "Paladin",
//             "server": "太阳海岸",
//             "icon": "Paladin",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 },
//                 {
//                     "id": 4
//                 }
//             ]
//         },
//         {
//             "name": "爱摸夜螺蛳",
//             "id": 7,
//             "guid": 1000007,
//             "type": "DarkKnight",
//             "server": "太阳海岸",
//             "icon": "DarkKnight",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 },
//                 {
//                     "id": 4
//                 }
//             ]
//         },
//         {
//             "name": "Cevurs",
//             "id": 92,
//             "guid": 1000092,
//             "type": "Ninja",
//             "server": "幻影群岛",
//             "icon": "Ninja",
//             "fights": [
//                 {
//                     "id": 9
//                 }
//             ]
//         },
//         {
//             "name": "西居雨生",
//             "id": 101,
//             "guid": 1000101,
//             "type": "Viper",
//             "server": "拉诺西亚",
//             "icon": "Viper",
//             "fights": [
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "肖宇",
//             "id": 54,
//             "guid": 1000054,
//             "type": "Paladin",
//             "server": "红茶川",
//             "icon": "Paladin",
//             "fights": [
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         },
//         {
//             "name": "日影镜",
//             "id": 85,
//             "guid": 1000085,
//             "type": "BlackMage",
//             "server": "萌芽池",
//             "icon": "BlackMage",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "果然是菇",
//             "id": 94,
//             "guid": 1000094,
//             "type": "DarkKnight",
//             "server": "宇宙和音",
//             "icon": "DarkKnight",
//             "fights": [
//                 {
//                     "id": 9
//                 },
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "伊克露西亚",
//             "id": 91,
//             "guid": 1000091,
//             "type": "RedMage",
//             "server": "神意之地",
//             "icon": "RedMage",
//             "fights": [
//                 {
//                     "id": 9
//                 },
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "Limit Break",
//             "id": 48,
//             "guid": 1000048,
//             "type": "LimitBreak",
//             "icon": "LimitBreak",
//             "fights": [
//                 {
//                     "id": 4
//                 },
//                 {
//                     "id": 6
//                 },
//                 {
//                     "id": 8
//                 },
//                 {
//                     "id": 9
//                 }
//             ]
//         },
//         {
//             "name": "矗姦",
//             "id": 9,
//             "guid": 1000009,
//             "type": "Summoner",
//             "server": "银泪湖",
//             "icon": "Summoner",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 },
//                 {
//                     "id": 4
//                 }
//             ]
//         },
//         {
//             "name": "矢摆的慢",
//             "id": 86,
//             "guid": 1000086,
//             "type": "Gunbreaker",
//             "server": "神意之地",
//             "icon": "Gunbreaker",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "Multiple Players",
//             "id": 10,
//             "guid": 1000010,
//             "type": "LimitBreak",
//             "icon": "LimitBreak",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 },
//                 {
//                     "id": 4
//                 },
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 },
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 },
//                 {
//                     "id": 9
//                 },
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "海德希",
//             "id": 96,
//             "guid": 1000096,
//             "type": "Pictomancer",
//             "server": "神意之地",
//             "icon": "Pictomancer",
//             "fights": [
//                 {
//                     "id": 9
//                 },
//                 {
//                     "id": 10
//                 }
//             ]
//         },
//         {
//             "name": "人生逃避号",
//             "id": 28,
//             "guid": 1000028,
//             "type": "BlackMage",
//             "server": "伊修加德",
//             "icon": "BlackMage",
//             "fights": [
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 }
//             ]
//         },
//         {
//             "name": "椰奶炒冰",
//             "id": 84,
//             "guid": 1000084,
//             "type": "Paladin",
//             "server": "神意之地",
//             "icon": "Paladin",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "冰凌可乐",
//             "id": 50,
//             "guid": 1000050,
//             "type": "Ninja",
//             "server": "水晶塔",
//             "icon": "Ninja",
//             "fights": [
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         },
//         {
//             "name": "露露提雅",
//             "id": 51,
//             "guid": 1000051,
//             "type": "Sage",
//             "server": "红茶川",
//             "icon": "Sage",
//             "fights": [
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         },
//         {
//             "name": "咕咕麦",
//             "id": 82,
//             "guid": 1000082,
//             "type": "Sage",
//             "server": "萌芽池",
//             "icon": "Sage",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "王离",
//             "id": 80,
//             "guid": 1000080,
//             "type": "WhiteMage",
//             "server": "红玉海",
//             "icon": "WhiteMage",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "早餐吃三明治",
//             "id": 6,
//             "guid": 1000006,
//             "type": "Machinist",
//             "server": "银泪湖",
//             "icon": "Machinist",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 },
//                 {
//                     "id": 4
//                 }
//             ]
//         },
//         {
//             "name": "太阳骑士高文",
//             "id": 83,
//             "guid": 1000083,
//             "type": "Viper",
//             "server": "萌芽池",
//             "icon": "Viper",
//             "fights": [
//                 {
//                     "id": 7
//                 },
//                 {
//                     "id": 8
//                 }
//             ]
//         },
//         {
//             "name": "达莱尔",
//             "id": 37,
//             "guid": 1000037,
//             "type": "Sage",
//             "server": "太阳海岸",
//             "icon": "Sage",
//             "fights": [
//                 {
//                     "id": 4
//                 }
//             ]
//         },
//         {
//             "name": "人生逃避号",
//             "id": 8,
//             "guid": 1000008,
//             "type": "Samurai",
//             "server": "伊修加德",
//             "icon": "Samurai",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 4
//                 }
//             ]
//         },
//         {
//             "name": "喵淦wk",
//             "id": 52,
//             "guid": 1000052,
//             "type": "Machinist",
//             "server": "伊修加德",
//             "icon": "Machinist",
//             "fights": [
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         },
//         {
//             "name": "达莱尔",
//             "id": 3,
//             "guid": 1000003,
//             "type": "Astrologian",
//             "server": "太阳海岸",
//             "icon": "Astrologian",
//             "fights": [
//                 {
//                     "id": 1
//                 },
//                 {
//                     "id": 2
//                 },
//                 {
//                     "id": 3
//                 }
//             ]
//         },
//         {
//             "name": "莉莉娅没睡醒",
//             "id": 49,
//             "guid": 1000049,
//             "type": "BlackMage",
//             "server": "水晶塔",
//             "icon": "BlackMage",
//             "fights": [
//                 {
//                     "id": 5
//                 },
//                 {
//                     "id": 6
//                 }
//             ]
//         }
//     ],
//     "enemies": [
//         {
//             "name": "Multiple Enemies",
//             "id": 47,
//             "guid": 1000047,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 4,
//                     "instances": 1
//                 },
//                 {
//                     "id": 9,
//                     "instances": 1
//                 }
//             ]
//         },
//         {
//             "name": "Wicked Thunder",
//             "id": 56,
//             "guid": 17322,
//             "type": "Boss",
//             "icon": "Boss",
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 1
//                 },
//                 {
//                     "id": 6,
//                     "instances": 1
//                 },
//                 {
//                     "id": 10,
//                     "instances": 1
//                 }
//             ]
//         },
//         {
//             "name": "Copy Cat",
//             "id": 25,
//             "guid": 17194,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 1,
//                     "groups": 2
//                 },
//                 {
//                     "id": 7,
//                     "instances": 1,
//                     "groups": 2
//                 }
//             ]
//         },
//         {
//             "name": "Black Cat",
//             "id": 16,
//             "guid": 2000016,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 9,
//                     "groups": 14
//                 },
//                 {
//                     "id": 7,
//                     "instances": 9,
//                     "groups": 13
//                 }
//             ]
//         },
//         {
//             "name": "Multiple Enemies",
//             "id": 89,
//             "guid": 1000089,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 7,
//                     "instances": 1
//                 }
//             ]
//         },
//         {
//             "name": "Soulshade",
//             "id": 23,
//             "guid": 2000023,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 8,
//                     "groups": 4
//                 },
//                 {
//                     "id": 7,
//                     "instances": 8,
//                     "groups": 4
//                 }
//             ]
//         },
//         {
//             "name": "Brute Distortion",
//             "id": 43,
//             "guid": 2000043,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 4,
//                     "instances": 8,
//                     "groups": 2
//                 },
//                 {
//                     "id": 9,
//                     "instances": 8,
//                     "groups": 2
//                 }
//             ]
//         },
//         {
//             "name": "Copy Cat",
//             "id": 26,
//             "guid": 2000026,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 2,
//                     "groups": 5
//                 },
//                 {
//                     "id": 7,
//                     "instances": 2,
//                     "groups": 6
//                 }
//             ]
//         },
//         {
//             "name": "Honey B. Lovely",
//             "id": 29,
//             "guid": 16941,
//             "type": "Boss",
//             "icon": "Boss",
//             "fights": [
//                 {
//                     "id": 2,
//                     "instances": 1
//                 },
//                 {
//                     "id": 3,
//                     "instances": 1
//                 },
//                 {
//                     "id": 8,
//                     "instances": 1
//                 }
//             ]
//         },
//         {
//             "name": "Honey B. Lovely",
//             "id": 30,
//             "guid": 2000030,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 2,
//                     "instances": 13,
//                     "groups": 12
//                 },
//                 {
//                     "id": 3,
//                     "instances": 13,
//                     "groups": 12
//                 },
//                 {
//                     "id": 8,
//                     "instances": 15,
//                     "groups": 12
//                 }
//             ]
//         },
//         {
//             "name": "Brute Bomber",
//             "id": 38,
//             "guid": 17094,
//             "type": "Boss",
//             "icon": "Boss",
//             "fights": [
//                 {
//                     "id": 4,
//                     "instances": 1
//                 },
//                 {
//                     "id": 9,
//                     "instances": 1
//                 }
//             ]
//         },
//         {
//             "name": "Wicked Thunder",
//             "id": 62,
//             "guid": 2000062,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 6
//                 },
//                 {
//                     "id": 6,
//                     "instances": 18
//                 },
//                 {
//                     "id": 10,
//                     "instances": 18
//                 }
//             ]
//         },
//         {
//             "name": "Black Cat",
//             "id": 12,
//             "guid": 17193,
//             "type": "Boss",
//             "icon": "Boss",
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 1
//                 },
//                 {
//                     "id": 7,
//                     "instances": 1
//                 }
//             ]
//         },
//         {
//             "name": "Brute Bomber",
//             "id": 39,
//             "guid": 2000039,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 4,
//                     "instances": 12,
//                     "groups": 10
//                 },
//                 {
//                     "id": 9,
//                     "instances": 12,
//                     "groups": 9
//                 }
//             ]
//         },
//         {
//             "name": "Groupbee",
//             "id": 34,
//             "guid": 2000034,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 3,
//                     "instances": 12,
//                     "groups": 2
//                 },
//                 {
//                     "id": 8,
//                     "instances": 12,
//                     "groups": 3
//                 }
//             ]
//         },
//         {
//             "name": "Multiple Enemies",
//             "id": 32,
//             "guid": 1000032,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 2,
//                     "instances": 1,
//                     "groups": 4
//                 },
//                 {
//                     "id": 3,
//                     "instances": 1,
//                     "groups": 4
//                 },
//                 {
//                     "id": 8,
//                     "instances": 1,
//                     "groups": 4
//                 }
//             ]
//         },
//         {
//             "name": "Multiple Enemies",
//             "id": 70,
//             "guid": 1000070,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 6,
//                     "instances": 1,
//                     "groups": 2
//                 }
//             ]
//         },
//         {
//             "name": "_rsv_13061_-1_1_0_0_S64755250_E64755250",
//             "id": 73,
//             "guid": 17327,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 6,
//                     "instances": 8,
//                     "groups": 1
//                 },
//                 {
//                     "id": 10,
//                     "instances": 8,
//                     "groups": 1
//                 }
//             ]
//         },
//         {
//             "name": "Sinister Spark",
//             "id": 45,
//             "guid": 17098,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 4,
//                     "instances": 8,
//                     "groups": 1
//                 },
//                 {
//                     "id": 9,
//                     "instances": 8,
//                     "groups": 1
//                 }
//             ]
//         },
//         {
//             "name": "_rsv_13059_-1_1_0_0_S64755250_E64755250",
//             "id": 64,
//             "guid": 17324,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 9,
//                     "groups": 1
//                 }
//             ]
//         },
//         {
//             "name": "Sweetheart",
//             "id": 35,
//             "guid": 16943,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 3,
//                     "instances": 17,
//                     "groups": 3
//                 }
//             ]
//         },
//         {
//             "name": "Wicked Replica",
//             "id": 66,
//             "guid": 17323,
//             "type": "NPC",
//             "icon": "NPC",
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 4,
//                     "groups": 1
//                 },
//                 {
//                     "id": 6,
//                     "instances": 8,
//                     "groups": 5
//                 },
//                 {
//                     "id": 10,
//                     "instances": 8,
//                     "groups": 4
//                 }
//             ]
//         },
//         {
//             "name": "Wicked Thunder",
//             "id": 72,
//             "guid": 17326,
//             "type": "Boss",
//             "icon": "Boss",
//             "fights": [
//                 {
//                     "id": 6,
//                     "instances": 1
//                 },
//                 {
//                     "id": 10,
//                     "instances": 1
//                 }
//             ]
//         }
//     ],
//     "friendlyPets": [
//         {
//             "name": "liturgic bell",
//             "id": 88,
//             "guid": 13961,
//             "type": "Pet",
//             "icon": "abilities/002000-002649.png",
//             "petOwner": 80,
//             "fights": [
//                 {
//                     "id": 7,
//                     "instances": 2
//                 },
//                 {
//                     "id": 8,
//                     "instances": 2
//                 }
//             ]
//         },
//         {
//             "name": "Topaz Titan",
//             "id": 18,
//             "guid": 13507,
//             "type": "Pet",
//             "icon": "abilities/002000-002773.png",
//             "petOwner": 9,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 8
//                 },
//                 {
//                     "id": 2,
//                     "instances": 9
//                 },
//                 {
//                     "id": 3,
//                     "instances": 9
//                 },
//                 {
//                     "id": 4,
//                     "instances": 11
//                 }
//             ]
//         },
//         {
//             "name": "Emerald Garuda",
//             "id": 19,
//             "guid": 13506,
//             "type": "Pet",
//             "icon": "abilities/002000-002774.png",
//             "petOwner": 9,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 8
//                 },
//                 {
//                     "id": 2,
//                     "instances": 8
//                 },
//                 {
//                     "id": 3,
//                     "instances": 9
//                 },
//                 {
//                     "id": 4,
//                     "instances": 11
//                 }
//             ]
//         },
//         {
//             "name": "Carbuncle",
//             "id": 57,
//             "guid": 13498,
//             "type": "Pet",
//             "icon": "abilities/000000-000516.png",
//             "petOwner": 53,
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 3
//                 },
//                 {
//                     "id": 6,
//                     "instances": 26
//                 }
//             ]
//         },
//         {
//             "name": "Solar Bahamut",
//             "id": 13,
//             "guid": 16926,
//             "type": "Pet",
//             "icon": "Pet",
//             "petOwner": 9,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 4
//                 },
//                 {
//                     "id": 2,
//                     "instances": 5
//                 },
//                 {
//                     "id": 3,
//                     "instances": 5
//                 },
//                 {
//                     "id": 4,
//                     "instances": 6
//                 }
//             ]
//         },
//         {
//             "name": "Emerald Garuda",
//             "id": 65,
//             "guid": 13506,
//             "type": "Pet",
//             "icon": "abilities/002000-002774.png",
//             "petOwner": 53,
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 1
//                 },
//                 {
//                     "id": 6,
//                     "instances": 12
//                 }
//             ]
//         },
//         {
//             "name": "Demi-Bahamut",
//             "id": 67,
//             "guid": 6982,
//             "type": "Pet",
//             "icon": "abilities/002000-002691.png",
//             "petOwner": 53,
//             "fights": [
//                 {
//                     "id": 6,
//                     "instances": 4
//                 }
//             ]
//         },
//         {
//             "name": "Topaz Titan",
//             "id": 61,
//             "guid": 13507,
//             "type": "Pet",
//             "icon": "abilities/002000-002773.png",
//             "petOwner": 53,
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 1
//                 },
//                 {
//                     "id": 6,
//                     "instances": 12
//                 }
//             ]
//         },
//         {
//             "name": "Esteem",
//             "id": 15,
//             "guid": 10489,
//             "type": "Pet",
//             "icon": "abilities/003000-003088.png",
//             "petOwner": 7,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 4
//                 },
//                 {
//                     "id": 2,
//                     "instances": 5
//                 },
//                 {
//                     "id": 3,
//                     "instances": 5
//                 },
//                 {
//                     "id": 4,
//                     "instances": 6
//                 }
//             ]
//         },
//         {
//             "name": "Demi-Phoenix",
//             "id": 71,
//             "guid": 10488,
//             "type": "Pet",
//             "icon": "abilities/002000-002765.png",
//             "petOwner": 53,
//             "fights": [
//                 {
//                     "id": 6,
//                     "instances": 2
//                 }
//             ]
//         },
//         {
//             "name": "Seraph",
//             "id": 105,
//             "guid": 10487,
//             "type": "Pet",
//             "icon": "abilities/002000-002850.png",
//             "petOwner": 100,
//             "fights": [
//                 {
//                     "id": 10,
//                     "instances": 5
//                 }
//             ]
//         },
//         {
//             "name": "Automaton Queen",
//             "id": 60,
//             "guid": 10490,
//             "type": "Pet",
//             "icon": "abilities/003000-003501.png",
//             "petOwner": 52,
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 2
//                 },
//                 {
//                     "id": 6,
//                     "instances": 23
//                 }
//             ]
//         },
//         {
//             "name": "bunshin",
//             "id": 98,
//             "guid": 10897,
//             "type": "Pet",
//             "icon": "abilities/002000-002927.png",
//             "petOwner": 92,
//             "fights": [
//                 {
//                     "id": 9,
//                     "instances": 7
//                 }
//             ]
//         },
//         {
//             "name": "Demi-Phoenix",
//             "id": 27,
//             "guid": 10488,
//             "type": "Pet",
//             "icon": "abilities/002000-002765.png",
//             "petOwner": 9,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 2
//                 },
//                 {
//                     "id": 2,
//                     "instances": 2
//                 },
//                 {
//                     "id": 3,
//                     "instances": 2
//                 },
//                 {
//                     "id": 4,
//                     "instances": 1
//                 }
//             ]
//         },
//         {
//             "name": "Automaton Queen",
//             "id": 17,
//             "guid": 10490,
//             "type": "Pet",
//             "icon": "abilities/003000-003501.png",
//             "petOwner": 6,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 12
//                 },
//                 {
//                     "id": 2,
//                     "instances": 13
//                 },
//                 {
//                     "id": 3,
//                     "instances": 14
//                 },
//                 {
//                     "id": 4,
//                     "instances": 18
//                 }
//             ]
//         },
//         {
//             "name": "Solar Bahamut",
//             "id": 58,
//             "guid": 16926,
//             "type": "Pet",
//             "icon": "Pet",
//             "petOwner": 53,
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 1
//                 },
//                 {
//                     "id": 6,
//                     "instances": 7
//                 }
//             ]
//         },
//         {
//             "name": "Earthly Star",
//             "id": 14,
//             "guid": 7245,
//             "type": "Pet",
//             "icon": "abilities/003000-003143.png",
//             "petOwner": 3,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 8
//                 },
//                 {
//                     "id": 2,
//                     "instances": 9
//                 },
//                 {
//                     "id": 3,
//                     "instances": 9
//                 }
//             ]
//         },
//         {
//             "name": "Eos",
//             "id": 104,
//             "guid": 1008,
//             "type": "Pet",
//             "icon": "abilities/002000-002823.png",
//             "petOwner": 100,
//             "fights": [
//                 {
//                     "id": 10,
//                     "instances": 9
//                 }
//             ]
//         },
//         {
//             "name": "Automaton Queen",
//             "id": 103,
//             "guid": 10490,
//             "type": "Pet",
//             "icon": "abilities/003000-003501.png",
//             "petOwner": 102,
//             "fights": [
//                 {
//                     "id": 10,
//                     "instances": 13
//                 }
//             ]
//         },
//         {
//             "name": "Topaz Carbuncle",
//             "id": 11,
//             "guid": 13498,
//             "type": "Pet",
//             "icon": "abilities/000000-000516.png",
//             "petOwner": 9,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 16
//                 },
//                 {
//                     "id": 2,
//                     "instances": 18
//                 },
//                 {
//                     "id": 3,
//                     "instances": 19
//                 },
//                 {
//                     "id": 4,
//                     "instances": 18
//                 }
//             ]
//         },
//         {
//             "name": "Demi-Bahamut",
//             "id": 22,
//             "guid": 6982,
//             "type": "Pet",
//             "icon": "abilities/002000-002691.png",
//             "petOwner": 9,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 2
//                 },
//                 {
//                     "id": 2,
//                     "instances": 2
//                 },
//                 {
//                     "id": 3,
//                     "instances": 2
//                 },
//                 {
//                     "id": 4,
//                     "instances": 4
//                 }
//             ]
//         },
//         {
//             "name": "Ruby Ifrit",
//             "id": 63,
//             "guid": 13505,
//             "type": "Pet",
//             "icon": "abilities/002000-002772.png",
//             "petOwner": 53,
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 1
//                 },
//                 {
//                     "id": 6,
//                     "instances": 12
//                 }
//             ]
//         },
//         {
//             "name": "Ruby Ifrit",
//             "id": 20,
//             "guid": 13505,
//             "type": "Pet",
//             "icon": "abilities/002000-002772.png",
//             "petOwner": 9,
//             "fights": [
//                 {
//                     "id": 1,
//                     "instances": 8
//                 },
//                 {
//                     "id": 2,
//                     "instances": 8
//                 },
//                 {
//                     "id": 3,
//                     "instances": 8
//                 },
//                 {
//                     "id": 4,
//                     "instances": 10
//                 }
//             ]
//         },
//         {
//             "name": "Esteem",
//             "id": 97,
//             "guid": 10489,
//             "type": "Pet",
//             "icon": "abilities/003000-003088.png",
//             "petOwner": 94,
//             "fights": [
//                 {
//                     "id": 9,
//                     "instances": 6
//                 },
//                 {
//                     "id": 10,
//                     "instances": 6
//                 }
//             ]
//         },
//         {
//             "name": "bunshin",
//             "id": 59,
//             "guid": 10897,
//             "type": "Pet",
//             "icon": "abilities/002000-002927.png",
//             "petOwner": 50,
//             "fights": [
//                 {
//                     "id": 5,
//                     "instances": 1
//                 },
//                 {
//                     "id": 6,
//                     "instances": 9
//                 }
//             ]
//         },
//         {
//             "name": "liturgic bell",
//             "id": 99,
//             "guid": 13961,
//             "type": "Pet",
//             "icon": "abilities/002000-002649.png",
//             "petOwner": 95,
//             "fights": [
//                 {
//                     "id": 9,
//                     "instances": 2
//                 },
//                 {
//                     "id": 10,
//                     "instances": 2
//                 }
//             ]
//         },
//         {
//             "name": "liturgic bell",
//             "id": 40,
//             "guid": 13961,
//             "type": "Pet",
//             "icon": "abilities/002000-002649.png",
//             "petOwner": 36,
//             "fights": [
//                 {
//                     "id": 4,
//                     "instances": 3
//                 },
//                 {
//                     "id": 6,
//                     "instances": 3
//                 }
//             ]
//         }
//     ],
//     "enemyPets": [],
//     "logVersion": 66,
//     "gameVersion": 1,
//     "phases": [],
//     "title": "AAC Light-Heavyweight",
//     "owner": "ITX351",
//     "start": 1736239588053,
//     "end": 1736261769578,
//     "zone": 62,
//     "exportedCharacters": [
//         {
//             "id": 13580413,
//             "name": "Cevurs",
//             "server": "幻影群岛",
//             "region": "CN"
//         },
//         {
//             "id": 18338331,
//             "name": "Ribey",
//             "server": "太阳海岸",
//             "region": "CN"
//         },
//         {
//             "id": 20346857,
//             "name": "乌尔达哈蛋挞",
//             "server": "神意之地",
//             "region": "CN"
//         },
//         {
//             "id": 19813355,
//             "name": "乌拉呀哈呀哈",
//             "server": "神意之地",
//             "region": "CN"
//         },
//         {
//             "id": 18191960,
//             "name": "云野絮",
//             "server": "萌芽池",
//             "region": "CN"
//         },
//         {
//             "id": 19803676,
//             "name": "人生逃避号",
//             "server": "伊修加德",
//             "region": "CN"
//         },
//         {
//             "id": 16044704,
//             "name": "伊克露西亚",
//             "server": "神意之地",
//             "region": "CN"
//         },
//         {
//             "id": 13743099,
//             "name": "冰一",
//             "server": "幻影群岛",
//             "region": "CN"
//         },
//         {
//             "id": 20510990,
//             "name": "冰凌可乐",
//             "server": "水晶塔",
//             "region": "CN"
//         },
//         {
//             "id": 16643985,
//             "name": "咕咕麦",
//             "server": "萌芽池",
//             "region": "CN"
//         },
//         {
//             "id": 18657934,
//             "name": "咚哒哒",
//             "server": "伊修加德",
//             "region": "CN"
//         },
//         {
//             "id": 19344575,
//             "name": "喵淦wk",
//             "server": "伊修加德",
//             "region": "CN"
//         },
//         {
//             "id": 12592059,
//             "name": "太阳骑士高文",
//             "server": "萌芽池",
//             "region": "CN"
//         },
//         {
//             "id": 19273562,
//             "name": "奥尔什方·f",
//             "server": "水晶塔",
//             "region": "CN"
//         },
//         {
//             "id": 13215229,
//             "name": "日影镜",
//             "server": "萌芽池",
//             "region": "CN"
//         },
//         {
//             "id": 19303317,
//             "name": "早餐吃三明治",
//             "server": "银泪湖",
//             "region": "CN"
//         },
//         {
//             "id": 20044866,
//             "name": "果然是菇",
//             "server": "宇宙和音",
//             "region": "CN"
//         },
//         {
//             "id": 19235120,
//             "name": "椰奶炒冰",
//             "server": "神意之地",
//             "region": "CN"
//         },
//         {
//             "id": 12001198,
//             "name": "海德希",
//             "server": "神意之地",
//             "region": "CN"
//         },
//         {
//             "id": 19147380,
//             "name": "焱阳",
//             "server": "宇宙和音",
//             "region": "CN"
//         },
//         {
//             "id": 19034306,
//             "name": "爱摸夜螺蛳",
//             "server": "太阳海岸",
//             "region": "CN"
//         },
//         {
//             "id": 19810229,
//             "name": "王离",
//             "server": "红玉海",
//             "region": "CN"
//         },
//         {
//             "id": 20462431,
//             "name": "王离",
//             "server": "太阳海岸",
//             "region": "CN"
//         },
//         {
//             "id": 19613937,
//             "name": "矗姦",
//             "server": "银泪湖",
//             "region": "CN"
//         },
//         {
//             "id": 20523386,
//             "name": "矢摆的慢",
//             "server": "神意之地",
//             "region": "CN"
//         },
//         {
//             "id": 20582381,
//             "name": "肖宇",
//             "server": "红茶川",
//             "region": "CN"
//         },
//         {
//             "id": 19259472,
//             "name": "莉莉娅没睡醒",
//             "server": "水晶塔",
//             "region": "CN"
//         },
//         {
//             "id": 16191335,
//             "name": "西居雨生",
//             "server": "拉诺西亚",
//             "region": "CN"
//         },
//         {
//             "id": 20197154,
//             "name": "达莱尔",
//             "server": "太阳海岸",
//             "region": "CN"
//         },
//         {
//             "id": 19864483,
//             "name": "锋面雨",
//             "server": "太阳海岸",
//             "region": "CN"
//         },
//         {
//             "id": 19078787,
//             "name": "露露提雅",
//             "server": "红茶川",
//             "region": "CN"
//         },
//         {
//             "id": 18676674,
//             "name": "马里奥丶黑叔",
//             "server": "幻影群岛",
//             "region": "CN"
//         }
//     ]
// }
