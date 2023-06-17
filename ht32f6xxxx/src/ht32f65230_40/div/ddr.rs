#[doc = "Register `DDR` reader"]
pub struct R(crate::R<DDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDR` writer"]
pub struct W(crate::W<DDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDR_SPEC>;
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
impl From<crate::W<DDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDR` reader - DDR"]
pub type DDR_R = crate::FieldReader<u32>;
#[doc = "Field `DDR` writer - DDR"]
pub type DDR_W<'a, const O: u8> = crate::FieldWriter<'a, DDR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DDR"]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DDR"]
    #[inline(always)]
    #[must_use]
    pub fn ddr(&mut self) -> DDR_W<0> {
        DDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddr](index.html) module"]
pub struct DDR_SPEC;
impl crate::RegisterSpec for DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddr::R](R) reader structure"]
impl crate::Readable for DDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddr::W](W) writer structure"]
impl crate::Writable for DDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DDR to value 0"]
impl crate::Resettable for DDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
