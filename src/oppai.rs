use std::ffi::CString;

use crate::mods::Mods;

/// The main pp calculator struct
pub struct Oppai(oppai_sys::ezpp_t);

impl Oppai {
    /// Create a new Oppai instance
    pub fn new() -> Self {
        let ezpp = unsafe { oppai_sys::ezpp_new() };
        Oppai(ezpp)
    }

    /// Set mods
    pub fn set_mods(&mut self, mods: Mods) {
        unsafe { oppai_sys::ezpp_set_mods(self.0, mods.bits() as i32) };
    }

    /// Get the pointer to the ezpp struct
    pub fn raw(&self) -> oppai_sys::ezpp_t {
        self.0
    }

    /// Calculate pp from map file
    ///
    /// pass `"-"` to read from stdin
    pub fn calculate<'a>(&mut self, mapfile: impl AsRef<str>) {
        let mapfile = mapfile.as_ref();
        let mapfile_cstr = CString::new(mapfile).expect("null bytes in mapfile");
        unsafe { oppai_sys::ezpp(self.0, mapfile_cstr.as_ptr() as *mut _) };
    }

    /// Get the pp value of the previously calculated map
    #[inline]
    pub fn get_pp(&self) -> f32 {
        unsafe { oppai_sys::ezpp_pp(self.0) }
    }

    /// Get the star rating of the previously calculated map
    #[inline]
    pub fn get_stars(&self) -> f32 {
        unsafe { oppai_sys::ezpp_stars(self.0) }
    }
}

impl Drop for Oppai {
    fn drop(&mut self) {
        unsafe { oppai_sys::ezpp_free(self.0) };
    }
}
