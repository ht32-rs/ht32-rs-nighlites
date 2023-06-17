#[doc = "Register `WDT_SR` reader"]
pub struct R(crate::R<WDT_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_SR` writer"]
pub struct W(crate::W<WDT_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_SR_SPEC>;
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
impl From<crate::W<WDT_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTUF` reader - WDTUF"]
pub type WDTUF_R = crate::BitReader;
#[doc = "Field `WDTUF` writer - WDTUF"]
pub type WDTUF_W<'a, const O: u8> = crate::BitWriter<'a, WDT_SR_SPEC, O>;
#[doc = "Field `WDTERR` reader - WDTERR"]
pub type WDTERR_R = crate::BitReader;
#[doc = "Field `WDTERR` writer - WDTERR"]
pub type WDTERR_W<'a, const O: u8> = crate::BitWriter<'a, WDT_SR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    pub fn wdtuf(&self) -> WDTUF_R {
        WDTUF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    pub fn wdterr(&self) -> WDTERR_R {
        WDTERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDTUF"]
    #[inline(always)]
    #[must_use]
    pub fn wdtuf(&mut self) -> WDTUF_W<0> {
        WDTUF_W::new(self)
    }
    #[doc = "Bit 1 - WDTERR"]
    #[inline(always)]
    #[must_use]
    pub fn wdterr(&mut self) -> WDTERR_W<1> {
        WDTERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_sr](index.html) module"]
pub struct WDT_SR_SPEC;
impl crate::RegisterSpec for WDT_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_sr::R](R) reader structure"]
impl crate::Readable for WDT_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_sr::W](W) writer structure"]
impl crate::Writable for WDT_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_SR to value 0"]
impl crate::Resettable for WDT_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
