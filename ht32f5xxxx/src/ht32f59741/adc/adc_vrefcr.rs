#[doc = "Register `ADC_VREFCR` reader"]
pub struct R(crate::R<ADC_VREFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_VREFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_VREFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_VREFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_VREFCR` writer"]
pub struct W(crate::W<ADC_VREFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_VREFCR_SPEC>;
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
impl From<crate::W<ADC_VREFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_VREFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREFEN` reader - VREFEN"]
pub type VREFEN_R = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREFEN"]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, ADC_VREFCR_SPEC, O>;
#[doc = "Field `VREFSEL` reader - VREFSEL"]
pub type VREFSEL_R = crate::FieldReader;
#[doc = "Field `VREFSEL` writer - VREFSEL"]
pub type VREFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_VREFCR_SPEC, 2, O>;
#[doc = "Field `MVDDAEN` reader - MVDDAEN"]
pub type MVDDAEN_R = crate::BitReader;
#[doc = "Field `MVDDAEN` writer - MVDDAEN"]
pub type MVDDAEN_W<'a, const O: u8> = crate::BitWriter<'a, ADC_VREFCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - VREFEN"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - VREFSEL"]
    #[inline(always)]
    pub fn vrefsel(&self) -> VREFSEL_R {
        VREFSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - MVDDAEN"]
    #[inline(always)]
    pub fn mvddaen(&self) -> MVDDAEN_R {
        MVDDAEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<0> {
        VREFEN_W::new(self)
    }
    #[doc = "Bits 4:5 - VREFSEL"]
    #[inline(always)]
    #[must_use]
    pub fn vrefsel(&mut self) -> VREFSEL_W<4> {
        VREFSEL_W::new(self)
    }
    #[doc = "Bit 8 - MVDDAEN"]
    #[inline(always)]
    #[must_use]
    pub fn mvddaen(&mut self) -> MVDDAEN_W<8> {
        MVDDAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_VREFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_vrefcr](index.html) module"]
pub struct ADC_VREFCR_SPEC;
impl crate::RegisterSpec for ADC_VREFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_vrefcr::R](R) reader structure"]
impl crate::Readable for ADC_VREFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_vrefcr::W](W) writer structure"]
impl crate::Writable for ADC_VREFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_VREFCR to value 0"]
impl crate::Resettable for ADC_VREFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
