#[doc = "Register `DIV_DDR` reader"]
pub struct R(crate::R<DIV_DDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_DDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_DDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_DDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_DDR` writer"]
pub struct W(crate::W<DIV_DDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_DDR_SPEC>;
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
impl From<crate::W<DIV_DDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_DDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDR` reader - DDR"]
pub type DDR_R = crate::FieldReader<u32>;
#[doc = "Field `DDR` writer - DDR"]
pub type DDR_W<'a, const O: u8> = crate::FieldWriter<'a, DIV_DDR_SPEC, 32, O, u32>;
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
#[doc = "DIV_DDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_ddr](index.html) module"]
pub struct DIV_DDR_SPEC;
impl crate::RegisterSpec for DIV_DDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_ddr::R](R) reader structure"]
impl crate::Readable for DIV_DDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_ddr::W](W) writer structure"]
impl crate::Writable for DIV_DDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV_DDR to value 0"]
impl crate::Resettable for DIV_DDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
