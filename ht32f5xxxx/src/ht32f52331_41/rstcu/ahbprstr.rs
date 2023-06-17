#[doc = "Register `AHBPRSTR` reader"]
pub struct R(crate::R<AHBPRSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBPRSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBPRSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBPRSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBPRSTR` writer"]
pub struct W(crate::W<AHBPRSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBPRSTR_SPEC>;
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
impl From<crate::W<AHBPRSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBPRSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBRST` reader - USBRST"]
pub type USBRST_R = crate::BitReader;
#[doc = "Field `USBRST` writer - USBRST"]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, AHBPRSTR_SPEC, O>;
#[doc = "Field `CRCRST` reader - CRCRST"]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRCRST"]
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, AHBPRSTR_SPEC, O>;
#[doc = "Field `PARST` reader - PARST"]
pub type PARST_R = crate::BitReader;
#[doc = "Field `PARST` writer - PARST"]
pub type PARST_W<'a, const O: u8> = crate::BitWriter<'a, AHBPRSTR_SPEC, O>;
#[doc = "Field `PBRST` reader - PBRST"]
pub type PBRST_R = crate::BitReader;
#[doc = "Field `PBRST` writer - PBRST"]
pub type PBRST_W<'a, const O: u8> = crate::BitWriter<'a, AHBPRSTR_SPEC, O>;
#[doc = "Field `PCRST` reader - PCRST"]
pub type PCRST_R = crate::BitReader;
#[doc = "Field `PCRST` writer - PCRST"]
pub type PCRST_W<'a, const O: u8> = crate::BitWriter<'a, AHBPRSTR_SPEC, O>;
impl R {
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
}
impl W {
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHBPRSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbprstr](index.html) module"]
pub struct AHBPRSTR_SPEC;
impl crate::RegisterSpec for AHBPRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbprstr::R](R) reader structure"]
impl crate::Readable for AHBPRSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbprstr::W](W) writer structure"]
impl crate::Writable for AHBPRSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHBPRSTR to value 0"]
impl crate::Resettable for AHBPRSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
