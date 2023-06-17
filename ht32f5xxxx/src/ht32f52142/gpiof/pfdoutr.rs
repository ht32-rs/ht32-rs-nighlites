#[doc = "Register `PFDOUTR` reader"]
pub struct R(crate::R<PFDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFDOUTR` writer"]
pub struct W(crate::W<PFDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFDOUTR_SPEC>;
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
impl From<crate::W<PFDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFDOUT0` reader - PFDOUT0"]
pub type PFDOUT0_R = crate::BitReader;
#[doc = "Field `PFDOUT0` writer - PFDOUT0"]
pub type PFDOUT0_W<'a, const O: u8> = crate::BitWriter<'a, PFDOUTR_SPEC, O>;
#[doc = "Field `PFDOUT1` reader - PFDOUT1"]
pub type PFDOUT1_R = crate::BitReader;
#[doc = "Field `PFDOUT1` writer - PFDOUT1"]
pub type PFDOUT1_W<'a, const O: u8> = crate::BitWriter<'a, PFDOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFDOUT0"]
    #[inline(always)]
    pub fn pfdout0(&self) -> PFDOUT0_R {
        PFDOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFDOUT1"]
    #[inline(always)]
    pub fn pfdout1(&self) -> PFDOUT1_R {
        PFDOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFDOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn pfdout0(&mut self) -> PFDOUT0_W<0> {
        PFDOUT0_W::new(self)
    }
    #[doc = "Bit 1 - PFDOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn pfdout1(&mut self) -> PFDOUT1_W<1> {
        PFDOUT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfdoutr](index.html) module"]
pub struct PFDOUTR_SPEC;
impl crate::RegisterSpec for PFDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfdoutr::R](R) reader structure"]
impl crate::Readable for PFDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfdoutr::W](W) writer structure"]
impl crate::Writable for PFDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFDOUTR to value 0"]
impl crate::Resettable for PFDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
