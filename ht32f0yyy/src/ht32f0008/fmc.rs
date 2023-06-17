#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TADR"]
    pub tadr: TADR,
    #[doc = "0x04 - WRDR"]
    pub wrdr: WRDR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - OCMR"]
    pub ocmr: OCMR,
    #[doc = "0x10 - OPCR"]
    pub opcr: OPCR,
    #[doc = "0x14 - OIER"]
    pub oier: OIER,
    #[doc = "0x18 - OISR"]
    pub oisr: OISR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - PPSR0"]
    pub ppsr0: PPSR0,
    #[doc = "0x24 - PPSR1"]
    pub ppsr1: PPSR1,
    #[doc = "0x28 - PPSR2"]
    pub ppsr2: PPSR2,
    #[doc = "0x2c - PPSR3"]
    pub ppsr3: PPSR3,
    #[doc = "0x30 - CPSR"]
    pub cpsr: CPSR,
    _reserved11: [u8; 0xcc],
    #[doc = "0x100 - VMCR"]
    pub vmcr: VMCR,
    _reserved12: [u8; 0x7c],
    #[doc = "0x180 - MDID"]
    pub mdid: MDID,
    #[doc = "0x184 - PNSR"]
    pub pnsr: PNSR,
    #[doc = "0x188 - PSSR"]
    pub pssr: PSSR,
    #[doc = "0x18c - DID"]
    pub did: DID,
    _reserved16: [u8; 0x70],
    #[doc = "0x200 - CFCR"]
    pub cfcr: CFCR,
    _reserved17: [u8; 0x010c],
    #[doc = "0x310 - CIDR0"]
    pub cidr0: CIDR0,
    #[doc = "0x314 - CIDR1"]
    pub cidr1: CIDR1,
    #[doc = "0x318 - CIDR2"]
    pub cidr2: CIDR2,
    #[doc = "0x31c - CIDR3"]
    pub cidr3: CIDR3,
}
#[doc = "TADR (rw) register accessor: an alias for `Reg<TADR_SPEC>`"]
pub type TADR = crate::Reg<tadr::TADR_SPEC>;
#[doc = "TADR"]
pub mod tadr;
#[doc = "WRDR (rw) register accessor: an alias for `Reg<WRDR_SPEC>`"]
pub type WRDR = crate::Reg<wrdr::WRDR_SPEC>;
#[doc = "WRDR"]
pub mod wrdr;
#[doc = "OCMR (rw) register accessor: an alias for `Reg<OCMR_SPEC>`"]
pub type OCMR = crate::Reg<ocmr::OCMR_SPEC>;
#[doc = "OCMR"]
pub mod ocmr;
#[doc = "OPCR (rw) register accessor: an alias for `Reg<OPCR_SPEC>`"]
pub type OPCR = crate::Reg<opcr::OPCR_SPEC>;
#[doc = "OPCR"]
pub mod opcr;
#[doc = "OIER (rw) register accessor: an alias for `Reg<OIER_SPEC>`"]
pub type OIER = crate::Reg<oier::OIER_SPEC>;
#[doc = "OIER"]
pub mod oier;
#[doc = "OISR (rw) register accessor: an alias for `Reg<OISR_SPEC>`"]
pub type OISR = crate::Reg<oisr::OISR_SPEC>;
#[doc = "OISR"]
pub mod oisr;
#[doc = "PPSR0 (rw) register accessor: an alias for `Reg<PPSR0_SPEC>`"]
pub type PPSR0 = crate::Reg<ppsr0::PPSR0_SPEC>;
#[doc = "PPSR0"]
pub mod ppsr0;
#[doc = "PPSR1 (rw) register accessor: an alias for `Reg<PPSR1_SPEC>`"]
pub type PPSR1 = crate::Reg<ppsr1::PPSR1_SPEC>;
#[doc = "PPSR1"]
pub mod ppsr1;
#[doc = "PPSR2 (rw) register accessor: an alias for `Reg<PPSR2_SPEC>`"]
pub type PPSR2 = crate::Reg<ppsr2::PPSR2_SPEC>;
#[doc = "PPSR2"]
pub mod ppsr2;
#[doc = "PPSR3 (rw) register accessor: an alias for `Reg<PPSR3_SPEC>`"]
pub type PPSR3 = crate::Reg<ppsr3::PPSR3_SPEC>;
#[doc = "PPSR3"]
pub mod ppsr3;
#[doc = "CPSR (rw) register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "CPSR"]
pub mod cpsr;
#[doc = "VMCR (rw) register accessor: an alias for `Reg<VMCR_SPEC>`"]
pub type VMCR = crate::Reg<vmcr::VMCR_SPEC>;
#[doc = "VMCR"]
pub mod vmcr;
#[doc = "MDID (rw) register accessor: an alias for `Reg<MDID_SPEC>`"]
pub type MDID = crate::Reg<mdid::MDID_SPEC>;
#[doc = "MDID"]
pub mod mdid;
#[doc = "PNSR (rw) register accessor: an alias for `Reg<PNSR_SPEC>`"]
pub type PNSR = crate::Reg<pnsr::PNSR_SPEC>;
#[doc = "PNSR"]
pub mod pnsr;
#[doc = "PSSR (rw) register accessor: an alias for `Reg<PSSR_SPEC>`"]
pub type PSSR = crate::Reg<pssr::PSSR_SPEC>;
#[doc = "PSSR"]
pub mod pssr;
#[doc = "DID (rw) register accessor: an alias for `Reg<DID_SPEC>`"]
pub type DID = crate::Reg<did::DID_SPEC>;
#[doc = "DID"]
pub mod did;
#[doc = "CFCR (rw) register accessor: an alias for `Reg<CFCR_SPEC>`"]
pub type CFCR = crate::Reg<cfcr::CFCR_SPEC>;
#[doc = "CFCR"]
pub mod cfcr;
#[doc = "CIDR0 (rw) register accessor: an alias for `Reg<CIDR0_SPEC>`"]
pub type CIDR0 = crate::Reg<cidr0::CIDR0_SPEC>;
#[doc = "CIDR0"]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: an alias for `Reg<CIDR1_SPEC>`"]
pub type CIDR1 = crate::Reg<cidr1::CIDR1_SPEC>;
#[doc = "CIDR1"]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: an alias for `Reg<CIDR2_SPEC>`"]
pub type CIDR2 = crate::Reg<cidr2::CIDR2_SPEC>;
#[doc = "CIDR2"]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: an alias for `Reg<CIDR3_SPEC>`"]
pub type CIDR3 = crate::Reg<cidr3::CIDR3_SPEC>;
#[doc = "CIDR3"]
pub mod cidr3;
