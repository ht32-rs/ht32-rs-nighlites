#[doc = "Register `CRC_CR` reader"]
pub struct R(crate::R<CRC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC_CR` writer"]
pub struct W(crate::W<CRC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_CR_SPEC>;
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
impl From<crate::W<CRC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POLY` reader - POLY"]
pub type POLY_R = crate::FieldReader;
#[doc = "Field `POLY` writer - POLY"]
pub type POLY_W<'a, const O: u8> = crate::FieldWriter<'a, CRC_CR_SPEC, 2, O>;
#[doc = "Field `DATBIRV` reader - DATBIRV"]
pub type DATBIRV_R = crate::BitReader;
#[doc = "Field `DATBIRV` writer - DATBIRV"]
pub type DATBIRV_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CR_SPEC, O>;
#[doc = "Field `DATBYRV` reader - DATBYRV"]
pub type DATBYRV_R = crate::BitReader;
#[doc = "Field `DATBYRV` writer - DATBYRV"]
pub type DATBYRV_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CR_SPEC, O>;
#[doc = "Field `DATCMPL` reader - DATCMPL"]
pub type DATCMPL_R = crate::BitReader;
#[doc = "Field `DATCMPL` writer - DATCMPL"]
pub type DATCMPL_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CR_SPEC, O>;
#[doc = "Field `SUMBIRV` reader - SUMBIRV"]
pub type SUMBIRV_R = crate::BitReader;
#[doc = "Field `SUMBIRV` writer - SUMBIRV"]
pub type SUMBIRV_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CR_SPEC, O>;
#[doc = "Field `SUMBYRV` reader - SUMBYRV"]
pub type SUMBYRV_R = crate::BitReader;
#[doc = "Field `SUMBYRV` writer - SUMBYRV"]
pub type SUMBYRV_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CR_SPEC, O>;
#[doc = "Field `SUMCMPL` reader - SUMCMPL"]
pub type SUMCMPL_R = crate::BitReader;
#[doc = "Field `SUMCMPL` writer - SUMCMPL"]
pub type SUMCMPL_W<'a, const O: u8> = crate::BitWriter<'a, CRC_CR_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - POLY"]
    #[inline(always)]
    pub fn poly(&self) -> POLY_R {
        POLY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DATBIRV"]
    #[inline(always)]
    pub fn datbirv(&self) -> DATBIRV_R {
        DATBIRV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DATBYRV"]
    #[inline(always)]
    pub fn datbyrv(&self) -> DATBYRV_R {
        DATBYRV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DATCMPL"]
    #[inline(always)]
    pub fn datcmpl(&self) -> DATCMPL_R {
        DATCMPL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SUMBIRV"]
    #[inline(always)]
    pub fn sumbirv(&self) -> SUMBIRV_R {
        SUMBIRV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SUMBYRV"]
    #[inline(always)]
    pub fn sumbyrv(&self) -> SUMBYRV_R {
        SUMBYRV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SUMCMPL"]
    #[inline(always)]
    pub fn sumcmpl(&self) -> SUMCMPL_R {
        SUMCMPL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - POLY"]
    #[inline(always)]
    #[must_use]
    pub fn poly(&mut self) -> POLY_W<0> {
        POLY_W::new(self)
    }
    #[doc = "Bit 2 - DATBIRV"]
    #[inline(always)]
    #[must_use]
    pub fn datbirv(&mut self) -> DATBIRV_W<2> {
        DATBIRV_W::new(self)
    }
    #[doc = "Bit 3 - DATBYRV"]
    #[inline(always)]
    #[must_use]
    pub fn datbyrv(&mut self) -> DATBYRV_W<3> {
        DATBYRV_W::new(self)
    }
    #[doc = "Bit 4 - DATCMPL"]
    #[inline(always)]
    #[must_use]
    pub fn datcmpl(&mut self) -> DATCMPL_W<4> {
        DATCMPL_W::new(self)
    }
    #[doc = "Bit 5 - SUMBIRV"]
    #[inline(always)]
    #[must_use]
    pub fn sumbirv(&mut self) -> SUMBIRV_W<5> {
        SUMBIRV_W::new(self)
    }
    #[doc = "Bit 6 - SUMBYRV"]
    #[inline(always)]
    #[must_use]
    pub fn sumbyrv(&mut self) -> SUMBYRV_W<6> {
        SUMBYRV_W::new(self)
    }
    #[doc = "Bit 7 - SUMCMPL"]
    #[inline(always)]
    #[must_use]
    pub fn sumcmpl(&mut self) -> SUMCMPL_W<7> {
        SUMCMPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_cr](index.html) module"]
pub struct CRC_CR_SPEC;
impl crate::RegisterSpec for CRC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc_cr::R](R) reader structure"]
impl crate::Readable for CRC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_cr::W](W) writer structure"]
impl crate::Writable for CRC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRC_CR to value 0"]
impl crate::Resettable for CRC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
