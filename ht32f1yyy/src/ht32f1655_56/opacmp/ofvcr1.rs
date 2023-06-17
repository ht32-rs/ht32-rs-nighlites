#[doc = "Register `OFVCR1` reader"]
pub struct R(crate::R<OFVCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFVCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFVCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFVCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFVCR1` writer"]
pub struct W(crate::W<OFVCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFVCR1_SPEC>;
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
impl From<crate::W<OFVCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFVCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A1OF` reader - A1OF"]
pub type A1OF_R = crate::FieldReader;
#[doc = "Field `A1OF` writer - A1OF"]
pub type A1OF_W<'a, const O: u8> = crate::FieldWriter<'a, OFVCR1_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - A1OF"]
    #[inline(always)]
    pub fn a1of(&self) -> A1OF_R {
        A1OF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A1OF"]
    #[inline(always)]
    #[must_use]
    pub fn a1of(&mut self) -> A1OF_W<0> {
        A1OF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OFVCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofvcr1](index.html) module"]
pub struct OFVCR1_SPEC;
impl crate::RegisterSpec for OFVCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofvcr1::R](R) reader structure"]
impl crate::Readable for OFVCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofvcr1::W](W) writer structure"]
impl crate::Writable for OFVCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFVCR1 to value 0"]
impl crate::Resettable for OFVCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
