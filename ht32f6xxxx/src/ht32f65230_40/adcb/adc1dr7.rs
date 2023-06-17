#[doc = "Register `ADC1DR7` reader"]
pub struct R(crate::R<ADC1DR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1DR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1DR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1DR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1DR7` writer"]
pub struct W(crate::W<ADC1DR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1DR7_SPEC>;
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
impl From<crate::W<ADC1DR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1DR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1D` reader - AD1D"]
pub type AD1D_R = crate::FieldReader<u16>;
#[doc = "Field `AD1D` writer - AD1D"]
pub type AD1D_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1DR7_SPEC, 16, O, u16>;
#[doc = "Field `AD1VLD` reader - AD1VLD"]
pub type AD1VLD_R = crate::BitReader;
#[doc = "Field `AD1VLD` writer - AD1VLD"]
pub type AD1VLD_W<'a, const O: u8> = crate::BitWriter<'a, ADC1DR7_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - AD1D"]
    #[inline(always)]
    pub fn ad1d(&self) -> AD1D_R {
        AD1D_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - AD1VLD"]
    #[inline(always)]
    pub fn ad1vld(&self) -> AD1VLD_R {
        AD1VLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - AD1D"]
    #[inline(always)]
    #[must_use]
    pub fn ad1d(&mut self) -> AD1D_W<0> {
        AD1D_W::new(self)
    }
    #[doc = "Bit 31 - AD1VLD"]
    #[inline(always)]
    #[must_use]
    pub fn ad1vld(&mut self) -> AD1VLD_W<31> {
        AD1VLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1DR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1dr7](index.html) module"]
pub struct ADC1DR7_SPEC;
impl crate::RegisterSpec for ADC1DR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1dr7::R](R) reader structure"]
impl crate::Readable for ADC1DR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1dr7::W](W) writer structure"]
impl crate::Writable for ADC1DR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1DR7 to value 0"]
impl crate::Resettable for ADC1DR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
