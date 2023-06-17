#[doc = "Register `CMPCI0` reader"]
pub struct R(crate::R<CMPCI0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCI0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCI0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCI0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCI0` writer"]
pub struct W(crate::W<CMPCI0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCI0_SPEC>;
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
impl From<crate::W<CMPCI0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCI0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPIPSEL` reader - CMPIPSEL"]
pub type CMPIPSEL_R = crate::FieldReader;
#[doc = "Field `CMPIPSEL` writer - CMPIPSEL"]
pub type CMPIPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, CMPCI0_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - CMPIPSEL"]
    #[inline(always)]
    pub fn cmpipsel(&self) -> CMPIPSEL_R {
        CMPIPSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CMPIPSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cmpipsel(&mut self) -> CMPIPSEL_W<0> {
        CMPIPSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPCI0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpci0](index.html) module"]
pub struct CMPCI0_SPEC;
impl crate::RegisterSpec for CMPCI0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpci0::R](R) reader structure"]
impl crate::Readable for CMPCI0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpci0::W](W) writer structure"]
impl crate::Writable for CMPCI0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPCI0 to value 0"]
impl crate::Resettable for CMPCI0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
