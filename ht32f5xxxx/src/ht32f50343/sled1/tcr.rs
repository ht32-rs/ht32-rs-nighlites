#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0H` reader - T0H"]
pub type T0H_R = crate::FieldReader;
#[doc = "Field `T0H` writer - T0H"]
pub type T0H_W<'a, const O: u8> = crate::FieldWriter<'a, TCR_SPEC, 5, O>;
#[doc = "Field `T1H` reader - T1H"]
pub type T1H_R = crate::FieldReader;
#[doc = "Field `T1H` writer - T1H"]
pub type T1H_W<'a, const O: u8> = crate::FieldWriter<'a, TCR_SPEC, 5, O>;
#[doc = "Field `TRST` reader - TRST"]
pub type TRST_R = crate::FieldReader;
#[doc = "Field `TRST` writer - TRST"]
pub type TRST_W<'a, const O: u8> = crate::FieldWriter<'a, TCR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:4 - T0H"]
    #[inline(always)]
    pub fn t0h(&self) -> T0H_R {
        T0H_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - T1H"]
    #[inline(always)]
    pub fn t1h(&self) -> T1H_R {
        T1H_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - TRST"]
    #[inline(always)]
    pub fn trst(&self) -> TRST_R {
        TRST_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - T0H"]
    #[inline(always)]
    #[must_use]
    pub fn t0h(&mut self) -> T0H_W<0> {
        T0H_W::new(self)
    }
    #[doc = "Bits 8:12 - T1H"]
    #[inline(always)]
    #[must_use]
    pub fn t1h(&mut self) -> T1H_W<8> {
        T1H_W::new(self)
    }
    #[doc = "Bits 16:23 - TRST"]
    #[inline(always)]
    #[must_use]
    pub fn trst(&mut self) -> TRST_W<16> {
        TRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
