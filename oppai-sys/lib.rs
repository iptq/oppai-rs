#![allow(non_camel_case_types)]

// Modes
pub const MODE_STD: u32 = 0;
pub const MODE_TAIKO: u32 = 1;

// Mods
pub const MODS_NOMOD: u32 = 0;
pub const MODS_NF: u32 = 1;
pub const MODS_EZ: u32 = 2;
pub const MODS_TD: u32 = 4;
pub const MODS_HD: u32 = 8;
pub const MODS_HR: u32 = 16;
pub const MODS_SD: u32 = 32;
pub const MODS_DT: u32 = 64;
pub const MODS_RX: u32 = 128;
pub const MODS_HT: u32 = 256;
pub const MODS_NC: u32 = 512;
pub const MODS_FL: u32 = 1024;
pub const MODS_AT: u32 = 2048;
pub const MODS_SO: u32 = 4096;
pub const MODS_AP: u32 = 8192;
pub const MODS_PF: u32 = 16384;
pub const MODS_KEY4: u32 = 32768;
pub const MODS_KEY5: u32 = 65536;
pub const MODS_KEY6: u32 = 131072;
pub const MODS_KEY7: u32 = 262144;
pub const MODS_KEY8: u32 = 524288;
pub const MODS_FADEIN: u32 = 1048576;
pub const MODS_RANDOM: u32 = 2097152;
pub const MODS_CINEMA: u32 = 4194304;
pub const MODS_TARGET: u32 = 8388608;
pub const MODS_KEY9: u32 = 16777216;
pub const MODS_KEYCOOP: u32 = 33554432;
pub const MODS_KEY1: u32 = 67108864;
pub const MODS_KEY3: u32 = 134217728;
pub const MODS_KEY2: u32 = 268435456;
pub const MODS_SCOREV2: u32 = 536870912;
pub const MODS_TOUCH_DEVICE: u32 = 4;
pub const MODS_NOVIDEO: u32 = 4;
pub const MODS_SPEED_CHANGING: u32 = 832;
pub const MODS_MAP_CHANGING: u32 = 850;

pub const ERR_MORE: i32 = -1;
pub const ERR_SYNTAX: i32 = -2;
pub const ERR_TRUNCATED: i32 = -3;
pub const ERR_NOTIMPLEMENTED: i32 = -4;
pub const ERR_IO: i32 = -5;
pub const ERR_FORMAT: i32 = -6;
pub const ERR_OOM: i32 = -7;
pub const DIFF_SPEED: u32 = 0;
pub const DIFF_AIM: u32 = 1;
pub const OPPAI_VERSION_MAJOR: u32 = 3;
pub const OPPAI_VERSION_MINOR: u32 = 2;
pub const OPPAI_VERSION_PATCH: u32 = 3;
pub const OBJ_CIRCLE: u32 = 1;
pub const OBJ_SLIDER: u32 = 2;
pub const OBJ_SPINNER: u32 = 8;
pub const SOUND_NONE: u32 = 0;
pub const SOUND_NORMAL: u32 = 1;
pub const SOUND_WHISTLE: u32 = 2;
pub const SOUND_FINISH: u32 = 4;
pub const SOUND_CLAP: u32 = 8;
pub const AUTOCALC_BIT: u32 = 1;
pub const OWNS_MAP_BIT: u32 = 2;
pub const M_BLOCK_SIZE: u32 = 4096;
pub const AR0_MS: f64 = 1800.0;
pub const AR5_MS: f64 = 1200.0;
pub const AR10_MS: f64 = 450.0;
pub const AR_MS_STEP1: f64 = 120.0;
pub const AR_MS_STEP2: f64 = 150.0;
pub const P_OVERRIDE_MODE: u32 = 1;
pub const P_FOUND_AR: u32 = 2;
pub const CIRCLESIZE_BUFF_TRESHOLD: f64 = 30.0;
pub const PLAYFIELD_WIDTH: f64 = 512.0;
pub const PLAYFIELD_HEIGHT: f64 = 384.0;
pub const SINGLE_SPACING: f64 = 125.0;
pub const STAR_SCALING_FACTOR: f64 = 0.0675;
pub const EXTREME_SCALING_FACTOR: f64 = 0.5;
pub const STRAIN_STEP: f64 = 400.0;
pub const DECAY_WEIGHT: f64 = 0.9;
pub const MAX_SPEED_BONUS: f64 = 45.0;
pub const MIN_SPEED_BONUS: f64 = 75.0;
pub const ANGLE_BONUS_SCALE: u32 = 90;
pub const AIM_TIMING_THRESHOLD: u32 = 107;
pub const SPEED_ANGLE_BONUS_BEGIN: f64 = 2.6179938779914944;
pub const AIM_ANGLE_BONUS_BEGIN: f64 = 1.0471975511965976;
pub const TAIKO_STAR_SCALING_FACTOR: f64 = 0.04125;
pub const TAIKO_TYPE_CHANGE_BONUS: f64 = 0.75;
pub const TAIKO_RHYTHM_CHANGE_BONUS: f64 = 1.0;
pub const TAIKO_RHYTHM_CHANGE_BASE_THRESHOLD: f64 = 0.2;
pub const TAIKO_RHYTHM_CHANGE_BASE: f64 = 2.0;

// main struct
pub type ezpp_t = *mut ezpp;
extern "C" {
    pub fn ezpp_new() -> ezpp_t;
    pub fn ezpp_free(ez: ezpp_t);
    pub fn ezpp(ez: ezpp_t, map: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn ezpp_pp(ez: ezpp_t) -> f32;
    pub fn ezpp_stars(ez: ezpp_t) -> f32;
    pub fn ezpp_set_autocalc(ez: ezpp_t, autocalc: ::std::os::raw::c_int);
    pub fn ezpp_autocalc(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_data(
        ez: ezpp_t,
        data: *mut ::std::os::raw::c_char,
        data_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ezpp_aim_stars(ez: ezpp_t) -> f32;
    pub fn ezpp_speed_stars(ez: ezpp_t) -> f32;
    pub fn ezpp_aim_pp(ez: ezpp_t) -> f32;
    pub fn ezpp_speed_pp(ez: ezpp_t) -> f32;
    pub fn ezpp_acc_pp(ez: ezpp_t) -> f32;
    pub fn ezpp_accuracy_percent(ez: ezpp_t) -> f32;
    pub fn ezpp_n300(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_n100(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_n50(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_nmiss(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_ar(ez: ezpp_t) -> f32;
    pub fn ezpp_cs(ez: ezpp_t) -> f32;
    pub fn ezpp_od(ez: ezpp_t) -> f32;
    pub fn ezpp_hp(ez: ezpp_t) -> f32;
    pub fn ezpp_artist(ez: ezpp_t) -> *mut ::std::os::raw::c_char;
    pub fn ezpp_artist_unicode(ez: ezpp_t) -> *mut ::std::os::raw::c_char;
    pub fn ezpp_title(ez: ezpp_t) -> *mut ::std::os::raw::c_char;
    pub fn ezpp_title_unicode(ez: ezpp_t) -> *mut ::std::os::raw::c_char;
    pub fn ezpp_version(ez: ezpp_t) -> *mut ::std::os::raw::c_char;
    pub fn ezpp_creator(ez: ezpp_t) -> *mut ::std::os::raw::c_char;
    pub fn ezpp_ncircles(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_nsliders(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_nspinners(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_nobjects(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_odms(ez: ezpp_t) -> f32;
    pub fn ezpp_mode(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_combo(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_max_combo(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_mods(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_score_version(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn ezpp_time_at(ez: ezpp_t, i: ::std::os::raw::c_int) -> f32;
    pub fn ezpp_strain_at(
        ez: ezpp_t,
        i: ::std::os::raw::c_int,
        difficulty_type: ::std::os::raw::c_int,
    ) -> f32;
    pub fn ezpp_set_aim_stars(ez: ezpp_t, aim_stars: f32);
    pub fn ezpp_set_speed_stars(ez: ezpp_t, speed_stars: f32);
    pub fn ezpp_set_base_ar(ez: ezpp_t, ar: f32);
    pub fn ezpp_set_base_od(ez: ezpp_t, od: f32);
    pub fn ezpp_set_base_cs(ez: ezpp_t, cs: f32);
    pub fn ezpp_set_base_hp(ez: ezpp_t, hp: f32);
    pub fn ezpp_set_mode_override(ez: ezpp_t, mode_override: ::std::os::raw::c_int);
    pub fn ezpp_set_mode(ez: ezpp_t, mode: ::std::os::raw::c_int);
    pub fn ezpp_set_mods(ez: ezpp_t, mods: ::std::os::raw::c_int);
    pub fn ezpp_set_combo(ez: ezpp_t, combo: ::std::os::raw::c_int);
    pub fn ezpp_set_nmiss(ez: ezpp_t, nmiss: ::std::os::raw::c_int);
    pub fn ezpp_set_score_version(ez: ezpp_t, score_version: ::std::os::raw::c_int);
    pub fn ezpp_set_accuracy_percent(ez: ezpp_t, accuracy_percent: f32);
    pub fn ezpp_set_accuracy(ez: ezpp_t, n100: ::std::os::raw::c_int, n50: ::std::os::raw::c_int);
    pub fn ezpp_set_end(ez: ezpp_t, end: ::std::os::raw::c_int);
    pub fn ezpp_set_end_time(ez: ezpp_t, end: f32);
    pub fn ezpp_dup(ez: ezpp_t, mapfile: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn ezpp_data_dup(
        ez: ezpp_t,
        data: *mut ::std::os::raw::c_char,
        data_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn errstr(err: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn oppai_version(
        major: *mut ::std::os::raw::c_int,
        minor: *mut ::std::os::raw::c_int,
        patch: *mut ::std::os::raw::c_int,
    );
    pub fn oppai_version_str() -> *mut ::std::os::raw::c_char;
}
pub type _bindgen_ty_1 = u32;
extern "C" {
    pub fn info(fmt: *mut ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    pub fn get_inf() -> f32;
    pub fn get_nan() -> f32;
    pub fn v2f_sub(dst: *mut f32, a: *mut f32, b: *mut f32);
    pub fn v2f_len(v: *mut f32) -> f32;
    pub fn v2f_dot(a: *mut f32, b: *mut f32) -> f32;
    pub fn is_nan(b: f32) -> ::std::os::raw::c_int;
    pub fn is_inf(b: f32) -> ::std::os::raw::c_int;
    pub fn whitespace(c: ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct slice {
    pub start: *mut ::std::os::raw::c_char,
    pub end: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_slice() {
    assert_eq!(
        ::std::mem::size_of::<slice>(),
        16usize,
        concat!("Size of: ", stringify!(slice))
    );
    assert_eq!(
        ::std::mem::align_of::<slice>(),
        8usize,
        concat!("Alignment of ", stringify!(slice))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<slice>())).start as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(slice),
            "::",
            stringify!(start)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<slice>())).end as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(slice),
            "::",
            stringify!(end)
        )
    );
}
pub type slice_t = slice;
extern "C" {
    pub fn slice_write(s: *mut slice_t, f: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn slice_whitespace(s: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn slice_trim(s: *mut slice_t);
    pub fn slice_cmp(s: *mut slice_t, str: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn slice_len(s: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn slice_split(
        s: *mut slice_t,
        separator_list: *mut ::std::os::raw::c_char,
        arr: *mut slice_t,
        nmax: ::std::os::raw::c_int,
        err: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn array_reserve_i(
        n: ::std::os::raw::c_int,
        cap: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
        data: *mut *mut ::std::os::raw::c_void,
        esize: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn array_free_i(
        cap: *mut ::std::os::raw::c_int,
        len: *mut ::std::os::raw::c_int,
        data: *mut *mut ::std::os::raw::c_void,
        esize: ::std::os::raw::c_int,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timing {
    pub time: f32,
    pub ms_per_beat: f32,
    pub change: ::std::os::raw::c_int,
    pub px_per_beat: f32,
    pub beat_len: f32,
    pub velocity: f32,
}
#[test]
fn bindgen_test_layout_timing() {
    assert_eq!(
        ::std::mem::size_of::<timing>(),
        24usize,
        concat!("Size of: ", stringify!(timing))
    );
    assert_eq!(
        ::std::mem::align_of::<timing>(),
        4usize,
        concat!("Alignment of ", stringify!(timing))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timing>())).time as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timing),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timing>())).ms_per_beat as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(timing),
            "::",
            stringify!(ms_per_beat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timing>())).change as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timing),
            "::",
            stringify!(change)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timing>())).px_per_beat as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(timing),
            "::",
            stringify!(px_per_beat)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timing>())).beat_len as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(timing),
            "::",
            stringify!(beat_len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timing>())).velocity as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(timing),
            "::",
            stringify!(velocity)
        )
    );
}
pub type timing_t = timing;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct object {
    pub time: f32,
    pub type_: ::std::os::raw::c_int,
    pub nsound_types: ::std::os::raw::c_int,
    pub sound_types: *mut ::std::os::raw::c_int,
    pub normpos: [f32; 2usize],
    pub angle: f32,
    pub strains: [f32; 2usize],
    pub is_single: ::std::os::raw::c_int,
    pub delta_time: f32,
    pub d_distance: f32,
    pub timing_point: ::std::os::raw::c_int,
    pub pos: [f32; 2usize],
    pub distance: f32,
    pub repetitions: ::std::os::raw::c_int,
    pub duration: f32,
    pub tick_spacing: f32,
    pub slider_is_drum_roll: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_object() {
    assert_eq!(
        ::std::mem::size_of::<object>(),
        88usize,
        concat!("Size of: ", stringify!(object))
    );
    assert_eq!(
        ::std::mem::align_of::<object>(),
        8usize,
        concat!("Alignment of ", stringify!(object))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).time as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).type_ as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).nsound_types as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(nsound_types)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).sound_types as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(sound_types)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).normpos as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(normpos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).angle as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(angle)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).strains as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(strains)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).is_single as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(is_single)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).delta_time as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(delta_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).d_distance as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(d_distance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).timing_point as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(timing_point)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).pos as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(pos)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).distance as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(distance)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).repetitions as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(repetitions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).duration as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(duration)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).tick_spacing as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(tick_spacing)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<object>())).slider_is_drum_roll as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(object),
            "::",
            stringify!(slider_is_drum_roll)
        )
    );
}
pub type object_t = object;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ezpp {
    pub map: *mut ::std::os::raw::c_char,
    pub data: *mut ::std::os::raw::c_char,
    pub data_size: ::std::os::raw::c_int,
    pub flags: ::std::os::raw::c_int,
    pub format_version: ::std::os::raw::c_int,
    pub mode: ::std::os::raw::c_int,
    pub mode_override: ::std::os::raw::c_int,
    pub original_mode: ::std::os::raw::c_int,
    pub score_version: ::std::os::raw::c_int,
    pub mods: ::std::os::raw::c_int,
    pub combo: ::std::os::raw::c_int,
    pub accuracy_percent: f32,
    pub n300: ::std::os::raw::c_int,
    pub n100: ::std::os::raw::c_int,
    pub n50: ::std::os::raw::c_int,
    pub nmiss: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
    pub end_time: f32,
    pub base_ar: f32,
    pub base_cs: f32,
    pub base_od: f32,
    pub base_hp: f32,
    pub max_combo: ::std::os::raw::c_int,
    pub title: *mut ::std::os::raw::c_char,
    pub title_unicode: *mut ::std::os::raw::c_char,
    pub artist: *mut ::std::os::raw::c_char,
    pub artist_unicode: *mut ::std::os::raw::c_char,
    pub creator: *mut ::std::os::raw::c_char,
    pub version: *mut ::std::os::raw::c_char,
    pub ncircles: ::std::os::raw::c_int,
    pub nsliders: ::std::os::raw::c_int,
    pub nspinners: ::std::os::raw::c_int,
    pub nobjects: ::std::os::raw::c_int,
    pub ar: f32,
    pub od: f32,
    pub cs: f32,
    pub hp: f32,
    pub odms: f32,
    pub sv: f32,
    pub tick_rate: f32,
    pub speed_mul: f32,
    pub stars: f32,
    pub aim_stars: f32,
    pub aim_difficulty: f32,
    pub aim_length_bonus: f32,
    pub speed_stars: f32,
    pub speed_difficulty: f32,
    pub speed_length_bonus: f32,
    pub pp: f32,
    pub aim_pp: f32,
    pub speed_pp: f32,
    pub acc_pp: f32,
    pub section: [::std::os::raw::c_char; 64usize],
    pub buf: [::std::os::raw::c_char; 65535usize],
    pub p_flags: ::std::os::raw::c_int,
    pub objects: ezpp__bindgen_ty_1,
    pub timing_points: ezpp__bindgen_ty_2,
    pub interval_end: f32,
    pub max_strain: f32,
    pub highest_strains: ezpp__bindgen_ty_3,
    pub block: *mut ::std::os::raw::c_char,
    pub end_of_block: *mut ::std::os::raw::c_char,
    pub blocks: ezpp__bindgen_ty_4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ezpp__bindgen_ty_1 {
    pub cap: ::std::os::raw::c_int,
    pub len: ::std::os::raw::c_int,
    pub data: *mut object_t,
}
#[test]
fn bindgen_test_layout_ezpp__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<ezpp__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(ezpp__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<ezpp__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(ezpp__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_1>())).cap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_1),
            "::",
            stringify!(cap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_1>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_1),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_1>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_1),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ezpp__bindgen_ty_2 {
    pub cap: ::std::os::raw::c_int,
    pub len: ::std::os::raw::c_int,
    pub data: *mut timing_t,
}
#[test]
fn bindgen_test_layout_ezpp__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<ezpp__bindgen_ty_2>(),
        16usize,
        concat!("Size of: ", stringify!(ezpp__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<ezpp__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(ezpp__bindgen_ty_2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_2>())).cap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_2),
            "::",
            stringify!(cap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_2>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_2),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_2>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_2),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ezpp__bindgen_ty_3 {
    pub cap: ::std::os::raw::c_int,
    pub len: ::std::os::raw::c_int,
    pub data: *mut f32,
}
#[test]
fn bindgen_test_layout_ezpp__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<ezpp__bindgen_ty_3>(),
        16usize,
        concat!("Size of: ", stringify!(ezpp__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<ezpp__bindgen_ty_3>(),
        8usize,
        concat!("Alignment of ", stringify!(ezpp__bindgen_ty_3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_3>())).cap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_3),
            "::",
            stringify!(cap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_3>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_3),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_3>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_3),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ezpp__bindgen_ty_4 {
    pub cap: ::std::os::raw::c_int,
    pub len: ::std::os::raw::c_int,
    pub data: *mut *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_ezpp__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<ezpp__bindgen_ty_4>(),
        16usize,
        concat!("Size of: ", stringify!(ezpp__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<ezpp__bindgen_ty_4>(),
        8usize,
        concat!("Alignment of ", stringify!(ezpp__bindgen_ty_4))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_4>())).cap as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_4),
            "::",
            stringify!(cap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_4>())).len as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_4),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp__bindgen_ty_4>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp__bindgen_ty_4),
            "::",
            stringify!(data)
        )
    );
}
#[test]
fn bindgen_test_layout_ezpp() {
    assert_eq!(
        ::std::mem::size_of::<ezpp>(),
        65936usize,
        concat!("Size of: ", stringify!(ezpp))
    );
    assert_eq!(
        ::std::mem::align_of::<ezpp>(),
        8usize,
        concat!("Alignment of ", stringify!(ezpp))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).map as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(map))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).data_size as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(data_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).flags as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).format_version as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(format_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).mode as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).mode_override as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(mode_override)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).original_mode as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(original_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).score_version as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(score_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).mods as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(mods)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).combo as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(combo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).accuracy_percent as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(accuracy_percent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).n300 as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(n300)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).n100 as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(n100)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).n50 as *const _ as usize },
        64usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(n50))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).nmiss as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(nmiss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).end as *const _ as usize },
        72usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(end))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).end_time as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(end_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).base_ar as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(base_ar)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).base_cs as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(base_cs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).base_od as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(base_od)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).base_hp as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(base_hp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).max_combo as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(max_combo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).title as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(title)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).title_unicode as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(title_unicode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).artist as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(artist)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).artist_unicode as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(artist_unicode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).creator as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(creator)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).version as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).ncircles as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(ncircles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).nsliders as *const _ as usize },
        156usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(nsliders)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).nspinners as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(nspinners)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).nobjects as *const _ as usize },
        164usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(nobjects)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).ar as *const _ as usize },
        168usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(ar))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).od as *const _ as usize },
        172usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(od))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).cs as *const _ as usize },
        176usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(cs))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).hp as *const _ as usize },
        180usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(hp))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).odms as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(odms)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).sv as *const _ as usize },
        188usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(sv))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).tick_rate as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(tick_rate)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).speed_mul as *const _ as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(speed_mul)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).stars as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(stars)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).aim_stars as *const _ as usize },
        204usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(aim_stars)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).aim_difficulty as *const _ as usize },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(aim_difficulty)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).aim_length_bonus as *const _ as usize },
        212usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(aim_length_bonus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).speed_stars as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(speed_stars)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).speed_difficulty as *const _ as usize },
        220usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(speed_difficulty)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).speed_length_bonus as *const _ as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(speed_length_bonus)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).pp as *const _ as usize },
        228usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(pp))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).aim_pp as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(aim_pp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).speed_pp as *const _ as usize },
        236usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(speed_pp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).acc_pp as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(acc_pp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).section as *const _ as usize },
        244usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(section)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).buf as *const _ as usize },
        308usize,
        concat!("Offset of field: ", stringify!(ezpp), "::", stringify!(buf))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).p_flags as *const _ as usize },
        65844usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(p_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).objects as *const _ as usize },
        65848usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(objects)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).timing_points as *const _ as usize },
        65864usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(timing_points)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).interval_end as *const _ as usize },
        65880usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(interval_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).max_strain as *const _ as usize },
        65884usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(max_strain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).highest_strains as *const _ as usize },
        65888usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(highest_strains)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).block as *const _ as usize },
        65904usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(block)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).end_of_block as *const _ as usize },
        65912usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(end_of_block)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ezpp>())).blocks as *const _ as usize },
        65920usize,
        concat!(
            "Offset of field: ",
            stringify!(ezpp),
            "::",
            stringify!(blocks)
        )
    );
}
extern "C" {
    pub fn m_reserve(ez: ezpp_t, min_size: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn m_alloc(ez: ezpp_t, size: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_void;
    pub fn m_strndup(
        ez: ezpp_t,
        s: *mut ::std::os::raw::c_char,
        n: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn m_free(ez: ezpp_t);
    pub static mut od10_ms: [f32; 2usize];
    pub static mut od0_ms: [f32; 2usize];
    pub static mut od_ms_step: [f32; 2usize];
    pub fn mods_apply(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub static mut playfield_center: [f32; 2usize];
    pub fn print_line(line: *mut slice_t);
    pub fn p_warn(e: *mut ::std::os::raw::c_char, line: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_consume_til(
        s: *mut slice_t,
        separators: *mut ::std::os::raw::c_char,
        dst: *mut slice_t,
    ) -> ::std::os::raw::c_int;
    pub fn p_float(value: *mut slice_t) -> f32;
    pub fn p_section_name(s: *mut slice_t, name: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_property(
        s: *mut slice_t,
        name: *mut slice_t,
        value: *mut slice_t,
    ) -> ::std::os::raw::c_int;
    pub fn p_slicedup(ez: ezpp_t, s: *mut slice_t) -> *mut ::std::os::raw::c_char;
    pub fn p_metadata(ez: ezpp_t, line: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_general(ez: ezpp_t, line: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_difficulty(ez: ezpp_t, line: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_timing(ez: ezpp_t, line: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_objects(ez: ezpp_t, line: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_line(ez: ezpp_t, line: *mut slice_t) -> ::std::os::raw::c_int;
    pub fn p_end(ez: ezpp_t);
    pub fn p_reset(ez: ezpp_t);
    pub fn p_map(ez: ezpp_t, f: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
    pub fn p_map_mem(
        ez: ezpp_t,
        data: *mut ::std::os::raw::c_char,
        data_size: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub static mut decay_base: [f32; 2usize];
    pub static mut weight_scaling: [f32; 2usize];
    pub fn d_spacing_weight(
        distance: f32,
        delta_time: f32,
        prev_distance: f32,
        prev_delta_time: f32,
        angle: f32,
        type_: ::std::os::raw::c_int,
        is_single: *mut ::std::os::raw::c_int,
    ) -> f32;
    pub fn d_calc_strain(
        type_: ::std::os::raw::c_int,
        o: *mut object_t,
        prev: *mut object_t,
        speedmul: f32,
    );
    pub fn dbl_desc(
        a: *const ::std::os::raw::c_void,
        b: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn d_update_max_strains(
        ez: ezpp_t,
        decay_factor: f32,
        cur_time: f32,
        prev_time: f32,
        cur_strain: f32,
        prev_strain: f32,
        first_obj: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn d_weigh_strains(ez: ezpp_t, pdiff: *mut f32, ptotal: *mut f32);
    pub fn d_calc_individual(ez: ezpp_t, type_: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn d_length_bonus(stars: f32, difficulty: f32) -> f32;
    pub fn d_std(ez: ezpp_t) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct taiko_object {
    pub hit: ::std::os::raw::c_int,
    pub strain: f32,
    pub time: f32,
    pub time_elapsed: f32,
    pub rim: ::std::os::raw::c_int,
    pub same_since: ::std::os::raw::c_int,
    pub last_switch_even: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_taiko_object() {
    assert_eq!(
        ::std::mem::size_of::<taiko_object>(),
        28usize,
        concat!("Size of: ", stringify!(taiko_object))
    );
    assert_eq!(
        ::std::mem::align_of::<taiko_object>(),
        4usize,
        concat!("Alignment of ", stringify!(taiko_object))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taiko_object>())).hit as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(taiko_object),
            "::",
            stringify!(hit)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taiko_object>())).strain as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(taiko_object),
            "::",
            stringify!(strain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taiko_object>())).time as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(taiko_object),
            "::",
            stringify!(time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taiko_object>())).time_elapsed as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(taiko_object),
            "::",
            stringify!(time_elapsed)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taiko_object>())).rim as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(taiko_object),
            "::",
            stringify!(rim)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taiko_object>())).same_since as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(taiko_object),
            "::",
            stringify!(same_since)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<taiko_object>())).last_switch_even as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(taiko_object),
            "::",
            stringify!(last_switch_even)
        )
    );
}
pub type taiko_object_t = taiko_object;
extern "C" {
    pub fn taiko_change_bonus(cur: *mut taiko_object_t, prev: *mut taiko_object_t) -> f32;
    pub fn taiko_rhythm_bonus(cur: *mut taiko_object_t, prev: *mut taiko_object_t) -> f32;
    pub fn taiko_strain(cur: *mut taiko_object_t, prev: *mut taiko_object_t);
    pub fn swap_ptrs(a: *mut *mut ::std::os::raw::c_void, b: *mut *mut ::std::os::raw::c_void);
    pub fn d_taiko(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn d_calc(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn acc_calc(
        n300: ::std::os::raw::c_int,
        n100: ::std::os::raw::c_int,
        n50: ::std::os::raw::c_int,
        misses: ::std::os::raw::c_int,
    ) -> f32;
    pub fn acc_round(
        acc_percent: f32,
        nobjects: ::std::os::raw::c_int,
        misses: ::std::os::raw::c_int,
        n300: *mut ::std::os::raw::c_int,
        n100: *mut ::std::os::raw::c_int,
        n50: *mut ::std::os::raw::c_int,
    );
    pub fn taiko_acc_calc(
        n300: ::std::os::raw::c_int,
        n150: ::std::os::raw::c_int,
        nmiss: ::std::os::raw::c_int,
    ) -> f32;
    pub fn taiko_acc_round(
        acc_percent: f32,
        nobjects: ::std::os::raw::c_int,
        nmisses: ::std::os::raw::c_int,
        n300: *mut ::std::os::raw::c_int,
        n150: *mut ::std::os::raw::c_int,
    );
    pub fn base_pp(stars: f32) -> f32;
    pub fn pp_std(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn pp_taiko(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn params_from_map(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn calc(ez: ezpp_t) -> ::std::os::raw::c_int;
    pub fn free_owned_map(ez: ezpp_t);
    pub fn memclone(
        p: *mut ::std::os::raw::c_void,
        size: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
    pub fn strclone(s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
