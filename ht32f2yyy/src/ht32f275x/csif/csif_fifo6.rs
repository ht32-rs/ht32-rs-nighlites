#[doc = "Register `CSIF_FIFO6` reader"]
pub struct R(crate::R<CSIF_FIFO6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_FIFO6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_FIFO6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_FIFO6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_FIFO6` writer"]
pub struct W(crate::W<CSIF_FIFO6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_FIFO6_SPEC>;
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
impl From<crate::W<CSIF_FIFO6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_FIFO6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFODATA` reader - FIFODATA"]
pub type FIFODATA_R = crate::FieldReader<u32>;
#[doc = "Field `FIFODATA` writer - FIFODATA"]
pub type FIFODATA_W<'a, const O: u8> = crate::FieldWriter<'a, CSIF_FIFO6_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFODATA"]
    #[inline(always)]
    pub fn fifodata(&self) -> FIFODATA_R {
        FIFODATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFODATA"]
    #[inline(always)]
    #[must_use]
    pub fn fifodata(&mut self) -> FIFODATA_W<0> {
        FIFODATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_FIFO6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo6](index.html) module"]
pub struct CSIF_FIFO6_SPEC;
impl crate::RegisterSpec for CSIF_FIFO6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_fifo6::R](R) reader structure"]
impl crate::Readable for CSIF_FIFO6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_fifo6::W](W) writer structure"]
impl crate::Writable for CSIF_FIFO6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_FIFO6 to value 0"]
impl crate::Resettable for CSIF_FIFO6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
