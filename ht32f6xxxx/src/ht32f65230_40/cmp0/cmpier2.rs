#[doc = "Register `CMPIER2` reader"]
pub struct R(crate::R<CMPIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPIER2` writer"]
pub struct W(crate::W<CMPIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPIER2_SPEC>;
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
impl From<crate::W<CMPIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPFIEN` reader - CMPFIEN"]
pub type CMPFIEN_R = crate::BitReader;
#[doc = "Field `CMPFIEN` writer - CMPFIEN"]
pub type CMPFIEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPIER2_SPEC, O>;
#[doc = "Field `CMPRIEN` reader - CMPRIEN"]
pub type CMPRIEN_R = crate::BitReader;
#[doc = "Field `CMPRIEN` writer - CMPRIEN"]
pub type CMPRIEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPIER2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CMPFIEN"]
    #[inline(always)]
    pub fn cmpfien(&self) -> CMPFIEN_R {
        CMPFIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMPRIEN"]
    #[inline(always)]
    pub fn cmprien(&self) -> CMPRIEN_R {
        CMPRIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMPFIEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpfien(&mut self) -> CMPFIEN_W<0> {
        CMPFIEN_W::new(self)
    }
    #[doc = "Bit 1 - CMPRIEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmprien(&mut self) -> CMPRIEN_W<1> {
        CMPRIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPIER2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpier2](index.html) module"]
pub struct CMPIER2_SPEC;
impl crate::RegisterSpec for CMPIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpier2::R](R) reader structure"]
impl crate::Readable for CMPIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpier2::W](W) writer structure"]
impl crate::Writable for CMPIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPIER2 to value 0"]
impl crate::Resettable for CMPIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
