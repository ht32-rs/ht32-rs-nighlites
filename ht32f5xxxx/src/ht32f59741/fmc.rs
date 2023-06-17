#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - FMC_TADR"]
    pub fmc_tadr: FMC_TADR,
    #[doc = "0x04 - FMC_WRDR"]
    pub fmc_wrdr: FMC_WRDR,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - FMC_OCMR"]
    pub fmc_ocmr: FMC_OCMR,
    #[doc = "0x10 - FMC_OPCR"]
    pub fmc_opcr: FMC_OPCR,
    #[doc = "0x14 - FMC_OIER"]
    pub fmc_oier: FMC_OIER,
    #[doc = "0x18 - FMC_OISR"]
    pub fmc_oisr: FMC_OISR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - FMC_PPSR0"]
    pub fmc_ppsr0: FMC_PPSR0,
    #[doc = "0x24 - FMC_PPSR1"]
    pub fmc_ppsr1: FMC_PPSR1,
    #[doc = "0x28 - FMC_PPSR2"]
    pub fmc_ppsr2: FMC_PPSR2,
    #[doc = "0x2c - FMC_PPSR3"]
    pub fmc_ppsr3: FMC_PPSR3,
    #[doc = "0x30 - FMC_CPSR"]
    pub fmc_cpsr: FMC_CPSR,
    _reserved11: [u8; 0xcc],
    #[doc = "0x100 - FMC_VMCR"]
    pub fmc_vmcr: FMC_VMCR,
    _reserved12: [u8; 0x7c],
    #[doc = "0x180 - FMC_MDID"]
    pub fmc_mdid: FMC_MDID,
    #[doc = "0x184 - FMC_PNSR"]
    pub fmc_pnsr: FMC_PNSR,
    #[doc = "0x188 - FMC_PSSR"]
    pub fmc_pssr: FMC_PSSR,
    #[doc = "0x18c - FMC_DID"]
    pub fmc_did: FMC_DID,
    _reserved16: [u8; 0x70],
    #[doc = "0x200 - FMC_CFCR"]
    pub fmc_cfcr: FMC_CFCR,
    _reserved17: [u8; 0x010c],
    #[doc = "0x310 - FMC_CID0"]
    pub fmc_cid0: FMC_CID0,
    #[doc = "0x314 - FMC_CID1"]
    pub fmc_cid1: FMC_CID1,
    #[doc = "0x318 - FMC_CID2"]
    pub fmc_cid2: FMC_CID2,
    #[doc = "0x31c - FMC_CID3"]
    pub fmc_cid3: FMC_CID3,
}
#[doc = "FMC_TADR (rw) register accessor: an alias for `Reg<FMC_TADR_SPEC>`"]
pub type FMC_TADR = crate::Reg<fmc_tadr::FMC_TADR_SPEC>;
#[doc = "FMC_TADR"]
pub mod fmc_tadr;
#[doc = "FMC_WRDR (rw) register accessor: an alias for `Reg<FMC_WRDR_SPEC>`"]
pub type FMC_WRDR = crate::Reg<fmc_wrdr::FMC_WRDR_SPEC>;
#[doc = "FMC_WRDR"]
pub mod fmc_wrdr;
#[doc = "FMC_OCMR (rw) register accessor: an alias for `Reg<FMC_OCMR_SPEC>`"]
pub type FMC_OCMR = crate::Reg<fmc_ocmr::FMC_OCMR_SPEC>;
#[doc = "FMC_OCMR"]
pub mod fmc_ocmr;
#[doc = "FMC_OPCR (rw) register accessor: an alias for `Reg<FMC_OPCR_SPEC>`"]
pub type FMC_OPCR = crate::Reg<fmc_opcr::FMC_OPCR_SPEC>;
#[doc = "FMC_OPCR"]
pub mod fmc_opcr;
#[doc = "FMC_OIER (rw) register accessor: an alias for `Reg<FMC_OIER_SPEC>`"]
pub type FMC_OIER = crate::Reg<fmc_oier::FMC_OIER_SPEC>;
#[doc = "FMC_OIER"]
pub mod fmc_oier;
#[doc = "FMC_OISR (rw) register accessor: an alias for `Reg<FMC_OISR_SPEC>`"]
pub type FMC_OISR = crate::Reg<fmc_oisr::FMC_OISR_SPEC>;
#[doc = "FMC_OISR"]
pub mod fmc_oisr;
#[doc = "FMC_PPSR0 (rw) register accessor: an alias for `Reg<FMC_PPSR0_SPEC>`"]
pub type FMC_PPSR0 = crate::Reg<fmc_ppsr0::FMC_PPSR0_SPEC>;
#[doc = "FMC_PPSR0"]
pub mod fmc_ppsr0;
#[doc = "FMC_PPSR1 (rw) register accessor: an alias for `Reg<FMC_PPSR1_SPEC>`"]
pub type FMC_PPSR1 = crate::Reg<fmc_ppsr1::FMC_PPSR1_SPEC>;
#[doc = "FMC_PPSR1"]
pub mod fmc_ppsr1;
#[doc = "FMC_PPSR2 (rw) register accessor: an alias for `Reg<FMC_PPSR2_SPEC>`"]
pub type FMC_PPSR2 = crate::Reg<fmc_ppsr2::FMC_PPSR2_SPEC>;
#[doc = "FMC_PPSR2"]
pub mod fmc_ppsr2;
#[doc = "FMC_PPSR3 (rw) register accessor: an alias for `Reg<FMC_PPSR3_SPEC>`"]
pub type FMC_PPSR3 = crate::Reg<fmc_ppsr3::FMC_PPSR3_SPEC>;
#[doc = "FMC_PPSR3"]
pub mod fmc_ppsr3;
#[doc = "FMC_CPSR (rw) register accessor: an alias for `Reg<FMC_CPSR_SPEC>`"]
pub type FMC_CPSR = crate::Reg<fmc_cpsr::FMC_CPSR_SPEC>;
#[doc = "FMC_CPSR"]
pub mod fmc_cpsr;
#[doc = "FMC_VMCR (rw) register accessor: an alias for `Reg<FMC_VMCR_SPEC>`"]
pub type FMC_VMCR = crate::Reg<fmc_vmcr::FMC_VMCR_SPEC>;
#[doc = "FMC_VMCR"]
pub mod fmc_vmcr;
#[doc = "FMC_MDID (rw) register accessor: an alias for `Reg<FMC_MDID_SPEC>`"]
pub type FMC_MDID = crate::Reg<fmc_mdid::FMC_MDID_SPEC>;
#[doc = "FMC_MDID"]
pub mod fmc_mdid;
#[doc = "FMC_PNSR (rw) register accessor: an alias for `Reg<FMC_PNSR_SPEC>`"]
pub type FMC_PNSR = crate::Reg<fmc_pnsr::FMC_PNSR_SPEC>;
#[doc = "FMC_PNSR"]
pub mod fmc_pnsr;
#[doc = "FMC_PSSR (rw) register accessor: an alias for `Reg<FMC_PSSR_SPEC>`"]
pub type FMC_PSSR = crate::Reg<fmc_pssr::FMC_PSSR_SPEC>;
#[doc = "FMC_PSSR"]
pub mod fmc_pssr;
#[doc = "FMC_DID (rw) register accessor: an alias for `Reg<FMC_DID_SPEC>`"]
pub type FMC_DID = crate::Reg<fmc_did::FMC_DID_SPEC>;
#[doc = "FMC_DID"]
pub mod fmc_did;
#[doc = "FMC_CFCR (rw) register accessor: an alias for `Reg<FMC_CFCR_SPEC>`"]
pub type FMC_CFCR = crate::Reg<fmc_cfcr::FMC_CFCR_SPEC>;
#[doc = "FMC_CFCR"]
pub mod fmc_cfcr;
#[doc = "FMC_CID0 (rw) register accessor: an alias for `Reg<FMC_CID0_SPEC>`"]
pub type FMC_CID0 = crate::Reg<fmc_cid0::FMC_CID0_SPEC>;
#[doc = "FMC_CID0"]
pub mod fmc_cid0;
#[doc = "FMC_CID1 (rw) register accessor: an alias for `Reg<FMC_CID1_SPEC>`"]
pub type FMC_CID1 = crate::Reg<fmc_cid1::FMC_CID1_SPEC>;
#[doc = "FMC_CID1"]
pub mod fmc_cid1;
#[doc = "FMC_CID2 (rw) register accessor: an alias for `Reg<FMC_CID2_SPEC>`"]
pub type FMC_CID2 = crate::Reg<fmc_cid2::FMC_CID2_SPEC>;
#[doc = "FMC_CID2"]
pub mod fmc_cid2;
#[doc = "FMC_CID3 (rw) register accessor: an alias for `Reg<FMC_CID3_SPEC>`"]
pub type FMC_CID3 = crate::Reg<fmc_cid3::FMC_CID3_SPEC>;
#[doc = "FMC_CID3"]
pub mod fmc_cid3;
