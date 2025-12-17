use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use skyline::hooks::{getRegionAddress, Region};
use skyline::install_hook;
use skyline_config::{SdCardStorage, StorageHolder};

const BUILD_INFO: [u8; 20] = [
    0x76, 0x16, 0xf8, 0x96, 0x3d, 0xac, 0xcd, 0x70, 0xe2, 0x0f, 0xf3, 0x90, 0x4e, 0x13, 0x36, 0x7f,
    0x96, 0xf2, 0xd9, 0xb3,
];

const GBL_PERSIST_SAVE_DATA_OFFSET: usize = 0x229220;
const GBL_CG_SAVE_DATA_OFFSET: usize = 0x229280;
const GBL_BGM_SAVE_DATA_OFFSET: usize = 0x229298;
const GBL_MOVIE_SAVE_DATA_OFFSET: usize = 0x2292b0;
const PERSIST_GET_OFFSET: usize = 0xd1350;
const PERSIST_SET_OFFSET: usize = 0xd12d0;
const UNLOCK_ALL_OFFSET: usize = 0xe3050;
const UNLOCK_FROM_DATA_OFFSET: usize = 0xd1930;

static CONFIG: Lazy<Config> = Lazy::new(get_config);

#[derive(Serialize, Deserialize)]
struct Config {
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
    if unsafe { !has_same_build_info() } {
        return;
    }

    if !CONFIG.is_enabled {
        return;
    }

    install_hook!(unlock_all_function);
}

unsafe fn has_same_build_info() -> bool {
    let data_adress = getRegionAddress(Region::Data) as usize;
    let scan = core::slice::from_raw_parts((data_adress - 0x1000) as *const u8, 0x1000);

    let gnu_end_pos = match scan.windows(4).position(|w| w == b"GNU\x00") {
        Some(pos) => pos + 4,
        None => return false,
    };

    let build_info = &scan[gnu_end_pos..gnu_end_pos + 20]; // In the decompilation BUILD INFO had 20 bytes

    build_info == BUILD_INFO.as_slice()
}

fn get_config() -> Config {
    let sd_storage = SdCardStorage::new(
        "atmosphere/contents/01006A300BA2C000/romfs/skyline/config/nekobako_unlock_all_mod",
    );

    let mut storage_holder = StorageHolder::new(sd_storage);

    //Creates the config if it doesn't exist
    if !storage_holder.get_flag("config.yaml") {
        let default_config = Config { is_enabled: true };

        storage_holder
            .set_field_yaml("config.yaml", &default_config)
            .unwrap();
    }

    storage_holder.get_field_yaml("config.yaml").unwrap()
}
