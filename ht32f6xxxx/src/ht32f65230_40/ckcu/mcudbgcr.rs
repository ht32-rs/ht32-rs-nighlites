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
#[doc = "Field `DBDSLP` reader - DBDSLP"]
pub type DBDSLP_R = crate::BitReader;
#[doc = "Field `DBDSLP` writer - DBDSLP"]
pub type DBDSLP_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBWDT` reader - DBWDT"]
pub type DBWDT_R = crate::BitReader;
#[doc = "Field `DBWDT` writer - DBWDT"]
pub type DBWDT_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBMCTM` reader - DBMCTM"]
pub type DBMCTM_R = crate::BitReader;
#[doc = "Field `DBMCTM` writer - DBMCTM"]
pub type DBMCTM_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBGPTM` reader - DBGPTM"]
pub type DBGPTM_R = crate::BitReader;
#[doc = "Field `DBGPTM` writer - DBGPTM"]
pub type DBGPTM_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBUSR` reader - DBUSR"]
pub type DBUSR_R = crate::BitReader;
#[doc = "Field `DBUSR` writer - DBUSR"]
pub type DBUSR_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSPI` reader - DBSPI"]
pub type DBSPI_R = crate::BitReader;
#[doc = "Field `DBSPI` writer - DBSPI"]
pub type DBSPI_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBI2C` reader - DBI2C"]
pub type DBI2C_R = crate::BitReader;
#[doc = "Field `DBI2C` writer - DBI2C"]
pub type DBI2C_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBBFTM0` reader - DBBFTM0"]
pub type DBBFTM0_R = crate::BitReader;
#[doc = "Field `DBBFTM0` writer - DBBFTM0"]
pub type DBBFTM0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBBFTM1` reader - DBBFTM1"]
pub type DBBFTM1_R = crate::BitReader;
#[doc = "Field `DBBFTM1` writer - DBBFTM1"]
pub type DBBFTM1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBUR` reader - DBUR"]
pub type DBUR_R = crate::BitReader;
#[doc = "Field `DBUR` writer - DBUR"]
pub type DBUR_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSCTM0` reader - DBSCTM0"]
pub type DBSCTM0_R = crate::BitReader;
#[doc = "Field `DBSCTM0` writer - DBSCTM0"]
pub type DBSCTM0_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSCTM1` reader - DBSCTM1"]
pub type DBSCTM1_R = crate::BitReader;
#[doc = "Field `DBSCTM1` writer - DBSCTM1"]
pub type DBSCTM1_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSCTM2` reader - DBSCTM2"]
pub type DBSCTM2_R = crate::BitReader;
#[doc = "Field `DBSCTM2` writer - DBSCTM2"]
pub type DBSCTM2_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
#[doc = "Field `DBSCTM3` reader - DBSCTM3"]
pub type DBSCTM3_R = crate::BitReader;
#[doc = "Field `DBSCTM3` writer - DBSCTM3"]
pub type DBSCTM3_W<'a, const O: u8> = crate::BitWriter<'a, MCUDBGCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&self) -> DBSLP_R {
        DBSLP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBDSLP"]
    #[inline(always)]
    pub fn dbdslp(&self) -> DBDSLP_R {
        DBDSLP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&self) -> DBWDT_R {
        DBWDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DBMCTM"]
    #[inline(always)]
    pub fn dbmctm(&self) -> DBMCTM_R {
        DBMCTM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DBGPTM"]
    #[inline(always)]
    pub fn dbgptm(&self) -> DBGPTM_R {
        DBGPTM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - DBUSR"]
    #[inline(always)]
    pub fn dbusr(&self) -> DBUSR_R {
        DBUSR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - DBSPI"]
    #[inline(always)]
    pub fn dbspi(&self) -> DBSPI_R {
        DBSPI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - DBI2C"]
    #[inline(always)]
    pub fn dbi2c(&self) -> DBI2C_R {
        DBI2C_R::new(((self.bits >> 12) & 1) != 0)
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
    #[doc = "Bit 18 - DBUR"]
    #[inline(always)]
    pub fn dbur(&self) -> DBUR_R {
        DBUR_R::new(((self.bits >> 18) & 1) != 0)
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
    #[doc = "Bit 24 - DBSCTM2"]
    #[inline(always)]
    pub fn dbsctm2(&self) -> DBSCTM2_R {
        DBSCTM2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DBSCTM3"]
    #[inline(always)]
    pub fn dbsctm3(&self) -> DBSCTM3_R {
        DBSCTM3_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    #[must_use]
    pub fn dbslp(&mut self) -> DBSLP_W<0> {
        DBSLP_W::new(self)
    }
    #[doc = "Bit 1 - DBDSLP"]
    #[inline(always)]
    #[must_use]
    pub fn dbdslp(&mut self) -> DBDSLP_W<1> {
        DBDSLP_W::new(self)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    #[must_use]
    pub fn dbwdt(&mut self) -> DBWDT_W<3> {
        DBWDT_W::new(self)
    }
    #[doc = "Bit 4 - DBMCTM"]
    #[inline(always)]
    #[must_use]
    pub fn dbmctm(&mut self) -> DBMCTM_W<4> {
        DBMCTM_W::new(self)
    }
    #[doc = "Bit 6 - DBGPTM"]
    #[inline(always)]
    #[must_use]
    pub fn dbgptm(&mut self) -> DBGPTM_W<6> {
        DBGPTM_W::new(self)
    }
    #[doc = "Bit 8 - DBUSR"]
    #[inline(always)]
    #[must_use]
    pub fn dbusr(&mut self) -> DBUSR_W<8> {
        DBUSR_W::new(self)
    }
    #[doc = "Bit 10 - DBSPI"]
    #[inline(always)]
    #[must_use]
    pub fn dbspi(&mut self) -> DBSPI_W<10> {
        DBSPI_W::new(self)
    }
    #[doc = "Bit 12 - DBI2C"]
    #[inline(always)]
    #[must_use]
    pub fn dbi2c(&mut self) -> DBI2C_W<12> {
        DBI2C_W::new(self)
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
    #[doc = "Bit 18 - DBUR"]
    #[inline(always)]
    #[must_use]
    pub fn dbur(&mut self) -> DBUR_W<18> {
        DBUR_W::new(self)
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
    #[doc = "Bit 24 - DBSCTM2"]
    #[inline(always)]
    #[must_use]
    pub fn dbsctm2(&mut self) -> DBSCTM2_W<24> {
        DBSCTM2_W::new(self)
    }
    #[doc = "Bit 25 - DBSCTM3"]
    #[inline(always)]
    #[must_use]
    pub fn dbsctm3(&mut self) -> DBSCTM3_W<25> {
        DBSCTM3_W::new(self)
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
