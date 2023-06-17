#[doc = "Register `PDMA_CH4TSR` reader"]
pub struct R(crate::R<PDMA_CH4TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMA_CH4TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMA_CH4TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMA_CH4TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMA_CH4TSR` writer"]
pub struct W(crate::W<PDMA_CH4TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMA_CH4TSR_SPEC>;
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
impl From<crate::W<PDMA_CH4TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMA_CH4TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLKLEN` reader - BLKLEN"]
pub type BLKLEN_R = crate::FieldReader;
#[doc = "Field `BLKLEN` writer - BLKLEN"]
pub type BLKLEN_W<'a, const O: u8> = crate::FieldWriter<'a, PDMA_CH4TSR_SPEC, 8, O>;
#[doc = "Field `BLKCNT` reader - BLKCNT"]
pub type BLKCNT_R = crate::FieldReader<u16>;
#[doc = "Field `BLKCNT` writer - BLKCNT"]
pub type BLKCNT_W<'a, const O: u8> = crate::FieldWriter<'a, PDMA_CH4TSR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:7 - BLKLEN"]
    #[inline(always)]
    pub fn blklen(&self) -> BLKLEN_R {
        BLKLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - BLKLEN"]
    #[inline(always)]
    #[must_use]
    pub fn blklen(&mut self) -> BLKLEN_W<0> {
        BLKLEN_W::new(self)
    }
    #[doc = "Bits 16:31 - BLKCNT"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BLKCNT_W<16> {
        BLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDMA_CH4TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4tsr](index.html) module"]
pub struct PDMA_CH4TSR_SPEC;
impl crate::RegisterSpec for PDMA_CH4TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdma_ch4tsr::R](R) reader structure"]
impl crate::Readable for PDMA_CH4TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdma_ch4tsr::W](W) writer structure"]
impl crate::Writable for PDMA_CH4TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDMA_CH4TSR to value 0"]
impl crate::Resettable for PDMA_CH4TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
