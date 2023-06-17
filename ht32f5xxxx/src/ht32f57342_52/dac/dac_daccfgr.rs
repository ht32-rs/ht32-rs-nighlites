#[doc = "Register `DAC_DACCFGR` reader"]
pub struct R(crate::R<DAC_DACCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_DACCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_DACCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_DACCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_DACCFGR` writer"]
pub struct W(crate::W<DAC_DACCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_DACCFGR_SPEC>;
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
impl From<crate::W<DAC_DACCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_DACCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, DAC_DACCFGR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_DACCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_daccfgr](index.html) module"]
pub struct DAC_DACCFGR_SPEC;
impl crate::RegisterSpec for DAC_DACCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_daccfgr::R](R) reader structure"]
impl crate::Readable for DAC_DACCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_daccfgr::W](W) writer structure"]
impl crate::Writable for DAC_DACCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_DACCFGR to value 0"]
impl crate::Resettable for DAC_DACCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
