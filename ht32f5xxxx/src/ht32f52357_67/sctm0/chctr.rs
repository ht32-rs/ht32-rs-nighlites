#[doc = "Register `CHCTR` reader"]
pub struct R(crate::R<CHCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTR` writer"]
pub struct W(crate::W<CHCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTR_SPEC>;
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
impl From<crate::W<CHCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHE` reader - CHE"]
pub type CHE_R = crate::BitReader;
#[doc = "Field `CHE` writer - CHE"]
pub type CHE_W<'a, const O: u8> = crate::BitWriter<'a, CHCTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CHE"]
    #[inline(always)]
    pub fn che(&self) -> CHE_R {
        CHE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHE"]
    #[inline(always)]
    #[must_use]
    pub fn che(&mut self) -> CHE_W<0> {
        CHE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctr](index.html) module"]
pub struct CHCTR_SPEC;
impl crate::RegisterSpec for CHCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctr::R](R) reader structure"]
impl crate::Readable for CHCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctr::W](W) writer structure"]
impl crate::Writable for CHCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCTR to value 0"]
impl crate::Resettable for CHCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
