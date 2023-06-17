#[doc = "Register `CMPICLR1` reader"]
pub struct R(crate::R<CMPICLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPICLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPICLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPICLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPICLR1` writer"]
pub struct W(crate::W<CMPICLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPICLR1_SPEC>;
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
impl From<crate::W<CMPICLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPICLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF1ICLR` reader - CF1ICLR"]
pub type CF1ICLR_R = crate::BitReader;
#[doc = "Field `CF1ICLR` writer - CF1ICLR"]
pub type CF1ICLR_W<'a, const O: u8> = crate::BitWriter<'a, CMPICLR1_SPEC, O>;
#[doc = "Field `CR1ICLR` reader - CR1ICLR"]
pub type CR1ICLR_R = crate::BitReader;
#[doc = "Field `CR1ICLR` writer - CR1ICLR"]
pub type CR1ICLR_W<'a, const O: u8> = crate::BitWriter<'a, CMPICLR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF1ICLR"]
    #[inline(always)]
    pub fn cf1iclr(&self) -> CF1ICLR_R {
        CF1ICLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR1ICLR"]
    #[inline(always)]
    pub fn cr1iclr(&self) -> CR1ICLR_R {
        CR1ICLR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1ICLR"]
    #[inline(always)]
    #[must_use]
    pub fn cf1iclr(&mut self) -> CF1ICLR_W<0> {
        CF1ICLR_W::new(self)
    }
    #[doc = "Bit 1 - CR1ICLR"]
    #[inline(always)]
    #[must_use]
    pub fn cr1iclr(&mut self) -> CR1ICLR_W<1> {
        CR1ICLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPICLR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpiclr1](index.html) module"]
pub struct CMPICLR1_SPEC;
impl crate::RegisterSpec for CMPICLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpiclr1::R](R) reader structure"]
impl crate::Readable for CMPICLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpiclr1::W](W) writer structure"]
impl crate::Writable for CMPICLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPICLR1 to value 0"]
impl crate::Resettable for CMPICLR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
