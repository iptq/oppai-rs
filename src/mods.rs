#![allow(missing_docs)]

use bitflags::bitflags;

bitflags! {
    /// Mod bitflags
    pub struct Mods: u32 {
        const MODS_NF = 1;
        const MODS_EZ = 2;
        const MODS_TD = 4;
        const MODS_HD = 8;
        const MODS_HR = 16;
        const MODS_SD = 32;
        const MODS_DT = 64;
        const MODS_RX = 128;
        const MODS_HT = 256;
        const MODS_NC = 512;
        const MODS_FL = 1024;
        const MODS_AT = 2048;
        const MODS_SO = 4096;
        const MODS_AP = 8192;
        const MODS_PF = 16384;
        const MODS_KEY4 = 32768;
        const MODS_KEY5 = 65536;
        const MODS_KEY6 = 131072;
        const MODS_KEY7 = 262144;
        const MODS_KEY8 = 524288;
        const MODS_FADEIN = 1048576;
        const MODS_RANDOM = 2097152;
        const MODS_CINEMA = 4194304;
        const MODS_TARGET = 8388608;
        const MODS_KEY9 = 16777216;
        const MODS_KEYCOOP = 33554432;
        const MODS_KEY1 = 67108864;
        const MODS_KEY3 = 134217728;
        const MODS_KEY2 = 268435456;
        const MODS_SCOREV2 = 536870912;
    }
}
