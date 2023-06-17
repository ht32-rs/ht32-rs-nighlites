#[doc = "Register `DIV_DSR` reader"]
pub struct R(crate::R<DIV_DSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_DSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_DSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_DSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_DSR` writer"]
pub struct W(crate::W<DIV_DSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_DSR_SPEC>;
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
impl From<crate::W<DIV_DSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_DSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSR` reader - DSR"]
pub type DSR_R = crate::FieldReader<u32>;
#[doc = "Field `DSR` writer - DSR"]
pub type DSR_W<'a, const O: u8> = crate::FieldWriter<'a, DIV_DSR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DSR"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DSR"]
    #[inline(always)]
    #[must_use]
    pub fn dsr(&mut self) -> DSR_W<0> {
        DSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DIV_DSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_dsr](index.html) module"]
pub struct DIV_DSR_SPEC;
impl crate::RegisterSpec for DIV_DSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_dsr::R](R) reader structure"]
impl crate::Readable for DIV_DSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_dsr::W](W) writer structure"]
impl crate::Writable for DIV_DSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV_DSR to value 0"]
impl crate::Resettable for DIV_DSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
