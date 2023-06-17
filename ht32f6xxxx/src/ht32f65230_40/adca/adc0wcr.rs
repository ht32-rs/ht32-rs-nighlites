#[doc = "Register `ADC0WCR` reader"]
pub struct R(crate::R<ADC0WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0WCR` writer"]
pub struct W(crate::W<ADC0WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0WCR_SPEC>;
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
impl From<crate::W<ADC0WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0WLE` reader - AD0WLE"]
pub type AD0WLE_R = crate::BitReader;
#[doc = "Field `AD0WLE` writer - AD0WLE"]
pub type AD0WLE_W<'a, const O: u8> = crate::BitWriter<'a, ADC0WCR_SPEC, O>;
#[doc = "Field `AD0WUE` reader - AD0WUE"]
pub type AD0WUE_R = crate::BitReader;
#[doc = "Field `AD0WUE` writer - AD0WUE"]
pub type AD0WUE_W<'a, const O: u8> = crate::BitWriter<'a, ADC0WCR_SPEC, O>;
#[doc = "Field `AD0WALL` reader - AD0WALL"]
pub type AD0WALL_R = crate::BitReader;
#[doc = "Field `AD0WALL` writer - AD0WALL"]
pub type AD0WALL_W<'a, const O: u8> = crate::BitWriter<'a, ADC0WCR_SPEC, O>;
#[doc = "Field `AD0WCH` reader - AD0WCH"]
pub type AD0WCH_R = crate::FieldReader;
#[doc = "Field `AD0WCH` writer - AD0WCH"]
pub type AD0WCH_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0WCR_SPEC, 4, O>;
#[doc = "Field `AD0LCH` reader - AD0LCH"]
pub type AD0LCH_R = crate::FieldReader;
#[doc = "Field `AD0LCH` writer - AD0LCH"]
pub type AD0LCH_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0WCR_SPEC, 4, O>;
#[doc = "Field `AD0UCH` reader - AD0UCH"]
pub type AD0UCH_R = crate::FieldReader;
#[doc = "Field `AD0UCH` writer - AD0UCH"]
pub type AD0UCH_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0WCR_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - AD0WLE"]
    #[inline(always)]
    pub fn ad0wle(&self) -> AD0WLE_R {
        AD0WLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0WUE"]
    #[inline(always)]
    pub fn ad0wue(&self) -> AD0WUE_R {
        AD0WUE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0WALL"]
    #[inline(always)]
    pub fn ad0wall(&self) -> AD0WALL_R {
        AD0WALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD0WCH"]
    #[inline(always)]
    pub fn ad0wch(&self) -> AD0WCH_R {
        AD0WCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AD0LCH"]
    #[inline(always)]
    pub fn ad0lch(&self) -> AD0LCH_R {
        AD0LCH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AD0UCH"]
    #[inline(always)]
    pub fn ad0uch(&self) -> AD0UCH_R {
        AD0UCH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AD0WLE"]
    #[inline(always)]
    #[must_use]
    pub fn ad0wle(&mut self) -> AD0WLE_W<0> {
        AD0WLE_W::new(self)
    }
    #[doc = "Bit 1 - AD0WUE"]
    #[inline(always)]
    #[must_use]
    pub fn ad0wue(&mut self) -> AD0WUE_W<1> {
        AD0WUE_W::new(self)
    }
    #[doc = "Bit 2 - AD0WALL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0wall(&mut self) -> AD0WALL_W<2> {
        AD0WALL_W::new(self)
    }
    #[doc = "Bits 8:11 - AD0WCH"]
    #[inline(always)]
    #[must_use]
    pub fn ad0wch(&mut self) -> AD0WCH_W<8> {
        AD0WCH_W::new(self)
    }
    #[doc = "Bits 16:19 - AD0LCH"]
    #[inline(always)]
    #[must_use]
    pub fn ad0lch(&mut self) -> AD0LCH_W<16> {
        AD0LCH_W::new(self)
    }
    #[doc = "Bits 24:27 - AD0UCH"]
    #[inline(always)]
    #[must_use]
    pub fn ad0uch(&mut self) -> AD0UCH_W<24> {
        AD0UCH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0WCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0wcr](index.html) module"]
pub struct ADC0WCR_SPEC;
impl crate::RegisterSpec for ADC0WCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0wcr::R](R) reader structure"]
impl crate::Readable for ADC0WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0wcr::W](W) writer structure"]
impl crate::Writable for ADC0WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0WCR to value 0"]
impl crate::Resettable for ADC0WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
