#[doc = "Register `CMPISR1` reader"]
pub struct R(crate::R<CMPISR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPISR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPISR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPISR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPISR1` writer"]
pub struct W(crate::W<CMPISR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPISR1_SPEC>;
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
impl From<crate::W<CMPISR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPISR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF1IS` reader - CF1IS"]
pub type CF1IS_R = crate::BitReader;
#[doc = "Field `CF1IS` writer - CF1IS"]
pub type CF1IS_W<'a, const O: u8> = crate::BitWriter<'a, CMPISR1_SPEC, O>;
#[doc = "Field `CR1IS` reader - CR1IS"]
pub type CR1IS_R = crate::BitReader;
#[doc = "Field `CR1IS` writer - CR1IS"]
pub type CR1IS_W<'a, const O: u8> = crate::BitWriter<'a, CMPISR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF1IS"]
    #[inline(always)]
    pub fn cf1is(&self) -> CF1IS_R {
        CF1IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR1IS"]
    #[inline(always)]
    pub fn cr1is(&self) -> CR1IS_R {
        CR1IS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1IS"]
    #[inline(always)]
    #[must_use]
    pub fn cf1is(&mut self) -> CF1IS_W<0> {
        CF1IS_W::new(self)
    }
    #[doc = "Bit 1 - CR1IS"]
    #[inline(always)]
    #[must_use]
    pub fn cr1is(&mut self) -> CR1IS_W<1> {
        CR1IS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPISR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpisr1](index.html) module"]
pub struct CMPISR1_SPEC;
impl crate::RegisterSpec for CMPISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpisr1::R](R) reader structure"]
impl crate::Readable for CMPISR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpisr1::W](W) writer structure"]
impl crate::Writable for CMPISR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPISR1 to value 0"]
impl crate::Resettable for CMPISR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
