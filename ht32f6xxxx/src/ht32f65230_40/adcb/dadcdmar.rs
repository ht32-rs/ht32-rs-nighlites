#[doc = "Register `DADCDMAR` reader"]
pub struct R(crate::R<DADCDMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADCDMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADCDMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADCDMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DADCDMAR` writer"]
pub struct W(crate::W<DADCDMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADCDMAR_SPEC>;
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
impl From<crate::W<DADCDMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADCDMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADCDMAS` reader - DADCDMAS"]
pub type DADCDMAS_R = crate::BitReader;
#[doc = "Field `DADCDMAS` writer - DADCDMAS"]
pub type DADCDMAS_W<'a, const O: u8> = crate::BitWriter<'a, DADCDMAR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DADCDMAS"]
    #[inline(always)]
    pub fn dadcdmas(&self) -> DADCDMAS_R {
        DADCDMAS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DADCDMAS"]
    #[inline(always)]
    #[must_use]
    pub fn dadcdmas(&mut self) -> DADCDMAS_W<0> {
        DADCDMAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DADCDMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dadcdmar](index.html) module"]
pub struct DADCDMAR_SPEC;
impl crate::RegisterSpec for DADCDMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dadcdmar::R](R) reader structure"]
impl crate::Readable for DADCDMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dadcdmar::W](W) writer structure"]
impl crate::Writable for DADCDMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DADCDMAR to value 0"]
impl crate::Resettable for DADCDMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
