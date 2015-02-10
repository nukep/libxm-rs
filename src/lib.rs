#![feature(core, libc, std_misc)]

extern crate "libxm-sys" as raw;
extern crate libc;

use std::mem;

#[derive(Copy, Clone)]
pub enum CreateError {
    Unknown(libc::c_int),
    ModuleDataNotSane,
    MemoryAllocationFailed
}

#[derive(Copy, Clone)]
pub struct PlayingSpeed {
    bpm: u16,
    tempo: u16
}

#[derive(Copy, Clone)]
pub struct Position {
    pattern_index: u8,
    pattern: u8,
    row: u8,
    samples: u64
}

pub struct XMContext {
    raw: *mut raw::xm_context_t,
    _marker: std::marker::NoCopy
}

unsafe impl Send for XMContext {}
unsafe impl Sync for XMContext {}

impl XMContext {
    pub fn new(mod_data: &[u8], rate: u32) -> Result<XMContext, CreateError> {
        // What if `mod_data` is unexpectedly short (say, 4 bytes long)?

        // For now, check that the length is reasonable
        if mod_data.len() < 60 {
            return Err(CreateError::ModuleDataNotSane);
        }

        unsafe {
            let mut raw = mem::uninitialized();

            let mod_data_ptr = mem::transmute(mod_data.as_ptr());

            let result = raw::xm_create_context(&mut raw, mod_data_ptr, rate);
            match result {
                0 => Ok(XMContext {
                    raw: raw,
                    _marker: std::marker::NoCopy
                }),
                1 => Err(CreateError::ModuleDataNotSane),
                2 => Err(CreateError::MemoryAllocationFailed),
                _ => Err(CreateError::Unknown(result))
            }
        }
    }

    #[inline]
    pub fn generate_samples(&mut self, output: &mut [f32]) {
        unsafe {
            let output_len = std::num::cast(output.len()).expect("Integer overflow");
            raw::xm_generate_samples(self.raw, output.as_mut_ptr(), output_len);
        }
    }

    #[inline]
    pub fn set_max_loop_count(&mut self, loopcnt: u8) {
        unsafe { raw::xm_set_max_loop_count(self.raw, loopcnt); }
    }

    #[inline]
    pub fn get_loop_count(&self) -> u8 {
        unsafe { raw::xm_get_loop_count(self.raw) }
    }

    #[inline]
    pub fn get_module_name(&self) -> &[u8] {
        // Is name always UTF-8? Another encoding?
        unsafe {
            let name = raw::xm_get_module_name(self.raw);
            // Tell Rust that the name is owned by `self`
            let name_self = mem::copy_lifetime(self, &name);

            std::ffi::c_str_to_bytes(name_self)
        }
    }

    #[inline]
    pub fn get_tracker_name(&self) -> &[u8] {
        // Is name always UTF-8? Another encoding?
        unsafe {
            let name = raw::xm_get_tracker_name(self.raw);
            // Tell Rust that the name is owned by `self`
            let name_self = mem::copy_lifetime(self, &name);

            std::ffi::c_str_to_bytes(name_self)
        }
    }

    #[inline]
    pub fn get_number_of_channels(&self) -> u16 {
        unsafe { raw::xm_get_number_of_channels(self.raw) }
    }

    #[inline]
    pub fn get_module_length(&self) -> u16 {
        unsafe { raw::xm_get_module_length(self.raw) }
    }

    #[inline]
    pub fn get_number_of_patterns(&self) -> u16 {
        unsafe { raw::xm_get_number_of_patterns(self.raw) }
    }

    #[inline]
    pub fn get_number_of_rows(&self, pattern: u16) -> u16 {
        assert!(pattern < self.get_number_of_patterns());

        unsafe { raw::xm_get_number_of_rows(self.raw, pattern) }
    }

    #[inline]
    pub fn get_number_of_instruments(&self) -> u16 {
        unsafe { raw::xm_get_number_of_instruments(self.raw) }
    }

    #[inline]
    pub fn get_number_of_samples(&self, instrument: u16) -> u16 {
        assert!(instrument >= 1);
        assert!(instrument <= self.get_number_of_instruments());

        unsafe { raw::xm_get_number_of_samples(self.raw, instrument) }
    }

    #[inline]
    pub fn get_playing_speed(&self) -> PlayingSpeed {
        let (mut bpm, mut tempo) = (0, 0);
        unsafe { raw::xm_get_playing_speed(self.raw, &mut bpm, &mut tempo) };

        PlayingSpeed {
            bpm: bpm,
            tempo: tempo
        }
    }

    #[inline]
    pub fn get_position(&self) -> Position {
        let (mut pattern_index, mut pattern, mut row) = (0, 0, 0);
        let mut samples = 0;
        unsafe { raw::xm_get_position(self.raw, &mut pattern_index, &mut pattern, &mut row, &mut samples) };

        Position {
            pattern_index: pattern_index,
            pattern: pattern,
            row: row,
            samples: samples
        }
    }

    #[inline]
    pub fn get_latest_trigger_of_instrument(&self, instrument: u16) -> u64 {
        assert!(instrument >= 1);
        assert!(instrument <= self.get_number_of_instruments());

        unsafe { raw::xm_get_latest_trigger_of_instrument(self.raw, instrument) }
    }

    #[inline]
    pub fn get_latest_trigger_of_sample(&self, instrument: u16, sample: u16) -> u64 {
        assert!(instrument >= 1);
        assert!(instrument <= self.get_number_of_instruments());
        assert!(sample < self.get_number_of_samples(instrument));

        unsafe { raw::xm_get_latest_trigger_of_sample(self.raw, instrument, sample) }
    }

    #[inline]
    pub fn get_latest_trigger_of_channel(&self, channel: u16) -> u64 {
        assert!(channel >= 1);
        assert!(channel <= self.get_number_of_channels());

        unsafe { raw::xm_get_latest_trigger_of_channel(self.raw, channel) }
    }
}

impl Drop for XMContext {
    fn drop(&mut self) {
        unsafe {
            raw::xm_free_context(self.raw);
        }
    }
}
