#[doc = "Register `DAC_TG` reader"]
pub struct R(crate::R<DAC_TG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_TG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_TG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_TG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_TG` writer"]
pub struct W(crate::W<DAC_TG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_TG_SPEC>;
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
impl From<crate::W<DAC_TG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_TG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSWTG` reader - RSWTG"]
pub type RSWTG_R = crate::BitReader;
#[doc = "Field `RSWTG` writer - RSWTG"]
pub type RSWTG_W<'a, const O: u8> = crate::BitWriter<'a, DAC_TG_SPEC, O>;
#[doc = "Field `LSWTG` reader - LSWTG"]
pub type LSWTG_R = crate::BitReader;
#[doc = "Field `LSWTG` writer - LSWTG"]
pub type LSWTG_W<'a, const O: u8> = crate::BitWriter<'a, DAC_TG_SPEC, O>;
impl R {
    #[doc = "Bit 0 - RSWTG"]
    #[inline(always)]
    pub fn rswtg(&self) -> RSWTG_R {
        RSWTG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - LSWTG"]
    #[inline(always)]
    pub fn lswtg(&self) -> LSWTG_R {
        LSWTG_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RSWTG"]
    #[inline(always)]
    #[must_use]
    pub fn rswtg(&mut self) -> RSWTG_W<0> {
        RSWTG_W::new(self)
    }
    #[doc = "Bit 8 - LSWTG"]
    #[inline(always)]
    #[must_use]
    pub fn lswtg(&mut self) -> LSWTG_W<8> {
        LSWTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_TG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_tg](index.html) module"]
pub struct DAC_TG_SPEC;
impl crate::RegisterSpec for DAC_TG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_tg::R](R) reader structure"]
impl crate::Readable for DAC_TG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_tg::W](W) writer structure"]
impl crate::Writable for DAC_TG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_TG to value 0"]
impl crate::Resettable for DAC_TG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
