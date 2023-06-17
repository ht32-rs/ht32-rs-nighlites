#[doc = "Register `CMPRSR0` reader"]
pub struct R(crate::R<CMPRSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPRSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPRSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPRSR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPRSR0` writer"]
pub struct W(crate::W<CMPRSR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPRSR0_SPEC>;
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
impl From<crate::W<CMPRSR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPRSR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF0RAW` reader - CF0RAW"]
pub type CF0RAW_R = crate::BitReader;
#[doc = "Field `CF0RAW` writer - CF0RAW"]
pub type CF0RAW_W<'a, const O: u8> = crate::BitWriter<'a, CMPRSR0_SPEC, O>;
#[doc = "Field `CR0RAW` reader - CR0RAW"]
pub type CR0RAW_R = crate::BitReader;
#[doc = "Field `CR0RAW` writer - CR0RAW"]
pub type CR0RAW_W<'a, const O: u8> = crate::BitWriter<'a, CMPRSR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF0RAW"]
    #[inline(always)]
    pub fn cf0raw(&self) -> CF0RAW_R {
        CF0RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR0RAW"]
    #[inline(always)]
    pub fn cr0raw(&self) -> CR0RAW_R {
        CR0RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0RAW"]
    #[inline(always)]
    #[must_use]
    pub fn cf0raw(&mut self) -> CF0RAW_W<0> {
        CF0RAW_W::new(self)
    }
    #[doc = "Bit 1 - CR0RAW"]
    #[inline(always)]
    #[must_use]
    pub fn cr0raw(&mut self) -> CR0RAW_W<1> {
        CR0RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPRSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprsr0](index.html) module"]
pub struct CMPRSR0_SPEC;
impl crate::RegisterSpec for CMPRSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmprsr0::R](R) reader structure"]
impl crate::Readable for CMPRSR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmprsr0::W](W) writer structure"]
impl crate::Writable for CMPRSR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPRSR0 to value 0"]
impl crate::Resettable for CMPRSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
