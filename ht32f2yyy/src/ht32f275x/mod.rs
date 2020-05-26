#![doc = "Peripheral access API for HT32F275X microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn CKRDY();
    fn LVD();
    fn BOD();
    fn WDT();
    fn RTC();
    fn FMC();
    fn EVWUP();
    fn LPWUP();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn EXTI5();
    fn EXTI6();
    fn EXTI7();
    fn EXTI8();
    fn EXTI9();
    fn EXTI10();
    fn EXTI11();
    fn EXTI12();
    fn EXTI13();
    fn EXTI14();
    fn EXTI15();
    fn COMP();
    fn ADC();
    fn MCTMBRK();
    fn MCTMUP();
    fn MCTMTR();
    fn MCTMCC();
    fn GPTM0();
    fn GPTM1();
    fn BFTM0();
    fn BFTN1();
    fn I2C0();
    fn I2C1();
    fn SPI0();
    fn SPI1();
    fn USART0();
    fn USART1();
    fn SCI();
    fn USB();
    fn PDMACH0();
    fn PDMACH1();
    fn PDMACH2();
    fn PDMACH3();
    fn PDMACH4();
    fn PDMACH5();
    fn PDMACH6();
    fn PDMACH7();
    fn PDMACH8();
    fn PDMACH9();
    fn PDMACH10();
    fn PDMACH11();
    fn CSIF();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 68] = [
    Vector { _handler: CKRDY },
    Vector { _handler: LVD },
    Vector { _handler: BOD },
    Vector { _handler: WDT },
    Vector { _handler: RTC },
    Vector { _handler: FMC },
    Vector { _handler: EVWUP },
    Vector { _handler: LPWUP },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector { _handler: EXTI5 },
    Vector { _handler: EXTI6 },
    Vector { _handler: EXTI7 },
    Vector { _handler: EXTI8 },
    Vector { _handler: EXTI9 },
    Vector { _handler: EXTI10 },
    Vector { _handler: EXTI11 },
    Vector { _handler: EXTI12 },
    Vector { _handler: EXTI13 },
    Vector { _handler: EXTI14 },
    Vector { _handler: EXTI15 },
    Vector { _handler: COMP },
    Vector { _handler: ADC },
    Vector { _reserved: 0 },
    Vector { _handler: MCTMBRK },
    Vector { _handler: MCTMUP },
    Vector { _handler: MCTMTR },
    Vector { _handler: MCTMCC },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: GPTM0 },
    Vector { _handler: GPTM1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: BFTM0 },
    Vector { _handler: BFTN1 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: SPI0 },
    Vector { _handler: SPI1 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: SCI },
    Vector { _reserved: 0 },
    Vector { _handler: USB },
    Vector { _reserved: 0 },
    Vector { _handler: PDMACH0 },
    Vector { _handler: PDMACH1 },
    Vector { _handler: PDMACH2 },
    Vector { _handler: PDMACH3 },
    Vector { _handler: PDMACH4 },
    Vector { _handler: PDMACH5 },
    Vector { _handler: PDMACH6 },
    Vector { _handler: PDMACH7 },
    Vector { _handler: PDMACH8 },
    Vector { _handler: PDMACH9 },
    Vector { _handler: PDMACH10 },
    Vector { _handler: PDMACH11 },
    Vector { _handler: CSIF },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "0 - CKRDY"]
    CKRDY = 0,
    #[doc = "1 - LVD"]
    LVD = 1,
    #[doc = "2 - BOD"]
    BOD = 2,
    #[doc = "3 - WDT"]
    WDT = 3,
    #[doc = "4 - RTC"]
    RTC = 4,
    #[doc = "5 - FMC"]
    FMC = 5,
    #[doc = "6 - EVWUP"]
    EVWUP = 6,
    #[doc = "7 - LPWUP"]
    LPWUP = 7,
    #[doc = "8 - EXTI0"]
    EXTI0 = 8,
    #[doc = "9 - EXTI1"]
    EXTI1 = 9,
    #[doc = "10 - EXTI2"]
    EXTI2 = 10,
    #[doc = "11 - EXTI3"]
    EXTI3 = 11,
    #[doc = "12 - EXTI4"]
    EXTI4 = 12,
    #[doc = "13 - EXTI5"]
    EXTI5 = 13,
    #[doc = "14 - EXTI6"]
    EXTI6 = 14,
    #[doc = "15 - EXTI7"]
    EXTI7 = 15,
    #[doc = "16 - EXTI8"]
    EXTI8 = 16,
    #[doc = "17 - EXTI9"]
    EXTI9 = 17,
    #[doc = "18 - EXTI10"]
    EXTI10 = 18,
    #[doc = "19 - EXTI11"]
    EXTI11 = 19,
    #[doc = "20 - EXTI12"]
    EXTI12 = 20,
    #[doc = "21 - EXTI13"]
    EXTI13 = 21,
    #[doc = "22 - EXTI14"]
    EXTI14 = 22,
    #[doc = "23 - EXTI15"]
    EXTI15 = 23,
    #[doc = "24 - COMP"]
    COMP = 24,
    #[doc = "25 - ADC"]
    ADC = 25,
    #[doc = "27 - MCTMBRK"]
    MCTMBRK = 27,
    #[doc = "28 - MCTMUP"]
    MCTMUP = 28,
    #[doc = "29 - MCTMTR"]
    MCTMTR = 29,
    #[doc = "30 - MCTMCC"]
    MCTMCC = 30,
    #[doc = "35 - GPTM0"]
    GPTM0 = 35,
    #[doc = "36 - GPTM1"]
    GPTM1 = 36,
    #[doc = "41 - BFTM0"]
    BFTM0 = 41,
    #[doc = "42 - BFTN1"]
    BFTN1 = 42,
    #[doc = "43 - I2C0"]
    I2C0 = 43,
    #[doc = "44 - I2C1"]
    I2C1 = 44,
    #[doc = "45 - SPI0"]
    SPI0 = 45,
    #[doc = "46 - SPI1"]
    SPI1 = 46,
    #[doc = "47 - USART0"]
    USART0 = 47,
    #[doc = "48 - USART1"]
    USART1 = 48,
    #[doc = "51 - SCI"]
    SCI = 51,
    #[doc = "53 - USB"]
    USB = 53,
    #[doc = "55 - PDMACH0"]
    PDMACH0 = 55,
    #[doc = "56 - PDMACH1"]
    PDMACH1 = 56,
    #[doc = "57 - PDMACH2"]
    PDMACH2 = 57,
    #[doc = "58 - PDMACH3"]
    PDMACH3 = 58,
    #[doc = "59 - PDMACH4"]
    PDMACH4 = 59,
    #[doc = "60 - PDMACH5"]
    PDMACH5 = 60,
    #[doc = "61 - PDMACH6"]
    PDMACH6 = 61,
    #[doc = "62 - PDMACH7"]
    PDMACH7 = 62,
    #[doc = "63 - PDMACH8"]
    PDMACH8 = 63,
    #[doc = "64 - PDMACH9"]
    PDMACH9 = 64,
    #[doc = "65 - PDMACH10"]
    PDMACH10 = 65,
    #[doc = "66 - PDMACH11"]
    PDMACH11 = 66,
    #[doc = "67 - CSIF"]
    CSIF = 67,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "SysTick"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "SysTick"]
pub mod sys_tick;
#[doc = "Fault_Reports"]
pub struct FAULT_REPORTS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FAULT_REPORTS {}
impl FAULT_REPORTS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fault_reports::RegisterBlock {
        0xe000_ed28 as *const _
    }
}
impl Deref for FAULT_REPORTS {
    type Target = fault_reports::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FAULT_REPORTS::ptr() }
    }
}
#[doc = "Fault_Reports"]
pub mod fault_reports;
#[doc = "FMC"]
pub struct FMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FMC {}
impl FMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fmc::RegisterBlock {
        0x4008_0000 as *const _
    }
}
impl Deref for FMC {
    type Target = fmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FMC::ptr() }
    }
}
#[doc = "FMC"]
pub mod fmc;
#[doc = "PWRCU"]
pub struct PWRCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWRCU {}
impl PWRCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwrcu::RegisterBlock {
        0x4006_a100 as *const _
    }
}
impl Deref for PWRCU {
    type Target = pwrcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWRCU::ptr() }
    }
}
#[doc = "PWRCU"]
pub mod pwrcu;
#[doc = "CKCU"]
pub struct CKCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CKCU {}
impl CKCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ckcu::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for CKCU {
    type Target = ckcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CKCU::ptr() }
    }
}
#[doc = "CKCU"]
pub mod ckcu;
#[doc = "RSTCU"]
pub struct RSTCU {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTCU {}
impl RSTCU {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstcu::RegisterBlock {
        0x4008_8100 as *const _
    }
}
impl Deref for RSTCU {
    type Target = rstcu::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTCU::ptr() }
    }
}
#[doc = "RSTCU"]
pub mod rstcu;
#[doc = "GPIOA"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x4001_a000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "GPIOA"]
pub mod gpioa;
#[doc = "GPIOB"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x4001_b000 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "GPIOB"]
pub mod gpiob;
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioc::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOC"]
pub mod gpioc;
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiod::RegisterBlock {
        0x4001_d000 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpiod::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOD"]
pub mod gpiod;
#[doc = "GPIOE"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioe::RegisterBlock {
        0x4001_e000 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioe::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOE"]
pub mod gpioe;
#[doc = "AFIO"]
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFIO {}
impl AFIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afio::RegisterBlock {
        0x4002_2000 as *const _
    }
}
impl Deref for AFIO {
    type Target = afio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFIO::ptr() }
    }
}
#[doc = "AFIO"]
pub mod afio;
#[doc = "EXTI"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const exti::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "EXTI"]
pub mod exti;
#[doc = "ADC"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "ADC"]
pub mod adc;
#[doc = "OPACMP"]
pub struct OPACMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OPACMP {}
impl OPACMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const opacmp::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for OPACMP {
    type Target = opacmp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OPACMP::ptr() }
    }
}
#[doc = "OPACMP"]
pub mod opacmp;
#[doc = "MCTM"]
pub struct MCTM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCTM {}
impl MCTM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mctm::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for MCTM {
    type Target = mctm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCTM::ptr() }
    }
}
#[doc = "MCTM"]
pub mod mctm;
#[doc = "GPTM0"]
pub struct GPTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTM0 {}
impl GPTM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptm0::RegisterBlock {
        0x4006_e000 as *const _
    }
}
impl Deref for GPTM0 {
    type Target = gptm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTM0::ptr() }
    }
}
#[doc = "GPTM0"]
pub mod gptm0;
#[doc = "GPTM1"]
pub struct GPTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPTM1 {}
impl GPTM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gptm0::RegisterBlock {
        0x4006_f000 as *const _
    }
}
impl Deref for GPTM1 {
    type Target = gptm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPTM1::ptr() }
    }
}
#[doc = "BFTM0"]
pub struct BFTM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BFTM0 {}
impl BFTM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bftm0::RegisterBlock {
        0x4007_6000 as *const _
    }
}
impl Deref for BFTM0 {
    type Target = bftm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BFTM0::ptr() }
    }
}
#[doc = "BFTM0"]
pub mod bftm0;
#[doc = "BFTM1"]
pub struct BFTM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BFTM1 {}
impl BFTM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const bftm0::RegisterBlock {
        0x4007_7000 as *const _
    }
}
impl Deref for BFTM1 {
    type Target = bftm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*BFTM1::ptr() }
    }
}
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4006_a000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "WDT"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4006_8000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "WDT"]
pub mod wdt;
#[doc = "I2C0"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C0"]
pub mod i2c0;
#[doc = "I2C1"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4004_9000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "SPI0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "SPI0"]
pub mod spi0;
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "USART0"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "USART0"]
pub mod usart0;
#[doc = "USART1"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart1::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "USART1"]
pub mod usart1;
#[doc = "SCI"]
pub struct SCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCI {}
impl SCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sci::RegisterBlock {
        0x4004_3000 as *const _
    }
}
impl Deref for SCI {
    type Target = sci::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCI::ptr() }
    }
}
#[doc = "SCI"]
pub mod sci;
#[doc = "USB"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x4004_e000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB"]
pub mod usb;
#[doc = "PDMA"]
pub struct PDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PDMA {}
impl PDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pdma::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for PDMA {
    type Target = pdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PDMA::ptr() }
    }
}
#[doc = "PDMA"]
pub mod pdma;
#[doc = "CSIF"]
pub struct CSIF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CSIF {}
impl CSIF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const csif::RegisterBlock {
        0x400c_c000 as *const _
    }
}
impl Deref for CSIF {
    type Target = csif::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CSIF::ptr() }
    }
}
#[doc = "CSIF"]
pub mod csif;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "FAULT_REPORTS"]
    pub FAULT_REPORTS: FAULT_REPORTS,
    #[doc = "FMC"]
    pub FMC: FMC,
    #[doc = "PWRCU"]
    pub PWRCU: PWRCU,
    #[doc = "CKCU"]
    pub CKCU: CKCU,
    #[doc = "RSTCU"]
    pub RSTCU: RSTCU,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "AFIO"]
    pub AFIO: AFIO,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "OPACMP"]
    pub OPACMP: OPACMP,
    #[doc = "MCTM"]
    pub MCTM: MCTM,
    #[doc = "GPTM0"]
    pub GPTM0: GPTM0,
    #[doc = "GPTM1"]
    pub GPTM1: GPTM1,
    #[doc = "BFTM0"]
    pub BFTM0: BFTM0,
    #[doc = "BFTM1"]
    pub BFTM1: BFTM1,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "SCI"]
    pub SCI: SCI,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "PDMA"]
    pub PDMA: PDMA,
    #[doc = "CSIF"]
    pub CSIF: CSIF,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            FAULT_REPORTS: FAULT_REPORTS {
                _marker: PhantomData,
            },
            FMC: FMC {
                _marker: PhantomData,
            },
            PWRCU: PWRCU {
                _marker: PhantomData,
            },
            CKCU: CKCU {
                _marker: PhantomData,
            },
            RSTCU: RSTCU {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            AFIO: AFIO {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            OPACMP: OPACMP {
                _marker: PhantomData,
            },
            MCTM: MCTM {
                _marker: PhantomData,
            },
            GPTM0: GPTM0 {
                _marker: PhantomData,
            },
            GPTM1: GPTM1 {
                _marker: PhantomData,
            },
            BFTM0: BFTM0 {
                _marker: PhantomData,
            },
            BFTM1: BFTM1 {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            SCI: SCI {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            PDMA: PDMA {
                _marker: PhantomData,
            },
            CSIF: CSIF {
                _marker: PhantomData,
            },
        }
    }
}
