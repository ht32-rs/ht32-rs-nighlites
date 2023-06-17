#[doc = "Register `ADC0DMAR` reader"]
pub struct R(crate::R<ADC0DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0DMAR` writer"]
pub struct W(crate::W<ADC0DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0DMAR_SPEC>;
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
impl From<crate::W<ADC0DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0DMAS` reader - AD0DMAS"]
pub type AD0DMAS_R = crate::BitReader;
#[doc = "Field `AD0DMAS` writer - AD0DMAS"]
pub type AD0DMAS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0DMAR_SPEC, O>;
#[doc = "Field `AD0DMAG` reader - AD0DMAG"]
pub type AD0DMAG_R = crate::BitReader;
#[doc = "Field `AD0DMAG` writer - AD0DMAG"]
pub type AD0DMAG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0DMAR_SPEC, O>;
#[doc = "Field `AD0DMAC` reader - AD0DMAC"]
pub type AD0DMAC_R = crate::BitReader;
#[doc = "Field `AD0DMAC` writer - AD0DMAC"]
pub type AD0DMAC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0DMAR_SPEC, O>;
#[doc = "Field `AD0DMAHS` reader - AD0DMAHS"]
pub type AD0DMAHS_R = crate::BitReader;
#[doc = "Field `AD0DMAHS` writer - AD0DMAHS"]
pub type AD0DMAHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0DMAR_SPEC, O>;
#[doc = "Field `AD0DMAHG` reader - AD0DMAHG"]
pub type AD0DMAHG_R = crate::BitReader;
#[doc = "Field `AD0DMAHG` writer - AD0DMAHG"]
pub type AD0DMAHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0DMAR_SPEC, O>;
#[doc = "Field `AD0DMAHC` reader - AD0DMAHC"]
pub type AD0DMAHC_R = crate::BitReader;
#[doc = "Field `AD0DMAHC` writer - AD0DMAHC"]
pub type AD0DMAHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0DMAR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0DMAS"]
    #[inline(always)]
    pub fn ad0dmas(&self) -> AD0DMAS_R {
        AD0DMAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0DMAG"]
    #[inline(always)]
    pub fn ad0dmag(&self) -> AD0DMAG_R {
        AD0DMAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0DMAC"]
    #[inline(always)]
    pub fn ad0dmac(&self) -> AD0DMAC_R {
        AD0DMAC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD0DMAHS"]
    #[inline(always)]
    pub fn ad0dmahs(&self) -> AD0DMAHS_R {
        AD0DMAHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD0DMAHG"]
    #[inline(always)]
    pub fn ad0dmahg(&self) -> AD0DMAHG_R {
        AD0DMAHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD0DMAHC"]
    #[inline(always)]
    pub fn ad0dmahc(&self) -> AD0DMAHC_R {
        AD0DMAHC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0DMAS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0dmas(&mut self) -> AD0DMAS_W<0> {
        AD0DMAS_W::new(self)
    }
    #[doc = "Bit 1 - AD0DMAG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0dmag(&mut self) -> AD0DMAG_W<1> {
        AD0DMAG_W::new(self)
    }
    #[doc = "Bit 2 - AD0DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0dmac(&mut self) -> AD0DMAC_W<2> {
        AD0DMAC_W::new(self)
    }
    #[doc = "Bit 8 - AD0DMAHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0dmahs(&mut self) -> AD0DMAHS_W<8> {
        AD0DMAHS_W::new(self)
    }
    #[doc = "Bit 9 - AD0DMAHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0dmahg(&mut self) -> AD0DMAHG_W<9> {
        AD0DMAHG_W::new(self)
    }
    #[doc = "Bit 10 - AD0DMAHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0dmahc(&mut self) -> AD0DMAHC_W<10> {
        AD0DMAHC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0DMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0dmar](index.html) module"]
pub struct ADC0DMAR_SPEC;
impl crate::RegisterSpec for ADC0DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0dmar::R](R) reader structure"]
impl crate::Readable for ADC0DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0dmar::W](W) writer structure"]
impl crate::Writable for ADC0DMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0DMAR to value 0"]
impl crate::Resettable for ADC0DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
