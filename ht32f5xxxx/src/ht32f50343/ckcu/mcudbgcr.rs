#[doc = "Register `MCUDBGCR` reader"]
pub struct R(crate::R<MCUDBGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCUDBGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCUDBGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCUDBGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCUDBGCR` writer"]
pub struct W(crate::W<MCUDBGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCUDBGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCUDBGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCUDBGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBSLP` reader - DBSLP"]
pub type DBSLP_R = crate::BitReader;
#[doc = "Field `DBSLP` writer - DBSLP"]
pub type DBSLP_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBDSLP1` reader - DBDSLP1"]
pub type DBDSLP1_R = crate::BitReader;
#[doc = "Field `DBDSLP1` writer - DBDSLP1"]
pub type DBDSLP1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBWDT` reader - DBWDT"]
pub type DBWDT_R = crate::BitReader;
#[doc = "Field `DBWDT` writer - DBWDT"]
pub type DBWDT_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBGPTM` reader - DBGPTM"]
pub type DBGPTM_R = crate::BitReader;
#[doc = "Field `DBGPTM` writer - DBGPTM"]
pub type DBGPTM_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSPI0` reader - DBSPI0"]
pub type DBSPI0_R = crate::BitReader;
#[doc = "Field `DBSPI0` writer - DBSPI0"]
pub type DBSPI0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSPI1` reader - DBSPI1"]
pub type DBSPI1_R = crate::BitReader;
#[doc = "Field `DBSPI1` writer - DBSPI1"]
pub type DBSPI1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBI2C0` reader - DBI2C0"]
pub type DBI2C0_R = crate::BitReader;
#[doc = "Field `DBI2C0` writer - DBI2C0"]
pub type DBI2C0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBI2C1` reader - DBI2C1"]
pub type DBI2C1_R = crate::BitReader;
#[doc = "Field `DBI2C1` writer - DBI2C1"]
pub type DBI2C1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBDSLP2` reader - DBDSLP2"]
pub type DBDSLP2_R = crate::BitReader;
#[doc = "Field `DBDSLP2` writer - DBDSLP2"]
pub type DBDSLP2_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBBFTM0` reader - DBBFTM0"]
pub type DBBFTM0_R = crate::BitReader;
#[doc = "Field `DBBFTM0` writer - DBBFTM0"]
pub type DBBFTM0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBBFTM1` reader - DBBFTM1"]
pub type DBBFTM1_R = crate::BitReader;
#[doc = "Field `DBBFTM1` writer - DBBFTM1"]
pub type DBBFTM1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBUR0` reader - DBUR0"]
pub type DBUR0_R = crate::BitReader;
#[doc = "Field `DBUR0` writer - DBUR0"]
pub type DBUR0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBUR1` reader - DBUR1"]
pub type DBUR1_R = crate::BitReader;
#[doc = "Field `DBUR1` writer - DBUR1"]
pub type DBUR1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSCTM0` reader - DBSCTM0"]
pub type DBSCTM0_R = crate::BitReader;
#[doc = "Field `DBSCTM0` writer - DBSCTM0"]
pub type DBSCTM0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSCTM1` reader - DBSCTM1"]
pub type DBSCTM1_R = crate::BitReader;
#[doc = "Field `DBSCTM1` writer - DBSCTM1"]
pub type DBSCTM1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBPWM2` reader - DBPWM2"]
pub type DBPWM2_R = crate::BitReader;
#[doc = "Field `DBPWM2` writer - DBPWM2"]
pub type DBPWM2_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBPWM0` reader - DBPWM0"]
pub type DBPWM0_R = crate::BitReader;
#[doc = "Field `DBPWM0` writer - DBPWM0"]
pub type DBPWM0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBPWM1` reader - DBPWM1"]
pub type DBPWM1_R = crate::BitReader;
#[doc = "Field `DBPWM1` writer - DBPWM1"]
pub type DBPWM1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&self) -> DBSLP_R {
        DBSLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&self) -> DBDSLP1_R {
        DBDSLP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&self) -> DBWDT_R {
        DBWDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DBGPTM"]
    #[inline(always)]
    pub fn dbgptm(&self) -> DBGPTM_R {
        DBGPTM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    pub fn dbspi0(&self) -> DBSPI0_R {
        DBSPI0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    pub fn dbspi1(&self) -> DBSPI1_R {
        DBSPI1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    pub fn dbi2c0(&self) -> DBI2C0_R {
        DBI2C0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    pub fn dbi2c1(&self) -> DBI2C1_R {
        DBI2C1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&self) -> DBDSLP2_R {
        DBDSLP2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    pub fn dbbftm0(&self) -> DBBFTM0_R {
        DBBFTM0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    pub fn dbbftm1(&self) -> DBBFTM1_R {
        DBBFTM1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DBUR0"]
    #[inline(always)]
    pub fn dbur0(&self) -> DBUR0_R {
        DBUR0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DBUR1"]
    #[inline(always)]
    pub fn dbur1(&self) -> DBUR1_R {
        DBUR1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - DBSCTM0"]
    #[inline(always)]
    pub fn dbsctm0(&self) -> DBSCTM0_R {
        DBSCTM0_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DBSCTM1"]
    #[inline(always)]
    pub fn dbsctm1(&self) -> DBSCTM1_R {
        DBSCTM1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 29 - DBPWM2"]
    #[inline(always)]
    pub fn dbpwm2(&self) -> DBPWM2_R {
        DBPWM2_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DBPWM0"]
    #[inline(always)]
    pub fn dbpwm0(&self) -> DBPWM0_R {
        DBPWM0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DBPWM1"]
    #[inline(always)]
    pub fn dbpwm1(&self) -> DBPWM1_R {
        DBPWM1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    #[must_use]
    pub fn dbslp(&mut self) -> DBSLP_W<0> {
        DBSLP_W::new(self)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    #[must_use]
    pub fn dbdslp1(&mut self) -> DBDSLP1_W<1> {
        DBDSLP1_W::new(self)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    #[must_use]
    pub fn dbwdt(&mut self) -> DBWDT_W<3> {
        DBWDT_W::new(self)
    }
    #[doc = "Bit 6 - DBGPTM"]
    #[inline(always)]
    #[must_use]
    pub fn dbgptm(&mut self) -> DBGPTM_W<6> {
        DBGPTM_W::new(self)
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    #[must_use]
    pub fn dbspi0(&mut self) -> DBSPI0_W<10> {
        DBSPI0_W::new(self)
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    #[must_use]
    pub fn dbspi1(&mut self) -> DBSPI1_W<11> {
        DBSPI1_W::new(self)
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    #[must_use]
    pub fn dbi2c0(&mut self) -> DBI2C0_W<12> {
        DBI2C0_W::new(self)
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    #[must_use]
    pub fn dbi2c1(&mut self) -> DBI2C1_W<13> {
        DBI2C1_W::new(self)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    #[must_use]
    pub fn dbdslp2(&mut self) -> DBDSLP2_W<14> {
        DBDSLP2_W::new(self)
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbbftm0(&mut self) -> DBBFTM0_W<16> {
        DBBFTM0_W::new(self)
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    #[must_use]
    pub fn dbbftm1(&mut self) -> DBBFTM1_W<17> {
        DBBFTM1_W::new(self)
    }
    #[doc = "Bit 18 - DBUR0"]
    #[inline(always)]
    #[must_use]
    pub fn dbur0(&mut self) -> DBUR0_W<18> {
        DBUR0_W::new(self)
    }
    #[doc = "Bit 19 - DBUR1"]
    #[inline(always)]
    #[must_use]
    pub fn dbur1(&mut self) -> DBUR1_W<19> {
        DBUR1_W::new(self)
    }
    #[doc = "Bit 22 - DBSCTM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbsctm0(&mut self) -> DBSCTM0_W<22> {
        DBSCTM0_W::new(self)
    }
    #[doc = "Bit 23 - DBSCTM1"]
    #[inline(always)]
    #[must_use]
    pub fn dbsctm1(&mut self) -> DBSCTM1_W<23> {
        DBSCTM1_W::new(self)
    }
    #[doc = "Bit 29 - DBPWM2"]
    #[inline(always)]
    #[must_use]
    pub fn dbpwm2(&mut self) -> DBPWM2_W<29> {
        DBPWM2_W::new(self)
    }
    #[doc = "Bit 30 - DBPWM0"]
    #[inline(always)]
    #[must_use]
    pub fn dbpwm0(&mut self) -> DBPWM0_W<30> {
        DBPWM0_W::new(self)
    }
    #[doc = "Bit 31 - DBPWM1"]
    #[inline(always)]
    #[must_use]
    pub fn dbpwm1(&mut self) -> DBPWM1_W<31> {
        DBPWM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCUDBGCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcudbgcr](index.html) module"]
pub struct MCUDBGCR_SPEC;
impl crate::RegisterSpec for MCUDBGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcudbgcr::R](R) reader structure"]
impl crate::Readable for MCUDBGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcudbgcr::W](W) writer structure"]
impl crate::Writable for MCUDBGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCUDBGCR to value 0"]
impl crate::Resettable for MCUDBGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
