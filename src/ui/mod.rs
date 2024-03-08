mod component;

mod draw;
mod init;
pub(crate) mod renderloop;
mod style;

pub(crate) static mut IS_SHOW_UI: bool = true;

static mut CROP_TYPE_SELECTED: CropType = CropType::无;
static mut CROP_TYPE_LIST: Vec<CropType> = Vec::new();

static mut CROP_LEVEL_SELECTED: CropLevel = CropLevel::LV1;
static mut CROP_LEVEL_LIST: Vec<CropLevel> = Vec::new();

static mut CROP_GROWTH_STAGE_SELECTED: CropGrowthStage = CropGrowthStage::一阶段;
static mut CROP_GROWTH_STAGE_LIST: Vec<CropGrowthStage> = Vec::new();

static mut TIME_SECOND_SELECTED: u8 = 0;
static mut TIME_SECOND_LIST: Vec<u8> = Vec::new();
static mut TIME_HOUR_SELECTED: u8 = 0;
static mut TIME_HOUR_LIST: Vec<u8> = Vec::new();
static mut TIME_DAY_SELECTED: u8 = 1;
static mut TIME_DAY_LIST: Vec<u8> = Vec::new();
static mut TIME_SEASON_SELECTED: Season = Season::春;
static mut TIME_SEASON_LIST: Vec<Season> = Vec::new();
static mut TIME_YEAR_SELECTED: u8 = 1;
static mut TIME_YEAR_LIST: Vec<u8> = Vec::new();
static mut TIME_SLOW_MUL_SELECTED: TimeSlowMul = TimeSlowMul::默认;
static mut TIME_SLOW_MUL_LIST: Vec<TimeSlowMul> = Vec::new();

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum Season {
    春 = 0,
    夏 = 1,
    秋 = 2,
    冬 = 3,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum TimeSlowMul {
    暂停时间 = 0,
    百分之一 = 0x3D,
    十分之一 = 0x266,
    四分之一 = 0x600,
    二分之一 = 0xC00,
    默认 = 0x1800,
    一点五 = 0x2400,
    两点零 = 0x3000,
}

impl Into<TimeSlowMul> for u32 {
    fn into(self) -> TimeSlowMul {
        match self {
            0 => TimeSlowMul::暂停时间,
            0x3D => TimeSlowMul::百分之一,
            0x266 => TimeSlowMul::十分之一,
            0x600 => TimeSlowMul::四分之一,
            0xC00 => TimeSlowMul::二分之一,
            0x1800 => TimeSlowMul::默认,
            0x2400 => TimeSlowMul::一点五,
            0x3000 => TimeSlowMul::两点零,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(C)]
pub(crate) struct CropProp {
    pub(crate) type_: u8,
    pub(crate) stage_and_lv: u8,
}

impl CropProp {
    pub(crate) fn set_crop_type(&mut self, ct: CropType) {
        if ct as u8 == 0 {
            self.type_ = 0;
        }
        self.type_ = (ct as u8) << 1;
    }

    pub(crate) unsafe fn set_crop_growth_stage(&mut self, stage: CropGrowthStage) {
        self.stage_and_lv &= 0b0000_1111;
        self.stage_and_lv |= (stage as u8) << 4;
    }

    pub(crate) fn set_crop_level(&mut self, level: CropLevel) {
        self.stage_and_lv &= 0b0111_0000;
        self.stage_and_lv |= level as u8;
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum CropType {
    无 = 0,

    石头 = 1, // 可捡
    岩石 = 2, // 可砸
    树枝 = 3, // 可捡
    树桩 = 4, // 可劈
    木材 = 5, // 可砸，什么都不会出
    毒沼 = 6, // 可砸，什么都不会出

    // 矿石 = 7, // 锤子砸会闪退
    药草 = 8,    // 可捡
    解毒草 = 9,  // 可捡
    黑草 = 10,   // 可捡
    枯草 = 11,   // 可捡
    黄草 = 14,   // 可捡
    苦橙草 = 15, // 可捡

    // 种子 = 16, // 不可捡。名字就叫 “种子
    杂草 = 17,   // 可捡
    季节岩 = 18, // 可砸
    花卉 = 19,   // 可摧毁

    水晶 = 20, // 可砸，出的不知道是不是buff

    // 苹果 = 21, //  可砸，什么都不会出
    // 苹果 = 22    同上
    // 苹果 = 23    同上
    草莓 = 24,     // Strawberry
    卷心菜 = 25,   // Cabbage
    樱芜菁 = 26,   // Pink Turnip
    洋葱 = 27,     // Onion
    托伊药草 = 28, // Toyherb
    月落草 = 29,   // Moondrop Flower
    樱草 = 30,     // Cherry Grass
    灯草 = 31,     // Lamp Grass
    青水晶 = 33,   // Blue Crystal Flower
    金卷心菜 = 34, // Golden King Cabbage
    少女蜜瓜 = 35, // Pink Melon

    竹笋 = 36, // 可割

    南瓜 = 37,     // Pumpkin
    黄瓜 = 38,     // Cucumber
    玉米 = 39,     // Corn
    番茄 = 40,     // Tomato
    茄子 = 41,     // Eggplant
    菠萝 = 42,     // Pineapple
    粉红猫 = 43,   // Pink Cat
    铁千轮 = 44,   // Ironleaf
    四叶草 = 45,   // 4-Leaf Clover
    原之焰火 = 46, // Fireflower
    绿水晶 = 47,   // Green Crystal Flower
    金南瓜 = 48,   // Golden Pumpkin

    蓝草 = 49, // 可捡
    绿草 = 50, // 可捡
    紫草 = 51, // 可捡
    靛草 = 52, // 可捡

    红叶花 = 59,     // Autumn Grass
    剧毒蒲公英 = 60, // Pom-Pom Grass
    红水晶 = 61,     // Red Crystal Flower
    金马铃薯 = 62,   // Golden Potato
    芜菁 = 63,       // Turnip
    白萝卜 = 64,     // Radish
    葱 = 65,         // Leek
    白菜 = 66,       // Napa Cabbage
    树形草 = 67,     // Noel Grass
    白水晶 = 68,     // White Crystal Flower
    金芜青 = 69,     // Golden Turnip
    火热果实 = 70,   // Hot-Hot Fruit

    白草 = 71, // 可捡
               //无效 = 72  从72开始的编号都是无效的东西
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]

pub(crate) enum CropLevel {
    LV1 = 0,
    LV2 = 1,
    LV3 = 2,
    LV4 = 3,
    LV5 = 4,
    LV6 = 5,
    LV7 = 6,
    LV8 = 7,
    LV9 = 8,
    LV10 = 9,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    strum_macros::EnumIter,
    strum_macros::Display,
)]
pub(crate) enum CropGrowthStage {
    // 无 = 0,
    一阶段 = 0x1,
    二阶段 = 0x2,
    三阶段 = 0x3,
    四阶段 = 0x4,
    五阶段 = 0x5,
}
