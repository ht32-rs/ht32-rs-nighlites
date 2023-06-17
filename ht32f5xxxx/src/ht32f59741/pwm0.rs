#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM_CNTCFR"]
    pub pwm_cntcfr: PWM_CNTCFR,
    #[doc = "0x04 - PWM_MDCFR"]
    pub pwm_mdcfr: PWM_MDCFR,
    #[doc = "0x08 - PWM_TRCFR"]
    pub pwm_trcfr: PWM_TRCFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - PWM_CTR"]
    pub pwm_ctr: PWM_CTR,
    _reserved4: [u8; 0x2c],
    #[doc = "0x40 - PWM_CH0OCFR"]
    pub pwm_ch0ocfr: PWM_CH0OCFR,
    #[doc = "0x44 - PWM_CH1OCFR"]
    pub pwm_ch1ocfr: PWM_CH1OCFR,
    #[doc = "0x48 - PWM_CH2OCFR"]
    pub pwm_ch2ocfr: PWM_CH2OCFR,
    #[doc = "0x4c - PWM_CH3OCFR"]
    pub pwm_ch3ocfr: PWM_CH3OCFR,
    #[doc = "0x50 - PWM_CHCTR"]
    pub pwm_chctr: PWM_CHCTR,
    #[doc = "0x54 - PWM_CHPOLR"]
    pub pwm_chpolr: PWM_CHPOLR,
    _reserved10: [u8; 0x1c],
    #[doc = "0x74 - PWM_DICTR"]
    pub pwm_dictr: PWM_DICTR,
    #[doc = "0x78 - PWM_EVGR"]
    pub pwm_evgr: PWM_EVGR,
    #[doc = "0x7c - PWM_INTSR"]
    pub pwm_intsr: PWM_INTSR,
    #[doc = "0x80 - PWM_CNTR"]
    pub pwm_cntr: PWM_CNTR,
    #[doc = "0x84 - PWM_PSCR"]
    pub pwm_pscr: PWM_PSCR,
    #[doc = "0x88 - PWM_CRR"]
    pub pwm_crr: PWM_CRR,
    _reserved16: [u8; 0x04],
    #[doc = "0x90 - PWM_CH0CR"]
    pub pwm_ch0cr: PWM_CH0CR,
    #[doc = "0x94 - PWM_CH1CC"]
    pub pwm_ch1ccr: PWM_CH1CCR,
    #[doc = "0x98 - PWM_CH2CR"]
    pub pwm_ch2cr: PWM_CH2CR,
    #[doc = "0x9c - PWM_CH3CR"]
    pub pwm_ch3cr: PWM_CH3CR,
    #[doc = "0xa0 - PWM_CH0ACR"]
    pub pwm_ch0acr: PWM_CH0ACR,
    #[doc = "0xa4 - PWM_CH1ACR"]
    pub pwm_ch1acr: PWM_CH1ACR,
    #[doc = "0xa8 - PWM_CH2ACR"]
    pub pwm_ch2acr: PWM_CH2ACR,
    #[doc = "0xac - PWM_CH3ACR"]
    pub pwm_ch3acr: PWM_CH3ACR,
}
#[doc = "PWM_CNTCFR (rw) register accessor: an alias for `Reg<PWM_CNTCFR_SPEC>`"]
pub type PWM_CNTCFR = crate::Reg<pwm_cntcfr::PWM_CNTCFR_SPEC>;
#[doc = "PWM_CNTCFR"]
pub mod pwm_cntcfr;
#[doc = "PWM_MDCFR (rw) register accessor: an alias for `Reg<PWM_MDCFR_SPEC>`"]
pub type PWM_MDCFR = crate::Reg<pwm_mdcfr::PWM_MDCFR_SPEC>;
#[doc = "PWM_MDCFR"]
pub mod pwm_mdcfr;
#[doc = "PWM_TRCFR (rw) register accessor: an alias for `Reg<PWM_TRCFR_SPEC>`"]
pub type PWM_TRCFR = crate::Reg<pwm_trcfr::PWM_TRCFR_SPEC>;
#[doc = "PWM_TRCFR"]
pub mod pwm_trcfr;
#[doc = "PWM_CTR (rw) register accessor: an alias for `Reg<PWM_CTR_SPEC>`"]
pub type PWM_CTR = crate::Reg<pwm_ctr::PWM_CTR_SPEC>;
#[doc = "PWM_CTR"]
pub mod pwm_ctr;
#[doc = "PWM_CH0OCFR (rw) register accessor: an alias for `Reg<PWM_CH0OCFR_SPEC>`"]
pub type PWM_CH0OCFR = crate::Reg<pwm_ch0ocfr::PWM_CH0OCFR_SPEC>;
#[doc = "PWM_CH0OCFR"]
pub mod pwm_ch0ocfr;
#[doc = "PWM_CH1OCFR (rw) register accessor: an alias for `Reg<PWM_CH1OCFR_SPEC>`"]
pub type PWM_CH1OCFR = crate::Reg<pwm_ch1ocfr::PWM_CH1OCFR_SPEC>;
#[doc = "PWM_CH1OCFR"]
pub mod pwm_ch1ocfr;
#[doc = "PWM_CH2OCFR (rw) register accessor: an alias for `Reg<PWM_CH2OCFR_SPEC>`"]
pub type PWM_CH2OCFR = crate::Reg<pwm_ch2ocfr::PWM_CH2OCFR_SPEC>;
#[doc = "PWM_CH2OCFR"]
pub mod pwm_ch2ocfr;
#[doc = "PWM_CH3OCFR (rw) register accessor: an alias for `Reg<PWM_CH3OCFR_SPEC>`"]
pub type PWM_CH3OCFR = crate::Reg<pwm_ch3ocfr::PWM_CH3OCFR_SPEC>;
#[doc = "PWM_CH3OCFR"]
pub mod pwm_ch3ocfr;
#[doc = "PWM_CHCTR (rw) register accessor: an alias for `Reg<PWM_CHCTR_SPEC>`"]
pub type PWM_CHCTR = crate::Reg<pwm_chctr::PWM_CHCTR_SPEC>;
#[doc = "PWM_CHCTR"]
pub mod pwm_chctr;
#[doc = "PWM_CHPOLR (rw) register accessor: an alias for `Reg<PWM_CHPOLR_SPEC>`"]
pub type PWM_CHPOLR = crate::Reg<pwm_chpolr::PWM_CHPOLR_SPEC>;
#[doc = "PWM_CHPOLR"]
pub mod pwm_chpolr;
#[doc = "PWM_DICTR (rw) register accessor: an alias for `Reg<PWM_DICTR_SPEC>`"]
pub type PWM_DICTR = crate::Reg<pwm_dictr::PWM_DICTR_SPEC>;
#[doc = "PWM_DICTR"]
pub mod pwm_dictr;
#[doc = "PWM_EVGR (rw) register accessor: an alias for `Reg<PWM_EVGR_SPEC>`"]
pub type PWM_EVGR = crate::Reg<pwm_evgr::PWM_EVGR_SPEC>;
#[doc = "PWM_EVGR"]
pub mod pwm_evgr;
#[doc = "PWM_INTSR (rw) register accessor: an alias for `Reg<PWM_INTSR_SPEC>`"]
pub type PWM_INTSR = crate::Reg<pwm_intsr::PWM_INTSR_SPEC>;
#[doc = "PWM_INTSR"]
pub mod pwm_intsr;
#[doc = "PWM_CNTR (rw) register accessor: an alias for `Reg<PWM_CNTR_SPEC>`"]
pub type PWM_CNTR = crate::Reg<pwm_cntr::PWM_CNTR_SPEC>;
#[doc = "PWM_CNTR"]
pub mod pwm_cntr;
#[doc = "PWM_PSCR (rw) register accessor: an alias for `Reg<PWM_PSCR_SPEC>`"]
pub type PWM_PSCR = crate::Reg<pwm_pscr::PWM_PSCR_SPEC>;
#[doc = "PWM_PSCR"]
pub mod pwm_pscr;
#[doc = "PWM_CRR (rw) register accessor: an alias for `Reg<PWM_CRR_SPEC>`"]
pub type PWM_CRR = crate::Reg<pwm_crr::PWM_CRR_SPEC>;
#[doc = "PWM_CRR"]
pub mod pwm_crr;
#[doc = "PWM_CH0CR (rw) register accessor: an alias for `Reg<PWM_CH0CR_SPEC>`"]
pub type PWM_CH0CR = crate::Reg<pwm_ch0cr::PWM_CH0CR_SPEC>;
#[doc = "PWM_CH0CR"]
pub mod pwm_ch0cr;
#[doc = "PWM_CH1CCR (rw) register accessor: an alias for `Reg<PWM_CH1CCR_SPEC>`"]
pub type PWM_CH1CCR = crate::Reg<pwm_ch1ccr::PWM_CH1CCR_SPEC>;
#[doc = "PWM_CH1CC"]
pub mod pwm_ch1ccr;
#[doc = "PWM_CH2CR (rw) register accessor: an alias for `Reg<PWM_CH2CR_SPEC>`"]
pub type PWM_CH2CR = crate::Reg<pwm_ch2cr::PWM_CH2CR_SPEC>;
#[doc = "PWM_CH2CR"]
pub mod pwm_ch2cr;
#[doc = "PWM_CH3CR (rw) register accessor: an alias for `Reg<PWM_CH3CR_SPEC>`"]
pub type PWM_CH3CR = crate::Reg<pwm_ch3cr::PWM_CH3CR_SPEC>;
#[doc = "PWM_CH3CR"]
pub mod pwm_ch3cr;
#[doc = "PWM_CH0ACR (rw) register accessor: an alias for `Reg<PWM_CH0ACR_SPEC>`"]
pub type PWM_CH0ACR = crate::Reg<pwm_ch0acr::PWM_CH0ACR_SPEC>;
#[doc = "PWM_CH0ACR"]
pub mod pwm_ch0acr;
#[doc = "PWM_CH1ACR (rw) register accessor: an alias for `Reg<PWM_CH1ACR_SPEC>`"]
pub type PWM_CH1ACR = crate::Reg<pwm_ch1acr::PWM_CH1ACR_SPEC>;
#[doc = "PWM_CH1ACR"]
pub mod pwm_ch1acr;
#[doc = "PWM_CH2ACR (rw) register accessor: an alias for `Reg<PWM_CH2ACR_SPEC>`"]
pub type PWM_CH2ACR = crate::Reg<pwm_ch2acr::PWM_CH2ACR_SPEC>;
#[doc = "PWM_CH2ACR"]
pub mod pwm_ch2acr;
#[doc = "PWM_CH3ACR (rw) register accessor: an alias for `Reg<PWM_CH3ACR_SPEC>`"]
pub type PWM_CH3ACR = crate::Reg<pwm_ch3acr::PWM_CH3ACR_SPEC>;
#[doc = "PWM_CH3ACR"]
pub mod pwm_ch3acr;
