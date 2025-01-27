use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    total_time: u64,
    item_level: u64,
    combat_time: u64,
    composition: Vec<Composition>,
    damage_done: Vec<Damage>,
    healing_done: Vec<HealingDone>,
    damage_taken: Vec<Damage>,
    death_events: Vec<DeathEvent>,
    log_version: u64,
    game_version: u64,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Composition {
    name: String,
    id: u64,
    guid: u64,
    #[serde(rename = "type")]
    ty: String,
    specs: Vec<Spec>
}


#[derive(Debug, Clone, Deserialize)]
pub struct Spec {
    spec: String,
    role: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Damage {
    name: String,
    guid: u64,
    #[serde(rename = "type")]
    ty: u64,
    ability_icon: String,
    total: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealingDone {
    // todo
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeathEvent {
    // todo
}

// {
//     "totalTime": 542727,
//     "itemLevel": 0,
//     "combatTime": 541425,
//     "composition": [
//         {
//             "name": "王离",
//             "id": 2,
//             "guid": 1000002,
//             "type": "Sage",
//             "specs": [
//                 {
//                     "spec": "healer",
//                     "role": "healer"
//                 }
//             ]
//         },
//         {
//             "name": "达莱尔",
//             "id": 3,
//             "guid": 1000003,
//             "type": "Astrologian",
//             "specs": [
//                 {
//                     "spec": "healer",
//                     "role": "healer"
//                 }
//             ]
//         },
//         {
//             "name": "锋面雨",
//             "id": 4,
//             "guid": 1000004,
//             "type": "Paladin",
//             "specs": [
//                 {
//                     "spec": "tank",
//                     "role": "tank"
//                 }
//             ]
//         },
//         {
//             "name": "咚哒哒",
//             "id": 5,
//             "guid": 1000005,
//             "type": "Viper",
//             "specs": [
//                 {
//                     "spec": "dps",
//                     "role": "dps"
//                 }
//             ]
//         },
//         {
//             "name": "爱摸夜螺蛳",
//             "id": 7,
//             "guid": 1000007,
//             "type": "DarkKnight",
//             "specs": [
//                 {
//                     "spec": "tank",
//                     "role": "tank"
//                 }
//             ]
//         },
//         {
//             "name": "早餐吃三明治",
//             "id": 6,
//             "guid": 1000006,
//             "type": "Machinist",
//             "specs": [
//                 {
//                     "spec": "dps",
//                     "role": "dps"
//                 }
//             ]
//         },
//         {
//             "name": "矗姦",
//             "id": 9,
//             "guid": 1000009,
//             "type": "Summoner",
//             "specs": [
//                 {
//                     "spec": "dps",
//                     "role": "dps"
//                 }
//             ]
//         },
//         {
//             "name": "人生逃避号",
//             "id": 28,
//             "guid": 1000028,
//             "type": "BlackMage",
//             "specs": [
//                 {
//                     "spec": "dps",
//                     "role": "dps"
//                 }
//             ]
//         }
//     ],
//     "damageDone": [
//         {
//             "name": "甜言蜜语",
//             "guid": 37251,
//             "type": 1024,
//             "abilityIcon": "000000-000405.png",
//             "total": 1582604
//         },
//         {
//             "name": "蜂蜂落幕曲",
//             "guid": 37263,
//             "type": 1024,
//             "abilityIcon": "000000-000405.png",
//             "total": 1682323
//         },
//         {
//             "name": "Leftward Dustdevil",
//             "guid": 37320,
//             "type": 128,
//             "abilityIcon": "000000-000405.png",
//             "total": 2191334
//         }
//     ],
//     "healingDone": [],
//     "damageTaken": [
//         {
//             "name": "山崩",
//             "guid": 25836,
//             "type": 1024,
//             "abilityIcon": "002000-002770.png",
//             "total": 509129
//         },
//         {
//             "name": "飞蛇之尾",
//             "guid": 34633,
//             "type": 128,
//             "abilityIcon": "003000-003728.png",
//             "total": 972200
//         },
//         {
//             "name": "黄宝石之仪",
//             "guid": 25824,
//             "type": 1024,
//             "abilityIcon": "002000-002761.png",
//             "total": 1102706
//         },
//         {
//             "name": "王室对撞机",
//             "guid": 25787,
//             "type": 128,
//             "abilityIcon": "003000-003047.png",
//             "total": 608437
//         },
//         {
//             "name": "暗影使者",
//             "guid": 25881,
//             "type": 1024,
//             "abilityIcon": "003000-003091.png",
//             "total": 176893
//         },
//         {
//             "name": "注药III",
//             "guid": 24312,
//             "type": 1024,
//             "abilityIcon": "003000-003680.png",
//             "total": 5263836
//         },
//         {
//             "name": "戮山",
//             "guid": 36930,
//             "type": 128,
//             "abilityIcon": "003000-003097.png",
//             "total": 482465
//         },
//         {
//             "name": "掘地飞轮",
//             "guid": 36981,
//             "type": 128,
//             "abilityIcon": "003000-003500.png",
//             "total": 782120
//         },
//         {
//             "name": "钻头",
//             "guid": 16498,
//             "type": 128,
//             "abilityIcon": "003000-003043.png",
//             "total": 1416771
//         },
//         {
//             "name": "暗影锋",
//             "guid": 17908,
//             "type": 1024,
//             "abilityIcon": "003000-003086.png",
//             "total": 133618
//         },
//         {
//             "name": "悖论",
//             "guid": 25797,
//             "type": 1024,
//             "abilityIcon": "002000-002672.png",
//             "total": 1351316
//         },
//         {
//             "name": "疾速盘蛇",
//             "guid": 34622,
//             "type": 128,
//             "abilityIcon": "003000-003717.png",
//             "total": 683800
//         },
//         {
//             "name": "灼热之闪",
//             "guid": 36991,
//             "type": 1024,
//             "abilityIcon": "002000-002781.png",
//             "total": 323058
//         },
//         {
//             "name": "热狙击弹",
//             "guid": 7413,
//             "type": 128,
//             "abilityIcon": "003000-003033.png",
//             "total": 1254149
//         },
//         {
//             "name": "箭毒II",
//             "guid": 24316,
//             "type": 1024,
//             "abilityIcon": "003000-003684.png",
//             "total": 63157
//         },
//         {
//             "name": "悔罪",
//             "guid": 16459,
//             "type": 1024,
//             "abilityIcon": "002000-002518.png",
//             "total": 655686
//         },
//         {
//             "name": "灵极脉冲",
//             "guid": 36994,
//             "type": 1024,
//             "abilityIcon": "002000-002784.png",
//             "total": 1885660
//         },
//         {
//             "name": "腐秽黑暗",
//             "guid": 25756,
//             "type": 1024,
//             "abilityIcon": "003000-003090.png",
//             "total": 175522
//         },
//         {
//             "name": "冰澈",
//             "guid": 3576,
//             "type": 1024,
//             "abilityIcon": "002000-002659.png",
//             "total": 348840
//         },
//         {
//             "name": "心神风息",
//             "guid": 37033,
//             "type": 1024,
//             "abilityIcon": "003000-003688.png",
//             "total": 406596
//         },
//         {
//             "name": "暗影使者",
//             "guid": 25757,
//             "type": 1024,
//             "abilityIcon": "003000-003091.png",
//             "total": 407589
//         },
//         {
//             "name": "耀星",
//             "guid": 36989,
//             "type": 1024,
//             "abilityIcon": "002000-002151.png",
//             "total": 872141
//         },
//         {
//             "name": "祖灵之蛇一式",
//             "guid": 34640,
//             "type": 128,
//             "abilityIcon": "003000-003735.png",
//             "total": 290757
//         },
//         {
//             "name": "落陷凶星",
//             "guid": 25871,
//             "type": 1024,
//             "abilityIcon": "003000-003559.png",
//             "total": 3898546
//         },
//         {
//             "name": "大气爆发",
//             "guid": 25854,
//             "type": 1024,
//             "abilityIcon": "000000-000405.png",
//             "total": 435482
//         },
//         {
//             "name": "绿宝石之仪",
//             "guid": 25825,
//             "type": 1024,
//             "abilityIcon": "002000-002762.png",
//             "total": 814751
//         },
//         {
//             "name": "王冠之领主",
//             "guid": 7444,
//             "type": 1024,
//             "abilityIcon": "003000-003147.png",
//             "total": 257702
//         },
//         {
//             "name": "沥血剑",
//             "guid": 3538,
//             "type": 128,
//             "abilityIcon": "002000-002506.png",
//             "total": 375548
//         },
//         {
//             "name": "爆炎",
//             "guid": 152,
//             "type": 1024,
//             "abilityIcon": "000000-000453.png",
//             "total": 448584
//         },
//         {
//             "name": "祖灵之牙三式",
//             "guid": 34629,
//             "type": 128,
//             "abilityIcon": "003000-003724.png",
//             "total": 755148
//         },
//         {
//             "name": "发炎III",
//             "guid": 24313,
//             "type": 1024,
//             "abilityIcon": "003000-003681.png",
//             "total": 752283
//         },
//         {
//             "name": "祈告剑",
//             "guid": 36918,
//             "type": 128,
//             "abilityIcon": "002000-002522.png",
//             "total": 520428
//         },
//         {
//             "name": "能量吸收",
//             "guid": 16508,
//             "type": 1024,
//             "abilityIcon": "000000-000514.png",
//             "total": 165450
//         },
//         {
//             "name": "炽炎",
//             "guid": 3577,
//             "type": 1024,
//             "abilityIcon": "002000-002660.png",
//             "total": 5447870
//         },
//         {
//             "name": "焚灼",
//             "guid": 1001881,
//             "type": 64,
//             "abilityIcon": "213000-213248.png",
//             "total": 1015583
//         },
//         {
//             "name": "打桩枪",
//             "guid": 16503,
//             "type": 128,
//             "abilityIcon": "003000-003503.png",
//             "total": 529355
//         },
//         {
//             "name": "穿裂尖齿",
//             "guid": 34607,
//             "type": 128,
//             "abilityIcon": "003000-003702.png",
//             "total": 389424
//         },
//         {
//             "name": "背裂獠齿",
//             "guid": 34613,
//             "type": 128,
//             "abilityIcon": "003000-003708.png",
//             "total": 242582
//         },
//         {
//             "name": "野火",
//             "guid": 1000861,
//             "type": 1,
//             "abilityIcon": "213000-213011.png",
//             "total": 520955
//         },
//         {
//             "name": "祖灵大蛇牙",
//             "guid": 34631,
//             "type": 128,
//             "abilityIcon": "003000-003726.png",
//             "total": 1275228
//         },
//         {
//             "name": "祖灵之牙四式",
//             "guid": 34630,
//             "type": 128,
//             "abilityIcon": "003000-003725.png",
//             "total": 680649
//         },
//         {
//             "name": "荣耀之剑",
//             "guid": 36922,
//             "type": 1024,
//             "abilityIcon": "002000-002956.png",
//             "total": 522224
//         },
//         {
//             "name": "暗影锋",
//             "guid": 16470,
//             "type": 1024,
//             "abilityIcon": "003000-003086.png",
//             "total": 705197
//         },
//         {
//             "name": "祖灵之牙一式",
//             "guid": 34627,
//             "type": 128,
//             "abilityIcon": "003000-003722.png",
//             "total": 763184
//         },
//         {
//             "name": "飞蛇连尾击",
//             "guid": 34644,
//             "type": 128,
//             "abilityIcon": "003000-003739.png",
//             "total": 255634
//         },
//         {
//             "name": "偿赎剑",
//             "guid": 25747,
//             "type": 128,
//             "abilityIcon": "002000-002951.png",
//             "total": 469434
//         },
//         {
//             "name": "星体爆炸",
//             "guid": 7441,
//             "type": 1024,
//             "abilityIcon": "003000-003143.png",
//             "total": 241217
//         },
//         {
//             "name": "地狱之火炎",
//             "guid": 25852,
//             "type": 1024,
//             "abilityIcon": "000000-000405.png",
//             "total": 383446
//         },
//         {
//             "name": "大宇宙",
//             "guid": 25874,
//             "type": 1024,
//             "abilityIcon": "003000-003562.png",
//             "total": 50366
//         },
//         {
//             "name": "厄运流转",
//             "guid": 1000248,
//             "type": 1,
//             "abilityIcon": "210000-210158.png",
//             "total": 138078
//         },
//         {
//             "name": "绝望",
//             "guid": 16505,
//             "type": 1024,
//             "abilityIcon": "002000-002665.png",
//             "total": 986932
//         },
//         {
//             "name": "烈日核爆",
//             "guid": 36996,
//             "type": 1024,
//             "abilityIcon": "002000-002786.png",
//             "total": 401555
//         },
//         {
//             "name": "高闪雷",
//             "guid": 36986,
//             "type": 1024,
//             "abilityIcon": "002000-002673.png",
//             "total": 261941
//         },
//         {
//             "name": "毁绝",
//             "guid": 7426,
//             "type": 1024,
//             "abilityIcon": "002000-002686.png",
//             "total": 379597
//         },
//         {
//             "name": "蛇尾击",
//             "guid": 34634,
//             "type": 128,
//             "abilityIcon": "003000-003729.png",
//             "total": 642257
//         },
//         {
//             "name": "毁荡",
//             "guid": 3579,
//             "type": 1024,
//             "abilityIcon": "002000-002682.png",
//             "total": 332000
//         },
//         {
//             "name": "魂灵风息",
//             "guid": 24318,
//             "type": 1024,
//             "abilityIcon": "003000-003686.png",
//             "total": 51178
//         },
//         {
//             "name": "高闪雷",
//             "guid": 1003871,
//             "type": 64,
//             "abilityIcon": "212000-212661.png",
//             "total": 1059660
//         },
//         {
//             "name": "先锋剑",
//             "guid": 9,
//             "type": 128,
//             "abilityIcon": "000000-000158.png",
//             "total": 229525
//         },
//         {
//             "name": "掠影的蔑视",
//             "guid": 36933,
//             "type": 128,
//             "abilityIcon": "000000-000405.png",
//             "total": 221603
//         },
//         {
//             "name": "投盾",
//             "guid": 24,
//             "type": 128,
//             "abilityIcon": "000000-000164.png",
//             "total": 3937
//         },
//         {
//             "name": "葬送剑",
//             "guid": 36919,
//             "type": 128,
//             "abilityIcon": "002000-002523.png",
//             "total": 594067
//         },
//         {
//             "name": "祖灵之蛇四式",
//             "guid": 34643,
//             "type": 128,
//             "abilityIcon": "003000-003738.png",
//             "total": 246905
//         },
//         {
//             "name": "滚轮冲",
//             "guid": 17206,
//             "type": 128,
//             "abilityIcon": "003000-003505.png",
//             "total": 124459
//         },
//         {
//             "name": "暴乱剑",
//             "guid": 15,
//             "type": 128,
//             "abilityIcon": "000000-000156.png",
//             "total": 313579
//         },
//         {
//             "name": "回转飞锯",
//             "guid": 25788,
//             "type": 128,
//             "abilityIcon": "003000-003048.png",
//             "total": 554969
//         },
//         {
//             "name": "冰封",
//             "guid": 154,
//             "type": 1024,
//             "abilityIcon": "000000-000456.png",
//             "total": 202424
//         },
//         {
//             "name": "血红乱",
//             "guid": 36928,
//             "type": 128,
//             "abilityIcon": "003000-003095.png",
//             "total": 308559
//         },
//         {
//             "name": "强碎灵蛇",
//             "guid": 34620,
//             "type": 128,
//             "abilityIcon": "003000-003715.png",
//             "total": 501652
//         },
//         {
//             "name": "深红强袭",
//             "guid": 25885,
//             "type": 1024,
//             "abilityIcon": "002000-002779.png",
//             "total": 341386
//         },
//         {
//             "name": "真龙波",
//             "guid": 7428,
//             "type": 1024,
//             "abilityIcon": "002000-002692.png",
//             "total": 84805
//         },
//         {
//             "name": "侧击獠齿",
//             "guid": 34610,
//             "type": 128,
//             "abilityIcon": "003000-003705.png",
//             "total": 252931
//         },
//         {
//             "name": "圣灵",
//             "guid": 7384,
//             "type": 1024,
//             "abilityIcon": "002000-002514.png",
//             "total": 622896
//         },
//         {
//             "name": "铁臂拳",
//             "guid": 16504,
//             "type": 128,
//             "abilityIcon": "003000-003504.png",
//             "total": 804232
//         },
//         {
//             "name": "猛袭利齿",
//             "guid": 34608,
//             "type": 128,
//             "abilityIcon": "003000-003703.png",
//             "total": 335976
//         },
//         {
//             "name": "噬魂斩",
//             "guid": 3632,
//             "type": 128,
//             "abilityIcon": "003000-003055.png",
//             "total": 1374584
//         },
//         {
//             "name": "赤焰",
//             "guid": 16519,
//             "type": 1024,
//             "abilityIcon": "002000-002733.png",
//             "total": 74622
//         },
//         {
//             "name": "英勇之剑",
//             "guid": 25750,
//             "type": 1024,
//             "abilityIcon": "002000-002954.png",
//             "total": 505009
//         },
//         {
//             "name": "吸收斩",
//             "guid": 3623,
//             "type": 128,
//             "abilityIcon": "003000-003054.png",
//             "total": 1152950
//         },
//         {
//             "name": "灵泉之炎",
//             "guid": 16514,
//             "type": 1024,
//             "abilityIcon": "002000-002735.png",
//             "total": 539763
//         },
//         {
//             "name": "吸血深渊",
//             "guid": 17904,
//             "type": 1024,
//             "abilityIcon": "003000-003064.png",
//             "total": 135007
//         },
//         {
//             "name": "报应",
//             "guid": 36929,
//             "type": 128,
//             "abilityIcon": "003000-003096.png",
//             "total": 347811
//         },
//         {
//             "name": "王权剑",
//             "guid": 3539,
//             "type": 128,
//             "abilityIcon": "002000-002507.png",
//             "total": 438661
//         },
//         {
//             "name": "腐秽大地",
//             "guid": 1000749,
//             "type": 64,
//             "abilityIcon": "213000-213104.png",
//             "total": 120487
//         },
//         {
//             "name": "祖灵降临",
//             "guid": 34626,
//             "type": 128,
//             "abilityIcon": "003000-003721.png",
//             "total": 840697
//         },
//         {
//             "name": "将死",
//             "guid": 36980,
//             "type": 128,
//             "abilityIcon": "003000-003508.png",
//             "total": 827462
//         },
//         {
//             "name": "信念之剑",
//             "guid": 25748,
//             "type": 1024,
//             "abilityIcon": "002000-002952.png",
//             "total": 504008
//         },
//         {
//             "name": "飞蛇乱尾击",
//             "guid": 34645,
//             "type": 128,
//             "abilityIcon": "003000-003740.png",
//             "total": 247324
//         },
//         {
//             "name": "真理之剑",
//             "guid": 25749,
//             "type": 1024,
//             "abilityIcon": "002000-002953.png",
//             "total": 489617
//         },
//         {
//             "name": "异言",
//             "guid": 16507,
//             "type": 1024,
//             "abilityIcon": "002000-002667.png",
//             "total": 2178757
//         },
//         {
//             "name": "厄运流转",
//             "guid": 23,
//             "type": 128,
//             "abilityIcon": "000000-000161.png",
//             "total": 124325
//         },
//         {
//             "name": "伤残",
//             "guid": 3624,
//             "type": 1024,
//             "abilityIcon": "003000-003062.png",
//             "total": 104691
//         },
//         {
//             "name": "绝对统治",
//             "guid": 36921,
//             "type": 1024,
//             "abilityIcon": "002000-002955.png",
//             "total": 275444
//         },
//         {
//             "name": "赎罪剑",
//             "guid": 16460,
//             "type": 128,
//             "abilityIcon": "002000-002519.png",
//             "total": 468633
//         },
//         {
//             "name": "调停",
//             "guid": 16461,
//             "type": 128,
//             "abilityIcon": "002000-002520.png",
//             "total": 125916
//         },
//         {
//             "name": "双牙连击",
//             "guid": 34636,
//             "type": 128,
//             "abilityIcon": "003000-003731.png",
//             "total": 360282
//         },
//         {
//             "name": "攻击",
//             "guid": 7,
//             "type": 128,
//             "abilityIcon": "000000-000101.png",
//             "total": 2969992
//         },
//         {
//             "name": "坏死爆发",
//             "guid": 36990,
//             "type": 1024,
//             "abilityIcon": "002000-002699.png",
//             "total": 785789
//         },
//         {
//             "name": "祖灵之蛇二式",
//             "guid": 34641,
//             "type": 128,
//             "abilityIcon": "003000-003736.png",
//             "total": 276775
//         },
//         {
//             "name": "星极脉冲",
//             "guid": 25820,
//             "type": 1024,
//             "abilityIcon": "002000-002757.png",
//             "total": 577252
//         },
//         {
//             "name": "众生离绝",
//             "guid": 36999,
//             "type": 1024,
//             "abilityIcon": "002000-002789.png",
//             "total": 582160
//         },
//         {
//             "name": "热独头弹",
//             "guid": 7412,
//             "type": 128,
//             "abilityIcon": "003000-003032.png",
//             "total": 954968
//         },
//         {
//             "name": "光芒",
//             "guid": 36993,
//             "type": 1024,
//             "abilityIcon": "002000-002783.png",
//             "total": 275988
//         },
//         {
//             "name": "祖灵之蛇三式",
//             "guid": 34642,
//             "type": 128,
//             "abilityIcon": "003000-003737.png",
//             "total": 295346
//         },
//         {
//             "name": "死星核爆",
//             "guid": 3582,
//             "type": 1024,
//             "abilityIcon": "002000-002685.png",
//             "total": 75181
//         },
//         {
//             "name": "烈焰弹",
//             "guid": 36978,
//             "type": 128,
//             "abilityIcon": "003000-003506.png",
//             "total": 1574840
//         },
//         {
//             "name": "侧裂獠齿",
//             "guid": 34611,
//             "type": 128,
//             "abilityIcon": "003000-003706.png",
//             "total": 254008
//         },
//         {
//             "name": "疾速利齿",
//             "guid": 34609,
//             "type": 128,
//             "abilityIcon": "003000-003704.png",
//             "total": 356927
//         },
//         {
//             "name": "精雕怒斩",
//             "guid": 3643,
//             "type": 128,
//             "abilityIcon": "003000-003058.png",
//             "total": 299981
//         },
//         {
//             "name": "掠影的蔑视",
//             "guid": 36932,
//             "type": 128,
//             "abilityIcon": "003000-003099.png",
//             "total": 357661
//         },
//         {
//             "name": "背击獠齿",
//             "guid": 34612,
//             "type": 128,
//             "abilityIcon": "003000-003707.png",
//             "total": 330843
//         },
//         {
//             "name": "射击",
//             "guid": 8,
//             "type": 128,
//             "abilityIcon": "../../warcraft/abilities/ability_hunter_assassinate2.jpg",
//             "total": 1023897
//         },
//         {
//             "name": "祖灵之牙二式",
//             "guid": 34628,
//             "type": 128,
//             "abilityIcon": "003000-003723.png",
//             "total": 882168
//         },
//         {
//             "name": "死亡轮回",
//             "guid": 7449,
//             "type": 1024,
//             "abilityIcon": "002000-002694.png",
//             "total": 208502
//         },
//         {
//             "name": "空气锚",
//             "guid": 16500,
//             "type": 128,
//             "abilityIcon": "003000-003045.png",
//             "total": 664637
//         },
//         {
//             "name": "红宝石之仪",
//             "guid": 25823,
//             "type": 1024,
//             "abilityIcon": "002000-002760.png",
//             "total": 761160
//         },
//         {
//             "name": "热分裂弹",
//             "guid": 7411,
//             "type": 128,
//             "abilityIcon": "003000-003031.png",
//             "total": 668759
//         },
//         {
//             "name": "均衡注药III",
//             "guid": 1002616,
//             "type": 1,
//             "abilityIcon": "212000-212962.png",
//             "total": 1170176
//         },
//         {
//             "name": "重斩",
//             "guid": 3617,
//             "type": 128,
//             "abilityIcon": "003000-003051.png",
//             "total": 867179
//         },
//         {
//             "name": "神谕",
//             "guid": 37029,
//             "type": 1024,
//             "abilityIcon": "003000-003566.png",
//             "total": 400782
//         },
//         {
//             "name": "大地之怒",
//             "guid": 25853,
//             "type": 1024,
//             "abilityIcon": "000000-000405.png",
//             "total": 540497
//         },
//         {
//             "name": "咬噬尖齿",
//             "guid": 34606,
//             "type": 128,
//             "abilityIcon": "003000-003701.png",
//             "total": 390184
//         },
//         {
//             "name": "全金属爆发",
//             "guid": 36982,
//             "type": 128,
//             "abilityIcon": "003000-003049.png",
//             "total": 700423
//         },
//         {
//             "name": "血溅",
//             "guid": 7392,
//             "type": 128,
//             "abilityIcon": "003000-003080.png",
//             "total": 906373
//         },
//         {
//             "name": "猛袭盘蛇",
//             "guid": 34621,
//             "type": 128,
//             "abilityIcon": "003000-003716.png",
//             "total": 723944
//         },
//         {
//             "name": "血溅",
//             "guid": 17909,
//             "type": 128,
//             "abilityIcon": "003000-003080.png",
//             "total": 141494
//         },
//         {
//             "name": "天启",
//             "guid": 16518,
//             "type": 1024,
//             "abilityIcon": "002000-002732.png",
//             "total": 156042
//         },
//         {
//             "name": "螺旋气流",
//             "guid": 1002706,
//             "type": 64,
//             "abilityIcon": "212000-212695.png",
//             "total": 139600
//         },
//         {
//             "name": "双将",
//             "guid": 36979,
//             "type": 128,
//             "abilityIcon": "003000-003507.png",
//             "total": 846139
//         },
//         {
//             "name": "双牙乱击",
//             "guid": 34637,
//             "type": 128,
//             "abilityIcon": "003000-003732.png",
//             "total": 402377
//         },
//         {
//             "name": "深红旋风",
//             "guid": 25835,
//             "type": 1024,
//             "abilityIcon": "002000-002769.png",
//             "total": 380118
//         },
//         {
//             "name": "螺旋气流",
//             "guid": 25837,
//             "type": 1024,
//             "abilityIcon": "002000-002771.png",
//             "total": 391260
//         }
//     ],
//     "deathEvents": [],
//     "logVersion": 66,
//     "gameVersion": 1
// }