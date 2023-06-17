#[doc = "Register `CMPRSR1` reader"]
pub struct R(crate::R<CMPRSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRSR1` writer"]
pub struct W(crate::W<CMPRSR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRSR1_SPEC>;
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
impl From<crate::W<CMPRSR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRSR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF1RAW` reader - CF1RAW"]
pub type CF1RAW_R = crate::BitReader;
#[doc = "Field `CF1RAW` writer - CF1RAW"]
pub type CF1RAW_W<'a, const O: u8> = crate::BitWriter<'a, CMPRSR1_SPEC, O>;
#[doc = "Field `CR1RAW` reader - CR1RAW"]
pub type CR1RAW_R = crate::BitReader;
#[doc = "Field `CR1RAW` writer - CR1RAW"]
pub type CR1RAW_W<'a, const O: u8> = crate::BitWriter<'a, CMPRSR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF1RAW"]
    #[inline(always)]
    pub fn cf1raw(&self) -> CF1RAW_R {
        CF1RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR1RAW"]
    #[inline(always)]
    pub fn cr1raw(&self) -> CR1RAW_R {
        CR1RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1RAW"]
    #[inline(always)]
    #[must_use]
    pub fn cf1raw(&mut self) -> CF1RAW_W<0> {
        CF1RAW_W::new(self)
    }
    #[doc = "Bit 1 - CR1RAW"]
    #[inline(always)]
    #[must_use]
    pub fn cr1raw(&mut self) -> CR1RAW_W<1> {
        CR1RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPRSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprsr1](index.html) module"]
pub struct CMPRSR1_SPEC;
impl crate::RegisterSpec for CMPRSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmprsr1::R](R) reader structure"]
impl crate::Readable for CMPRSR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmprsr1::W](W) writer structure"]
impl crate::Writable for CMPRSR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPRSR1 to value 0"]
impl crate::Resettable for CMPRSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
