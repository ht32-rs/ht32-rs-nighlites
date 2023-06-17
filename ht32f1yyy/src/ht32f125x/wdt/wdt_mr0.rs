#[doc = "Register `WDT_MR0` reader"]
pub struct R(crate::R<WDT_MR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDT_MR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDT_MR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDT_MR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDT_MR0` writer"]
pub struct W(crate::W<WDT_MR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDT_MR0_SPEC>;
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
impl From<crate::W<WDT_MR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDT_MR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTV` reader - WDTV"]
pub type WDTV_R = crate::FieldReader<u16>;
#[doc = "Field `WDTV` writer - WDTV"]
pub type WDTV_W<'a, const O: u8> = crate::FieldWriter<'a, WDT_MR0_SPEC, 12, O, u16>;
#[doc = "Field `WDTFIEN` reader - WDTFIEN"]
pub type WDTFIEN_R = crate::BitReader;
#[doc = "Field `WDTFIEN` writer - WDTFIEN"]
pub type WDTFIEN_W<'a, const O: u8> = crate::BitWriter<'a, WDT_MR0_SPEC, O>;
#[doc = "Field `WDTRSTEN` reader - WDTRSTEN"]
pub type WDTRSTEN_R = crate::BitReader;
#[doc = "Field `WDTRSTEN` writer - WDTRSTEN"]
pub type WDTRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, WDT_MR0_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    pub fn wdtv(&self) -> WDTV_R {
        WDTV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    pub fn wdtfien(&self) -> WDTFIEN_R {
        WDTFIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    pub fn wdtrsten(&self) -> WDTRSTEN_R {
        WDTRSTEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - WDTV"]
    #[inline(always)]
    #[must_use]
    pub fn wdtv(&mut self) -> WDTV_W<0> {
        WDTV_W::new(self)
    }
    #[doc = "Bit 12 - WDTFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdtfien(&mut self) -> WDTFIEN_W<12> {
        WDTFIEN_W::new(self)
    }
    #[doc = "Bit 13 - WDTRSTEN"]
    #[inline(always)]
    #[must_use]
    pub fn wdtrsten(&mut self) -> WDTRSTEN_W<13> {
        WDTRSTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT_MR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_mr0](index.html) module"]
pub struct WDT_MR0_SPEC;
impl crate::RegisterSpec for WDT_MR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdt_mr0::R](R) reader structure"]
impl crate::Readable for WDT_MR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdt_mr0::W](W) writer structure"]
impl crate::Writable for WDT_MR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDT_MR0 to value 0"]
impl crate::Resettable for WDT_MR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
