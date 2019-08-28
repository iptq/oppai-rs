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

    /// Calculate pp from map file
    ///
    /// pass `"-"` to read from stdin
    pub fn calculate(&self, mapfile: impl AsRef<str>) {
        let mapfile = mapfile.as_ref();
        let mapfile_cstr = CString::new(mapfile).expect("null bytes in mapfile");
        unsafe { oppai_sys::ezpp(self.0, mapfile_cstr.as_ptr() as *mut _) };
    }
}

impl Drop for Oppai {
    fn drop(&mut self) {
        unsafe { oppai_sys::ezpp_free(self.0) };
    }
}
