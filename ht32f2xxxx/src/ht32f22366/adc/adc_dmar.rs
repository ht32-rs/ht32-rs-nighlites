#[doc = "Register `ADC_DMAR` reader"]
pub struct R(crate::R<ADC_DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_DMAR` writer"]
pub struct W(crate::W<ADC_DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_DMAR_SPEC>;
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
impl From<crate::W<ADC_DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDMAS` reader - ADDMAS"]
pub type ADDMAS_R = crate::BitReader;
#[doc = "Field `ADDMAS` writer - ADDMAS"]
pub type ADDMAS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DMAR_SPEC, O>;
#[doc = "Field `ADDMAG` reader - ADDMAG"]
pub type ADDMAG_R = crate::BitReader;
#[doc = "Field `ADDMAG` writer - ADDMAG"]
pub type ADDMAG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DMAR_SPEC, O>;
#[doc = "Field `ADDMAC` reader - ADDMAC"]
pub type ADDMAC_R = crate::BitReader;
#[doc = "Field `ADDMAC` writer - ADDMAC"]
pub type ADDMAC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DMAR_SPEC, O>;
#[doc = "Field `ADDMAHS` reader - ADDMAHS"]
pub type ADDMAHS_R = crate::BitReader;
#[doc = "Field `ADDMAHS` writer - ADDMAHS"]
pub type ADDMAHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DMAR_SPEC, O>;
#[doc = "Field `ADDMAHG` reader - ADDMAHG"]
pub type ADDMAHG_R = crate::BitReader;
#[doc = "Field `ADDMAHG` writer - ADDMAHG"]
pub type ADDMAHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DMAR_SPEC, O>;
#[doc = "Field `ADDMAHC` reader - ADDMAHC"]
pub type ADDMAHC_R = crate::BitReader;
#[doc = "Field `ADDMAHC` writer - ADDMAHC"]
pub type ADDMAHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_DMAR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    pub fn addmas(&self) -> ADDMAS_R {
        ADDMAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    pub fn addmag(&self) -> ADDMAG_R {
        ADDMAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    pub fn addmac(&self) -> ADDMAC_R {
        ADDMAC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - ADDMAHS"]
    #[inline(always)]
    pub fn addmahs(&self) -> ADDMAHS_R {
        ADDMAHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADDMAHG"]
    #[inline(always)]
    pub fn addmahg(&self) -> ADDMAHG_R {
        ADDMAHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADDMAHC"]
    #[inline(always)]
    pub fn addmahc(&self) -> ADDMAHC_R {
        ADDMAHC_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADDMAS"]
    #[inline(always)]
    #[must_use]
    pub fn addmas(&mut self) -> ADDMAS_W<0> {
        ADDMAS_W::new(self)
    }
    #[doc = "Bit 1 - ADDMAG"]
    #[inline(always)]
    #[must_use]
    pub fn addmag(&mut self) -> ADDMAG_W<1> {
        ADDMAG_W::new(self)
    }
    #[doc = "Bit 2 - ADDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn addmac(&mut self) -> ADDMAC_W<2> {
        ADDMAC_W::new(self)
    }
    #[doc = "Bit 8 - ADDMAHS"]
    #[inline(always)]
    #[must_use]
    pub fn addmahs(&mut self) -> ADDMAHS_W<8> {
        ADDMAHS_W::new(self)
    }
    #[doc = "Bit 9 - ADDMAHG"]
    #[inline(always)]
    #[must_use]
    pub fn addmahg(&mut self) -> ADDMAHG_W<9> {
        ADDMAHG_W::new(self)
    }
    #[doc = "Bit 10 - ADDMAHC"]
    #[inline(always)]
    #[must_use]
    pub fn addmahc(&mut self) -> ADDMAHC_W<10> {
        ADDMAHC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_DMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dmar](index.html) module"]
pub struct ADC_DMAR_SPEC;
impl crate::RegisterSpec for ADC_DMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dmar::R](R) reader structure"]
impl crate::Readable for ADC_DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_dmar::W](W) writer structure"]
impl crate::Writable for ADC_DMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_DMAR to value 0"]
impl crate::Resettable for ADC_DMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
