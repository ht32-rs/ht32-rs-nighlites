#[doc = "Register `ADC1WCR` reader"]
pub struct R(crate::R<ADC1WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1WCR` writer"]
pub struct W(crate::W<ADC1WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1WCR_SPEC>;
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
impl From<crate::W<ADC1WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1WLE` reader - AD1WLE"]
pub type AD1WLE_R = crate::BitReader;
#[doc = "Field `AD1WLE` writer - AD1WLE"]
pub type AD1WLE_W<'a, const O: u8> = crate::BitWriter<'a, ADC1WCR_SPEC, O>;
#[doc = "Field `AD1WUE` reader - AD1WUE"]
pub type AD1WUE_R = crate::BitReader;
#[doc = "Field `AD1WUE` writer - AD1WUE"]
pub type AD1WUE_W<'a, const O: u8> = crate::BitWriter<'a, ADC1WCR_SPEC, O>;
#[doc = "Field `AD1WALL` reader - AD1WALL"]
pub type AD1WALL_R = crate::BitReader;
#[doc = "Field `AD1WALL` writer - AD1WALL"]
pub type AD1WALL_W<'a, const O: u8> = crate::BitWriter<'a, ADC1WCR_SPEC, O>;
#[doc = "Field `AD1WCH` reader - AD1WCH"]
pub type AD1WCH_R = crate::FieldReader;
#[doc = "Field `AD1WCH` writer - AD1WCH"]
pub type AD1WCH_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1WCR_SPEC, 4, O>;
#[doc = "Field `AD1LCH` reader - AD1LCH"]
pub type AD1LCH_R = crate::FieldReader;
#[doc = "Field `AD1LCH` writer - AD1LCH"]
pub type AD1LCH_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1WCR_SPEC, 4, O>;
#[doc = "Field `AD1UCH` reader - AD1UCH"]
pub type AD1UCH_R = crate::FieldReader;
#[doc = "Field `AD1UCH` writer - AD1UCH"]
pub type AD1UCH_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1WCR_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - AD1WLE"]
    #[inline(always)]
    pub fn ad1wle(&self) -> AD1WLE_R {
        AD1WLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1WUE"]
    #[inline(always)]
    pub fn ad1wue(&self) -> AD1WUE_R {
        AD1WUE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1WALL"]
    #[inline(always)]
    pub fn ad1wall(&self) -> AD1WALL_R {
        AD1WALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD1WCH"]
    #[inline(always)]
    pub fn ad1wch(&self) -> AD1WCH_R {
        AD1WCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AD1LCH"]
    #[inline(always)]
    pub fn ad1lch(&self) -> AD1LCH_R {
        AD1LCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AD1UCH"]
    #[inline(always)]
    pub fn ad1uch(&self) -> AD1UCH_R {
        AD1UCH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AD1WLE"]
    #[inline(always)]
    #[must_use]
    pub fn ad1wle(&mut self) -> AD1WLE_W<0> {
        AD1WLE_W::new(self)
    }
    #[doc = "Bit 1 - AD1WUE"]
    #[inline(always)]
    #[must_use]
    pub fn ad1wue(&mut self) -> AD1WUE_W<1> {
        AD1WUE_W::new(self)
    }
    #[doc = "Bit 2 - AD1WALL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1wall(&mut self) -> AD1WALL_W<2> {
        AD1WALL_W::new(self)
    }
    #[doc = "Bits 8:11 - AD1WCH"]
    #[inline(always)]
    #[must_use]
    pub fn ad1wch(&mut self) -> AD1WCH_W<8> {
        AD1WCH_W::new(self)
    }
    #[doc = "Bits 16:19 - AD1LCH"]
    #[inline(always)]
    #[must_use]
    pub fn ad1lch(&mut self) -> AD1LCH_W<16> {
        AD1LCH_W::new(self)
    }
    #[doc = "Bits 24:27 - AD1UCH"]
    #[inline(always)]
    #[must_use]
    pub fn ad1uch(&mut self) -> AD1UCH_W<24> {
        AD1UCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1WCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1wcr](index.html) module"]
pub struct ADC1WCR_SPEC;
impl crate::RegisterSpec for ADC1WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1wcr::R](R) reader structure"]
impl crate::Readable for ADC1WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1wcr::W](W) writer structure"]
impl crate::Writable for ADC1WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1WCR to value 0"]
impl crate::Resettable for ADC1WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
