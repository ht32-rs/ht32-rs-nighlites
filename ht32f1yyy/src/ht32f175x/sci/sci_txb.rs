#[doc = "Register `SCI_TXB` reader"]
pub struct R(crate::R<SCI_TXB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCI_TXB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCI_TXB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCI_TXB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCI_TXB` writer"]
pub struct W(crate::W<SCI_TXB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCI_TXB_SPEC>;
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
impl From<crate::W<SCI_TXB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCI_TXB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TB` reader - TB"]
pub type TB_R = crate::FieldReader;
#[doc = "Field `TB` writer - TB"]
pub type TB_W<'a, const O: u8> = crate::FieldWriter<'a, SCI_TXB_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    pub fn tb(&self) -> TB_R {
        TB_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TB"]
    #[inline(always)]
    #[must_use]
    pub fn tb(&mut self) -> TB_W<0> {
        TB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI_TXB\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_txb](index.html) module"]
pub struct SCI_TXB_SPEC;
impl crate::RegisterSpec for SCI_TXB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sci_txb::R](R) reader structure"]
impl crate::Readable for SCI_TXB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sci_txb::W](W) writer structure"]
impl crate::Writable for SCI_TXB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCI_TXB to value 0"]
impl crate::Resettable for SCI_TXB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
