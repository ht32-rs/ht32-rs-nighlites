#[doc = "Register `CMPISR0` reader"]
pub struct R(crate::R<CMPISR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPISR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPISR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPISR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPISR0` writer"]
pub struct W(crate::W<CMPISR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPISR0_SPEC>;
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
impl From<crate::W<CMPISR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPISR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF0IS` reader - CF0IS"]
pub type CF0IS_R = crate::BitReader;
#[doc = "Field `CF0IS` writer - CF0IS"]
pub type CF0IS_W<'a, const O: u8> = crate::BitWriter<'a, CMPISR0_SPEC, O>;
#[doc = "Field `CR0IS` reader - CR0IS"]
pub type CR0IS_R = crate::BitReader;
#[doc = "Field `CR0IS` writer - CR0IS"]
pub type CR0IS_W<'a, const O: u8> = crate::BitWriter<'a, CMPISR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF0IS"]
    #[inline(always)]
    pub fn cf0is(&self) -> CF0IS_R {
        CF0IS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR0IS"]
    #[inline(always)]
    pub fn cr0is(&self) -> CR0IS_R {
        CR0IS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0IS"]
    #[inline(always)]
    #[must_use]
    pub fn cf0is(&mut self) -> CF0IS_W<0> {
        CF0IS_W::new(self)
    }
    #[doc = "Bit 1 - CR0IS"]
    #[inline(always)]
    #[must_use]
    pub fn cr0is(&mut self) -> CR0IS_W<1> {
        CR0IS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPISR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpisr0](index.html) module"]
pub struct CMPISR0_SPEC;
impl crate::RegisterSpec for CMPISR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpisr0::R](R) reader structure"]
impl crate::Readable for CMPISR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpisr0::W](W) writer structure"]
impl crate::Writable for CMPISR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPISR0 to value 0"]
impl crate::Resettable for CMPISR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
