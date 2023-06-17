#[doc = "Register `ADC0DR6` reader"]
pub struct R(crate::R<ADC0DR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0DR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0DR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0DR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0DR6` writer"]
pub struct W(crate::W<ADC0DR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0DR6_SPEC>;
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
impl From<crate::W<ADC0DR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0DR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0D` reader - AD0D"]
pub type AD0D_R = crate::FieldReader<u16>;
#[doc = "Field `AD0D` writer - AD0D"]
pub type AD0D_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0DR6_SPEC, 16, O, u16>;
#[doc = "Field `AD0VLD` reader - AD0VLD"]
pub type AD0VLD_R = crate::BitReader;
#[doc = "Field `AD0VLD` writer - AD0VLD"]
pub type AD0VLD_W<'a, const O: u8> = crate::BitWriter<'a, ADC0DR6_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - AD0D"]
    #[inline(always)]
    pub fn ad0d(&self) -> AD0D_R {
        AD0D_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - AD0VLD"]
    #[inline(always)]
    pub fn ad0vld(&self) -> AD0VLD_R {
        AD0VLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - AD0D"]
    #[inline(always)]
    #[must_use]
    pub fn ad0d(&mut self) -> AD0D_W<0> {
        AD0D_W::new(self)
    }
    #[doc = "Bit 31 - AD0VLD"]
    #[inline(always)]
    #[must_use]
    pub fn ad0vld(&mut self) -> AD0VLD_W<31> {
        AD0VLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0DR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0dr6](index.html) module"]
pub struct ADC0DR6_SPEC;
impl crate::RegisterSpec for ADC0DR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0dr6::R](R) reader structure"]
impl crate::Readable for ADC0DR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0dr6::W](W) writer structure"]
impl crate::Writable for ADC0DR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0DR6 to value 0"]
impl crate::Resettable for ADC0DR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
