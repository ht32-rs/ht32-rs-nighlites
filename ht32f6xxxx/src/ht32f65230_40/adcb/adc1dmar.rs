#[doc = "Register `ADC1DMAR` reader"]
pub struct R(crate::R<ADC1DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1DMAR` writer"]
pub struct W(crate::W<ADC1DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1DMAR_SPEC>;
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
impl From<crate::W<ADC1DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1DMAS` reader - AD1DMAS"]
pub type AD1DMAS_R = crate::BitReader;
#[doc = "Field `AD1DMAS` writer - AD1DMAS"]
pub type AD1DMAS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1DMAR_SPEC, O>;
#[doc = "Field `AD1DMAG` reader - AD1DMAG"]
pub type AD1DMAG_R = crate::BitReader;
#[doc = "Field `AD1DMAG` writer - AD1DMAG"]
pub type AD1DMAG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1DMAR_SPEC, O>;
#[doc = "Field `AD1DMAC` reader - AD1DMAC"]
pub type AD1DMAC_R = crate::BitReader;
#[doc = "Field `AD1DMAC` writer - AD1DMAC"]
pub type AD1DMAC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1DMAR_SPEC, O>;
#[doc = "Field `AD1DMAHS` reader - AD1DMAHS"]
pub type AD1DMAHS_R = crate::BitReader;
#[doc = "Field `AD1DMAHS` writer - AD1DMAHS"]
pub type AD1DMAHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1DMAR_SPEC, O>;
#[doc = "Field `AD1DMAHG` reader - AD1DMAHG"]
pub type AD1DMAHG_R = crate::BitReader;
#[doc = "Field `AD1DMAHG` writer - AD1DMAHG"]
pub type AD1DMAHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1DMAR_SPEC, O>;
#[doc = "Field `AD1DMAHC` reader - AD1DMAHC"]
pub type AD1DMAHC_R = crate::BitReader;
#[doc = "Field `AD1DMAHC` writer - AD1DMAHC"]
pub type AD1DMAHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1DMAR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1DMAS"]
    #[inline(always)]
    pub fn ad1dmas(&self) -> AD1DMAS_R {
        AD1DMAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1DMAG"]
    #[inline(always)]
    pub fn ad1dmag(&self) -> AD1DMAG_R {
        AD1DMAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1DMAC"]
    #[inline(always)]
    pub fn ad1dmac(&self) -> AD1DMAC_R {
        AD1DMAC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD1DMAHS"]
    #[inline(always)]
    pub fn ad1dmahs(&self) -> AD1DMAHS_R {
        AD1DMAHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD1DMAHG"]
    #[inline(always)]
    pub fn ad1dmahg(&self) -> AD1DMAHG_R {
        AD1DMAHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD1DMAHC"]
    #[inline(always)]
    pub fn ad1dmahc(&self) -> AD1DMAHC_R {
        AD1DMAHC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1DMAS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1dmas(&mut self) -> AD1DMAS_W<0> {
        AD1DMAS_W::new(self)
    }
    #[doc = "Bit 1 - AD1DMAG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1dmag(&mut self) -> AD1DMAG_W<1> {
        AD1DMAG_W::new(self)
    }
    #[doc = "Bit 2 - AD1DMAC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1dmac(&mut self) -> AD1DMAC_W<2> {
        AD1DMAC_W::new(self)
    }
    #[doc = "Bit 8 - AD1DMAHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1dmahs(&mut self) -> AD1DMAHS_W<8> {
        AD1DMAHS_W::new(self)
    }
    #[doc = "Bit 9 - AD1DMAHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1dmahg(&mut self) -> AD1DMAHG_W<9> {
        AD1DMAHG_W::new(self)
    }
    #[doc = "Bit 10 - AD1DMAHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1dmahc(&mut self) -> AD1DMAHC_W<10> {
        AD1DMAHC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1DMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1dmar](index.html) module"]
pub struct ADC1DMAR_SPEC;
impl crate::RegisterSpec for ADC1DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1dmar::R](R) reader structure"]
impl crate::Readable for ADC1DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1dmar::W](W) writer structure"]
impl crate::Writable for ADC1DMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1DMAR to value 0"]
impl crate::Resettable for ADC1DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
