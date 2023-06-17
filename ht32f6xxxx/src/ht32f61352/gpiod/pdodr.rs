#[doc = "Register `PDODR` reader"]
pub struct R(crate::R<PDODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDODR` writer"]
pub struct W(crate::W<PDODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDODR_SPEC>;
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
impl From<crate::W<PDODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDOD` reader - PDOD"]
pub type PDOD_R = crate::FieldReader;
#[doc = "Field `PDOD` writer - PDOD"]
pub type PDOD_W<'a, const O: u8> = crate::FieldWriter<'a, PDODR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDOD"]
    #[inline(always)]
    pub fn pdod(&self) -> PDOD_R {
        PDOD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDOD"]
    #[inline(always)]
    #[must_use]
    pub fn pdod(&mut self) -> PDOD_W<0> {
        PDOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdodr](index.html) module"]
pub struct PDODR_SPEC;
impl crate::RegisterSpec for PDODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdodr::R](R) reader structure"]
impl crate::Readable for PDODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdodr::W](W) writer structure"]
impl crate::Writable for PDODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDODR to value 0"]
impl crate::Resettable for PDODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
