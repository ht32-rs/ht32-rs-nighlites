#[doc = "Register `QTR` reader"]
pub struct R(crate::R<QTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QTR` writer"]
pub struct W(crate::W<QTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QTR_SPEC>;
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
impl From<crate::W<QTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QTR` reader - QTR"]
pub type QTR_R = crate::FieldReader<u32>;
#[doc = "Field `QTR` writer - QTR"]
pub type QTR_W<'a, const O: u8> = crate::FieldWriter<'a, QTR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - QTR"]
    #[inline(always)]
    pub fn qtr(&self) -> QTR_R {
        QTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - QTR"]
    #[inline(always)]
    #[must_use]
    pub fn qtr(&mut self) -> QTR_W<0> {
        QTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qtr](index.html) module"]
pub struct QTR_SPEC;
impl crate::RegisterSpec for QTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qtr::R](R) reader structure"]
impl crate::Readable for QTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [qtr::W](W) writer structure"]
impl crate::Writable for QTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets QTR to value 0"]
impl crate::Resettable for QTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
