#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CTRL"]
    pub ctrl: CTRL,
    #[doc = "0x04 - LOAD"]
    pub load: LOAD,
    #[doc = "0x08 - VAL"]
    pub val: VAL,
    #[doc = "0x0c - CALIB"]
    pub calib: CALIB,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CTRL"]
pub mod ctrl;
#[doc = "LOAD (rw) register accessor: an alias for `Reg<LOAD_SPEC>`"]
pub type LOAD = crate::Reg<load::LOAD_SPEC>;
#[doc = "LOAD"]
pub mod load;
#[doc = "VAL (rw) register accessor: an alias for `Reg<VAL_SPEC>`"]
pub type VAL = crate::Reg<val::VAL_SPEC>;
#[doc = "VAL"]
pub mod val;
#[doc = "CALIB (rw) register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "CALIB"]
pub mod calib;
