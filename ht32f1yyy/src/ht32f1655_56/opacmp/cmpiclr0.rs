#[doc = "Register `CMPICLR0` reader"]
pub struct R(crate::R<CMPICLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPICLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPICLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPICLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPICLR0` writer"]
pub struct W(crate::W<CMPICLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPICLR0_SPEC>;
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
impl From<crate::W<CMPICLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPICLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF0ICLR` reader - CF0ICLR"]
pub type CF0ICLR_R = crate::BitReader;
#[doc = "Field `CF0ICLR` writer - CF0ICLR"]
pub type CF0ICLR_W<'a, const O: u8> = crate::BitWriter<'a, CMPICLR0_SPEC, O>;
#[doc = "Field `CR0ICLR` reader - CR0ICLR"]
pub type CR0ICLR_R = crate::BitReader;
#[doc = "Field `CR0ICLR` writer - CR0ICLR"]
pub type CR0ICLR_W<'a, const O: u8> = crate::BitWriter<'a, CMPICLR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF0ICLR"]
    #[inline(always)]
    pub fn cf0iclr(&self) -> CF0ICLR_R {
        CF0ICLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR0ICLR"]
    #[inline(always)]
    pub fn cr0iclr(&self) -> CR0ICLR_R {
        CR0ICLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0ICLR"]
    #[inline(always)]
    #[must_use]
    pub fn cf0iclr(&mut self) -> CF0ICLR_W<0> {
        CF0ICLR_W::new(self)
    }
    #[doc = "Bit 1 - CR0ICLR"]
    #[inline(always)]
    #[must_use]
    pub fn cr0iclr(&mut self) -> CR0ICLR_W<1> {
        CR0ICLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPICLR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpiclr0](index.html) module"]
pub struct CMPICLR0_SPEC;
impl crate::RegisterSpec for CMPICLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpiclr0::R](R) reader structure"]
impl crate::Readable for CMPICLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpiclr0::W](W) writer structure"]
impl crate::Writable for CMPICLR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPICLR0 to value 0"]
impl crate::Resettable for CMPICLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
