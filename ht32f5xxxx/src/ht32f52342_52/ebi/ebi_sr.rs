#[doc = "Register `EBI_SR` reader"]
pub struct R(crate::R<EBI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EBI_SR` writer"]
pub struct W(crate::W<EBI_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBI_SR_SPEC>;
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
impl From<crate::W<EBI_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBI_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EBIBUSY` reader - EBIBUSY"]
pub type EBIBUSY_R = crate::BitReader;
#[doc = "Field `EBIBUSY` writer - EBIBUSY"]
pub type EBIBUSY_W<'a, const O: u8> = crate::BitWriter<'a, EBI_SR_SPEC, O>;
#[doc = "Field `EBISMRST` reader - EBISMRST"]
pub type EBISMRST_R = crate::BitReader;
#[doc = "Field `EBISMRST` writer - EBISMRST"]
pub type EBISMRST_W<'a, const O: u8> = crate::BitWriter<'a, EBI_SR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    pub fn ebibusy(&self) -> EBIBUSY_R {
        EBIBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    pub fn ebismrst(&self) -> EBISMRST_R {
        EBISMRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EBIBUSY"]
    #[inline(always)]
    #[must_use]
    pub fn ebibusy(&mut self) -> EBIBUSY_W<0> {
        EBIBUSY_W::new(self)
    }
    #[doc = "Bit 8 - EBISMRST"]
    #[inline(always)]
    #[must_use]
    pub fn ebismrst(&mut self) -> EBISMRST_W<8> {
        EBISMRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBI_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_sr](index.html) module"]
pub struct EBI_SR_SPEC;
impl crate::RegisterSpec for EBI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebi_sr::R](R) reader structure"]
impl crate::Readable for EBI_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ebi_sr::W](W) writer structure"]
impl crate::Writable for EBI_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EBI_SR to value 0"]
impl crate::Resettable for EBI_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
