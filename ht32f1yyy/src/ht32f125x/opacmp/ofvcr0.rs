#[doc = "Register `OFVCR0` reader"]
pub struct R(crate::R<OFVCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFVCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFVCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFVCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFVCR0` writer"]
pub struct W(crate::W<OFVCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFVCR0_SPEC>;
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
impl From<crate::W<OFVCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFVCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A0OF` reader - A0OF"]
pub type A0OF_R = crate::FieldReader;
#[doc = "Field `A0OF` writer - A0OF"]
pub type A0OF_W<'a, const O: u8> = crate::FieldWriter<'a, OFVCR0_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - A0OF"]
    #[inline(always)]
    pub fn a0of(&self) -> A0OF_R {
        A0OF_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - A0OF"]
    #[inline(always)]
    #[must_use]
    pub fn a0of(&mut self) -> A0OF_W<0> {
        A0OF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OFVCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofvcr0](index.html) module"]
pub struct OFVCR0_SPEC;
impl crate::RegisterSpec for OFVCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofvcr0::R](R) reader structure"]
impl crate::Readable for OFVCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofvcr0::W](W) writer structure"]
impl crate::Writable for OFVCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFVCR0 to value 0"]
impl crate::Resettable for OFVCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
