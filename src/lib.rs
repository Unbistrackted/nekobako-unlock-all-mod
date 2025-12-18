use nekobako_plugin_utils::{get_or_generate_config, is_enabled};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use skyline::hooks::{getRegionAddress, Region};
use skyline::install_hook;
use smart_default::SmartDefault;

const GBL_PERSIST_SAVE_DATA_OFFSET: usize = 0x229220;
const GBL_CG_SAVE_DATA_OFFSET: usize = 0x229280;
const GBL_BGM_SAVE_DATA_OFFSET: usize = 0x229298;
const GBL_MOVIE_SAVE_DATA_OFFSET: usize = 0x2292b0;
const PERSIST_GET_OFFSET: usize = 0xd1350;
const PERSIST_SET_OFFSET: usize = 0xd12d0;
const UNLOCK_ALL_OFFSET: usize = 0xe3050;
const UNLOCK_FROM_DATA_OFFSET: usize = 0xd1930;

static CONFIG: Lazy<Config> =
    Lazy::new(|| get_or_generate_config::<Config>(env!("CARGO_PKG_NAME")));

#[derive(Serialize, Deserialize, SmartDefault)]
struct Config {
    #[default = true]
    is_enabled: bool,
}

#[skyline::from_offset(PERSIST_GET_OFFSET)]
unsafe fn persist_get(gbl_script: *const i64, index: u32) -> u16;

#[skyline::from_offset(PERSIST_SET_OFFSET)]
unsafe fn persist_set(gbl_script: *const i64, index: u32, value: u16);

#[skyline::from_offset(UNLOCK_FROM_DATA_OFFSET)]
unsafe fn unlock_from_data(data_adress: *const i64, id: u32) -> u64;

#[skyline::hook(offset = UNLOCK_ALL_OFFSET)]
unsafe fn unlock_all_function(param_1: u64, param_2: *mut i32, param_3: *mut i32) {
    let base_adress = getRegionAddress(Region::Text) as usize;
    let persist_offset = (base_adress + GBL_PERSIST_SAVE_DATA_OFFSET) as *const i64;
    let cg_offset = (base_adress + GBL_CG_SAVE_DATA_OFFSET) as *const i64;
    let bgm_offset = (base_adress + GBL_BGM_SAVE_DATA_OFFSET) as *const i64;
    let movie_offset = (base_adress + GBL_MOVIE_SAVE_DATA_OFFSET) as *const i64;

    if *param_2 == 1 && *param_3 == 0 {
        for i in 0..=48 {
            if persist_get(persist_offset, i) == 0 {
                persist_set(persist_offset, i, 1);
            }
        }

        //Unlocks CGs
        for i in 1116..=1223 {
            unlock_from_data(cg_offset, i);
        }
        for i in 1351..=1548 {
            unlock_from_data(cg_offset, i);
        }
        for i in 2041..=2662 {
            unlock_from_data(cg_offset, i);
        }

        //Unlocks BGMs
        for i in 9..=223 {
            unlock_from_data(bgm_offset, i);
        }

        //Unlocks Movies
        for i in 1..10 {
            unlock_from_data(movie_offset, i);
        }
    }

    call_original!(param_1, param_2, param_3);

    persist_set(persist_offset, 0, 30);
}

#[skyline::main(name = "NekobakoUnlockAllMod")]
pub fn main() {
    if is_enabled!(*CONFIG) {
        install_hook!(unlock_all_function);
    }
}
