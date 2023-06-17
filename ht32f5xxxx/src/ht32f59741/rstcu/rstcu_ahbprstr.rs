#[doc = "Register `RSTCU_AHBPRSTR` reader"]
pub struct R(crate::R<RSTCU_AHBPRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCU_AHBPRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCU_AHBPRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCU_AHBPRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCU_AHBPRSTR` writer"]
pub struct W(crate::W<RSTCU_AHBPRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCU_AHBPRSTR_SPEC>;
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
impl From<crate::W<RSTCU_AHBPRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCU_AHBPRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMARST` reader - DMARST"]
pub type DMARST_R = crate::BitReader;
#[doc = "Field `DMARST` writer - DMARST"]
pub type DMARST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `USBRST` reader - USBRST"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USBRST"]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `CRCRST` reader - CRCRST"]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRCRST"]
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `PARST` reader - PARST"]
pub type PARST_R = crate::BitReader;
#[doc = "Field `PARST` writer - PARST"]
pub type PARST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `PBRST` reader - PBRST"]
pub type PBRST_R = crate::BitReader;
#[doc = "Field `PBRST` writer - PBRST"]
pub type PBRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `PCRST` reader - PCRST"]
pub type PCRST_R = crate::BitReader;
#[doc = "Field `PCRST` writer - PCRST"]
pub type PCRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `PDRST` reader - PDRST"]
pub type PDRST_R = crate::BitReader;
#[doc = "Field `PDRST` writer - PDRST"]
pub type PDRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `PERST` reader - PDRST"]
pub type PERST_R = crate::BitReader;
#[doc = "Field `PERST` writer - PDRST"]
pub type PERST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `AESRST` reader - AESRST"]
pub type AESRST_R = crate::BitReader;
#[doc = "Field `AESRST` writer - AESRST"]
pub type AESRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
#[doc = "Field `DIVRST` reader - DIVRST"]
pub type DIVRST_R = crate::BitReader;
#[doc = "Field `DIVRST` writer - DIVRST"]
pub type DIVRST_W<'a, const O: u8> = crate::BitWriter<'a, RSTCU_AHBPRSTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    pub fn dmarst(&self) -> DMARST_R {
        DMARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - USBRST"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CRCRST"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PARST"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDRST"]
    #[inline(always)]
    pub fn perst(&self) -> PERST_R {
        PERST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - AESRST"]
    #[inline(always)]
    pub fn aesrst(&self) -> AESRST_R {
        AESRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - DIVRST"]
    #[inline(always)]
    pub fn divrst(&self) -> DIVRST_R {
        DIVRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMARST"]
    #[inline(always)]
    #[must_use]
    pub fn dmarst(&mut self) -> DMARST_W<0> {
        DMARST_W::new(self)
    }
    #[doc = "Bit 5 - USBRST"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<5> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 7 - CRCRST"]
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<7> {
        CRCRST_W::new(self)
    }
    #[doc = "Bit 8 - PARST"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<8> {
        PARST_W::new(self)
    }
    #[doc = "Bit 9 - PBRST"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<9> {
        PBRST_W::new(self)
    }
    #[doc = "Bit 10 - PCRST"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<10> {
        PCRST_W::new(self)
    }
    #[doc = "Bit 11 - PDRST"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<11> {
        PDRST_W::new(self)
    }
    #[doc = "Bit 12 - PDRST"]
    #[inline(always)]
    #[must_use]
    pub fn perst(&mut self) -> PERST_W<12> {
        PERST_W::new(self)
    }
    #[doc = "Bit 15 - AESRST"]
    #[inline(always)]
    #[must_use]
    pub fn aesrst(&mut self) -> AESRST_W<15> {
        AESRST_W::new(self)
    }
    #[doc = "Bit 24 - DIVRST"]
    #[inline(always)]
    #[must_use]
    pub fn divrst(&mut self) -> DIVRST_W<24> {
        DIVRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RSTCU_AHBPRSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_ahbprstr](index.html) module"]
pub struct RSTCU_AHBPRSTR_SPEC;
impl crate::RegisterSpec for RSTCU_AHBPRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcu_ahbprstr::R](R) reader structure"]
impl crate::Readable for RSTCU_AHBPRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcu_ahbprstr::W](W) writer structure"]
impl crate::Writable for RSTCU_AHBPRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCU_AHBPRSTR to value 0"]
impl crate::Resettable for RSTCU_AHBPRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
