#[doc = "Register `CMPIER0` reader"]
pub struct R(crate::R<CMPIER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPIER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPIER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPIER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPIER0` writer"]
pub struct W(crate::W<CMPIER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPIER0_SPEC>;
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
impl From<crate::W<CMPIER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPIER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CF0IEN` reader - CF0IEN"]
pub type CF0IEN_R = crate::BitReader;
#[doc = "Field `CF0IEN` writer - CF0IEN"]
pub type CF0IEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPIER0_SPEC, O>;
#[doc = "Field `CR0IEN` reader - CR0IEN"]
pub type CR0IEN_R = crate::BitReader;
#[doc = "Field `CR0IEN` writer - CR0IEN"]
pub type CR0IEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPIER0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CF0IEN"]
    #[inline(always)]
    pub fn cf0ien(&self) -> CF0IEN_R {
        CF0IEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR0IEN"]
    #[inline(always)]
    pub fn cr0ien(&self) -> CR0IEN_R {
        CR0IEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CF0IEN"]
    #[inline(always)]
    #[must_use]
    pub fn cf0ien(&mut self) -> CF0IEN_W<0> {
        CF0IEN_W::new(self)
    }
    #[doc = "Bit 1 - CR0IEN"]
    #[inline(always)]
    #[must_use]
    pub fn cr0ien(&mut self) -> CR0IEN_W<1> {
        CR0IEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPIER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpier0](index.html) module"]
pub struct CMPIER0_SPEC;
impl crate::RegisterSpec for CMPIER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpier0::R](R) reader structure"]
impl crate::Readable for CMPIER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpier0::W](W) writer structure"]
impl crate::Writable for CMPIER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPIER0 to value 0"]
impl crate::Resettable for CMPIER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
