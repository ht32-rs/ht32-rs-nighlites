#[doc = "Register `CMPIER1` reader"]
pub struct R(crate::R<CMPIER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPIER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPIER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPIER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPIER1` writer"]
pub struct W(crate::W<CMPIER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPIER1_SPEC>;
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
impl From<crate::W<CMPIER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPIER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF1IEN` reader - CF1IEN"]
pub type CF1IEN_R = crate::BitReader;
#[doc = "Field `CF1IEN` writer - CF1IEN"]
pub type CF1IEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPIER1_SPEC, O>;
#[doc = "Field `CR1IEN` reader - CR1IEN"]
pub type CR1IEN_R = crate::BitReader;
#[doc = "Field `CR1IEN` writer - CR1IEN"]
pub type CR1IEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPIER1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF1IEN"]
    #[inline(always)]
    pub fn cf1ien(&self) -> CF1IEN_R {
        CF1IEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR1IEN"]
    #[inline(always)]
    pub fn cr1ien(&self) -> CR1IEN_R {
        CR1IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF1IEN"]
    #[inline(always)]
    #[must_use]
    pub fn cf1ien(&mut self) -> CF1IEN_W<0> {
        CF1IEN_W::new(self)
    }
    #[doc = "Bit 1 - CR1IEN"]
    #[inline(always)]
    #[must_use]
    pub fn cr1ien(&mut self) -> CR1IEN_W<1> {
        CR1IEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPIER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpier1](index.html) module"]
pub struct CMPIER1_SPEC;
impl crate::RegisterSpec for CMPIER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpier1::R](R) reader structure"]
impl crate::Readable for CMPIER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpier1::W](W) writer structure"]
impl crate::Writable for CMPIER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPIER1 to value 0"]
impl crate::Resettable for CMPIER1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
