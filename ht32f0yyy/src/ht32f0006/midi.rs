#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MIDI_CHAN"]
    pub midi_chan: MIDI_CHAN,
    #[doc = "0x04 - MIDI_FREQ"]
    pub midi_freq: MIDI_FREQ,
    #[doc = "0x08 - MIDI_VOL"]
    pub midi_vol: MIDI_VOL,
    #[doc = "0x0c - MIDI_ST_ADDR"]
    pub midi_st_addr: MIDI_ST_ADDR,
    #[doc = "0x10 - MIDI_RE_NUM"]
    pub midi_re_num: MIDI_RE_NUM,
    #[doc = "0x14 - MIDI_END_ADDR"]
    pub midi_end_addr: MIDI_END_ADDR,
    #[doc = "0x18 - MIDI_IER"]
    pub midi_ier: MIDI_IER,
    #[doc = "0x1c - MIDI_STATUS"]
    pub midi_status: MIDI_STATUS,
    #[doc = "0x20 - MIDI_MCU_CH0"]
    pub midi_mcu_ch0: MIDI_MCU_CH0,
    #[doc = "0x24 - MIDI_MCU_CH1"]
    pub midi_mcu_ch1: MIDI_MCU_CH1,
    #[doc = "0x28 - MIDI_MCU_CH2"]
    pub midi_mcu_ch2: MIDI_MCU_CH2,
    #[doc = "0x2c - MIDI_MCU_CH3"]
    pub midi_mcu_ch3: MIDI_MCU_CH3,
    #[doc = "0x30 - MIDI_L"]
    pub midi_l: MIDI_L,
    #[doc = "0x34 - MIDI_R"]
    pub midi_r: MIDI_R,
    #[doc = "0x38 - MIDI_SPI_DATA"]
    pub midi_spi_data: MIDI_SPI_DATA,
    #[doc = "0x3c - MIDI_SPI_ADDR"]
    pub midi_spi_addr: MIDI_SPI_ADDR,
    #[doc = "0x40 - MIDI_CTRL"]
    pub midi_ctrl: MIDI_CTRL,
}
#[doc = "MIDI_CHAN (rw) register accessor: an alias for `Reg<MIDI_CHAN_SPEC>`"]
pub type MIDI_CHAN = crate::Reg<midi_chan::MIDI_CHAN_SPEC>;
#[doc = "MIDI_CHAN"]
pub mod midi_chan;
#[doc = "MIDI_FREQ (rw) register accessor: an alias for `Reg<MIDI_FREQ_SPEC>`"]
pub type MIDI_FREQ = crate::Reg<midi_freq::MIDI_FREQ_SPEC>;
#[doc = "MIDI_FREQ"]
pub mod midi_freq;
#[doc = "MIDI_VOL (rw) register accessor: an alias for `Reg<MIDI_VOL_SPEC>`"]
pub type MIDI_VOL = crate::Reg<midi_vol::MIDI_VOL_SPEC>;
#[doc = "MIDI_VOL"]
pub mod midi_vol;
#[doc = "MIDI_ST_ADDR (rw) register accessor: an alias for `Reg<MIDI_ST_ADDR_SPEC>`"]
pub type MIDI_ST_ADDR = crate::Reg<midi_st_addr::MIDI_ST_ADDR_SPEC>;
#[doc = "MIDI_ST_ADDR"]
pub mod midi_st_addr;
#[doc = "MIDI_RE_NUM (rw) register accessor: an alias for `Reg<MIDI_RE_NUM_SPEC>`"]
pub type MIDI_RE_NUM = crate::Reg<midi_re_num::MIDI_RE_NUM_SPEC>;
#[doc = "MIDI_RE_NUM"]
pub mod midi_re_num;
#[doc = "MIDI_END_ADDR (rw) register accessor: an alias for `Reg<MIDI_END_ADDR_SPEC>`"]
pub type MIDI_END_ADDR = crate::Reg<midi_end_addr::MIDI_END_ADDR_SPEC>;
#[doc = "MIDI_END_ADDR"]
pub mod midi_end_addr;
#[doc = "MIDI_IER (rw) register accessor: an alias for `Reg<MIDI_IER_SPEC>`"]
pub type MIDI_IER = crate::Reg<midi_ier::MIDI_IER_SPEC>;
#[doc = "MIDI_IER"]
pub mod midi_ier;
#[doc = "MIDI_STATUS (rw) register accessor: an alias for `Reg<MIDI_STATUS_SPEC>`"]
pub type MIDI_STATUS = crate::Reg<midi_status::MIDI_STATUS_SPEC>;
#[doc = "MIDI_STATUS"]
pub mod midi_status;
#[doc = "MIDI_MCU_CH0 (rw) register accessor: an alias for `Reg<MIDI_MCU_CH0_SPEC>`"]
pub type MIDI_MCU_CH0 = crate::Reg<midi_mcu_ch0::MIDI_MCU_CH0_SPEC>;
#[doc = "MIDI_MCU_CH0"]
pub mod midi_mcu_ch0;
#[doc = "MIDI_MCU_CH1 (rw) register accessor: an alias for `Reg<MIDI_MCU_CH1_SPEC>`"]
pub type MIDI_MCU_CH1 = crate::Reg<midi_mcu_ch1::MIDI_MCU_CH1_SPEC>;
#[doc = "MIDI_MCU_CH1"]
pub mod midi_mcu_ch1;
#[doc = "MIDI_MCU_CH2 (rw) register accessor: an alias for `Reg<MIDI_MCU_CH2_SPEC>`"]
pub type MIDI_MCU_CH2 = crate::Reg<midi_mcu_ch2::MIDI_MCU_CH2_SPEC>;
#[doc = "MIDI_MCU_CH2"]
pub mod midi_mcu_ch2;
#[doc = "MIDI_MCU_CH3 (rw) register accessor: an alias for `Reg<MIDI_MCU_CH3_SPEC>`"]
pub type MIDI_MCU_CH3 = crate::Reg<midi_mcu_ch3::MIDI_MCU_CH3_SPEC>;
#[doc = "MIDI_MCU_CH3"]
pub mod midi_mcu_ch3;
#[doc = "MIDI_L (rw) register accessor: an alias for `Reg<MIDI_L_SPEC>`"]
pub type MIDI_L = crate::Reg<midi_l::MIDI_L_SPEC>;
#[doc = "MIDI_L"]
pub mod midi_l;
#[doc = "MIDI_R (rw) register accessor: an alias for `Reg<MIDI_R_SPEC>`"]
pub type MIDI_R = crate::Reg<midi_r::MIDI_R_SPEC>;
#[doc = "MIDI_R"]
pub mod midi_r;
#[doc = "MIDI_SPI_DATA (rw) register accessor: an alias for `Reg<MIDI_SPI_DATA_SPEC>`"]
pub type MIDI_SPI_DATA = crate::Reg<midi_spi_data::MIDI_SPI_DATA_SPEC>;
#[doc = "MIDI_SPI_DATA"]
pub mod midi_spi_data;
#[doc = "MIDI_SPI_ADDR (rw) register accessor: an alias for `Reg<MIDI_SPI_ADDR_SPEC>`"]
pub type MIDI_SPI_ADDR = crate::Reg<midi_spi_addr::MIDI_SPI_ADDR_SPEC>;
#[doc = "MIDI_SPI_ADDR"]
pub mod midi_spi_addr;
#[doc = "MIDI_CTRL (rw) register accessor: an alias for `Reg<MIDI_CTRL_SPEC>`"]
pub type MIDI_CTRL = crate::Reg<midi_ctrl::MIDI_CTRL_SPEC>;
#[doc = "MIDI_CTRL"]
pub mod midi_ctrl;
