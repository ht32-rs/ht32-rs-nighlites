#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - LCD_LCDCR"]
    pub lcd_lcdcr: LCD_LCDCR,
    #[doc = "0x04 - LCD_LCDFCR"]
    pub lcd_lcdfcr: LCD_LCDFCR,
    #[doc = "0x08 - LCD_LCDIER"]
    pub lcd_lcdier: LCD_LCDIER,
    #[doc = "0x0c - LCD_LCDSR"]
    pub lcd_lcdsr: LCD_LCDSR,
    #[doc = "0x10 - LCD_LCDSCR"]
    pub lcd_lcdscr: LCD_LCDSCR,
    _reserved5: [u8; 0x0c],
    #[doc = "0x20 - LCD_LCDRAM0"]
    pub lcd_lcdram0: LCD_LCDRAM0,
    #[doc = "0x24 - LCD_LCDRAM1"]
    pub lcd_lcdram1: LCD_LCDRAM1,
    #[doc = "0x28 - LCD_LCDRAM2"]
    pub lcd_lcdram2: LCD_LCDRAM2,
    #[doc = "0x2c - LCD_LCDRAM3"]
    pub lcd_lcdram3: LCD_LCDRAM3,
    #[doc = "0x30 - LCD_LCDRAM4"]
    pub lcd_lcdram4: LCD_LCDRAM4,
    #[doc = "0x34 - LCD_LCDRAM5"]
    pub lcd_lcdram5: LCD_LCDRAM5,
    #[doc = "0x38 - LCD_LCDRAM6"]
    pub lcd_lcdram6: LCD_LCDRAM6,
    #[doc = "0x3c - LCD_LCDRAM7"]
    pub lcd_lcdram7: LCD_LCDRAM7,
    #[doc = "0x40 - LCD_LCDRAM8"]
    pub lcd_lcdram8: LCD_LCDRAM8,
    #[doc = "0x44 - LCD_LCDRAM9"]
    pub lcd_lcdram9: LCD_LCDRAM9,
    #[doc = "0x48 - LCD_LCDRAM10"]
    pub lcd_lcdram10: LCD_LCDRAM10,
    #[doc = "0x4c - LCD_LCDRAM11"]
    pub lcd_lcdram11: LCD_LCDRAM11,
    #[doc = "0x50 - LCD_LCDRAM12"]
    pub lcd_lcdram12: LCD_LCDRAM12,
    #[doc = "0x54 - LCD_LCDRAM13"]
    pub lcd_lcdram13: LCD_LCDRAM13,
    #[doc = "0x58 - LCD_LCDRAM14"]
    pub lcd_lcdram14: LCD_LCDRAM14,
    #[doc = "0x5c - LCD_LCDRAM15"]
    pub lcd_lcdram15: LCD_LCDRAM15,
}
#[doc = "LCD_LCDCR (rw) register accessor: an alias for `Reg<LCD_LCDCR_SPEC>`"]
pub type LCD_LCDCR = crate::Reg<lcd_lcdcr::LCD_LCDCR_SPEC>;
#[doc = "LCD_LCDCR"]
pub mod lcd_lcdcr;
#[doc = "LCD_LCDFCR (rw) register accessor: an alias for `Reg<LCD_LCDFCR_SPEC>`"]
pub type LCD_LCDFCR = crate::Reg<lcd_lcdfcr::LCD_LCDFCR_SPEC>;
#[doc = "LCD_LCDFCR"]
pub mod lcd_lcdfcr;
#[doc = "LCD_LCDIER (rw) register accessor: an alias for `Reg<LCD_LCDIER_SPEC>`"]
pub type LCD_LCDIER = crate::Reg<lcd_lcdier::LCD_LCDIER_SPEC>;
#[doc = "LCD_LCDIER"]
pub mod lcd_lcdier;
#[doc = "LCD_LCDSR (rw) register accessor: an alias for `Reg<LCD_LCDSR_SPEC>`"]
pub type LCD_LCDSR = crate::Reg<lcd_lcdsr::LCD_LCDSR_SPEC>;
#[doc = "LCD_LCDSR"]
pub mod lcd_lcdsr;
#[doc = "LCD_LCDSCR (rw) register accessor: an alias for `Reg<LCD_LCDSCR_SPEC>`"]
pub type LCD_LCDSCR = crate::Reg<lcd_lcdscr::LCD_LCDSCR_SPEC>;
#[doc = "LCD_LCDSCR"]
pub mod lcd_lcdscr;
#[doc = "LCD_LCDRAM0 (rw) register accessor: an alias for `Reg<LCD_LCDRAM0_SPEC>`"]
pub type LCD_LCDRAM0 = crate::Reg<lcd_lcdram0::LCD_LCDRAM0_SPEC>;
#[doc = "LCD_LCDRAM0"]
pub mod lcd_lcdram0;
#[doc = "LCD_LCDRAM1 (rw) register accessor: an alias for `Reg<LCD_LCDRAM1_SPEC>`"]
pub type LCD_LCDRAM1 = crate::Reg<lcd_lcdram1::LCD_LCDRAM1_SPEC>;
#[doc = "LCD_LCDRAM1"]
pub mod lcd_lcdram1;
#[doc = "LCD_LCDRAM2 (rw) register accessor: an alias for `Reg<LCD_LCDRAM2_SPEC>`"]
pub type LCD_LCDRAM2 = crate::Reg<lcd_lcdram2::LCD_LCDRAM2_SPEC>;
#[doc = "LCD_LCDRAM2"]
pub mod lcd_lcdram2;
#[doc = "LCD_LCDRAM3 (rw) register accessor: an alias for `Reg<LCD_LCDRAM3_SPEC>`"]
pub type LCD_LCDRAM3 = crate::Reg<lcd_lcdram3::LCD_LCDRAM3_SPEC>;
#[doc = "LCD_LCDRAM3"]
pub mod lcd_lcdram3;
#[doc = "LCD_LCDRAM4 (rw) register accessor: an alias for `Reg<LCD_LCDRAM4_SPEC>`"]
pub type LCD_LCDRAM4 = crate::Reg<lcd_lcdram4::LCD_LCDRAM4_SPEC>;
#[doc = "LCD_LCDRAM4"]
pub mod lcd_lcdram4;
#[doc = "LCD_LCDRAM5 (rw) register accessor: an alias for `Reg<LCD_LCDRAM5_SPEC>`"]
pub type LCD_LCDRAM5 = crate::Reg<lcd_lcdram5::LCD_LCDRAM5_SPEC>;
#[doc = "LCD_LCDRAM5"]
pub mod lcd_lcdram5;
#[doc = "LCD_LCDRAM6 (rw) register accessor: an alias for `Reg<LCD_LCDRAM6_SPEC>`"]
pub type LCD_LCDRAM6 = crate::Reg<lcd_lcdram6::LCD_LCDRAM6_SPEC>;
#[doc = "LCD_LCDRAM6"]
pub mod lcd_lcdram6;
#[doc = "LCD_LCDRAM7 (rw) register accessor: an alias for `Reg<LCD_LCDRAM7_SPEC>`"]
pub type LCD_LCDRAM7 = crate::Reg<lcd_lcdram7::LCD_LCDRAM7_SPEC>;
#[doc = "LCD_LCDRAM7"]
pub mod lcd_lcdram7;
#[doc = "LCD_LCDRAM8 (rw) register accessor: an alias for `Reg<LCD_LCDRAM8_SPEC>`"]
pub type LCD_LCDRAM8 = crate::Reg<lcd_lcdram8::LCD_LCDRAM8_SPEC>;
#[doc = "LCD_LCDRAM8"]
pub mod lcd_lcdram8;
#[doc = "LCD_LCDRAM9 (rw) register accessor: an alias for `Reg<LCD_LCDRAM9_SPEC>`"]
pub type LCD_LCDRAM9 = crate::Reg<lcd_lcdram9::LCD_LCDRAM9_SPEC>;
#[doc = "LCD_LCDRAM9"]
pub mod lcd_lcdram9;
#[doc = "LCD_LCDRAM10 (rw) register accessor: an alias for `Reg<LCD_LCDRAM10_SPEC>`"]
pub type LCD_LCDRAM10 = crate::Reg<lcd_lcdram10::LCD_LCDRAM10_SPEC>;
#[doc = "LCD_LCDRAM10"]
pub mod lcd_lcdram10;
#[doc = "LCD_LCDRAM11 (rw) register accessor: an alias for `Reg<LCD_LCDRAM11_SPEC>`"]
pub type LCD_LCDRAM11 = crate::Reg<lcd_lcdram11::LCD_LCDRAM11_SPEC>;
#[doc = "LCD_LCDRAM11"]
pub mod lcd_lcdram11;
#[doc = "LCD_LCDRAM12 (rw) register accessor: an alias for `Reg<LCD_LCDRAM12_SPEC>`"]
pub type LCD_LCDRAM12 = crate::Reg<lcd_lcdram12::LCD_LCDRAM12_SPEC>;
#[doc = "LCD_LCDRAM12"]
pub mod lcd_lcdram12;
#[doc = "LCD_LCDRAM13 (rw) register accessor: an alias for `Reg<LCD_LCDRAM13_SPEC>`"]
pub type LCD_LCDRAM13 = crate::Reg<lcd_lcdram13::LCD_LCDRAM13_SPEC>;
#[doc = "LCD_LCDRAM13"]
pub mod lcd_lcdram13;
#[doc = "LCD_LCDRAM14 (rw) register accessor: an alias for `Reg<LCD_LCDRAM14_SPEC>`"]
pub type LCD_LCDRAM14 = crate::Reg<lcd_lcdram14::LCD_LCDRAM14_SPEC>;
#[doc = "LCD_LCDRAM14"]
pub mod lcd_lcdram14;
#[doc = "LCD_LCDRAM15 (rw) register accessor: an alias for `Reg<LCD_LCDRAM15_SPEC>`"]
pub type LCD_LCDRAM15 = crate::Reg<lcd_lcdram15::LCD_LCDRAM15_SPEC>;
#[doc = "LCD_LCDRAM15"]
pub mod lcd_lcdram15;
