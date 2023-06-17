#[doc = "Register `ADC1ICLR` reader"]
pub struct R(crate::R<ADC1ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1ICLR` writer"]
pub struct W(crate::W<ADC1ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1ICLR_SPEC>;
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
impl From<crate::W<ADC1ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1ICLRS` reader - AD1ICLRS"]
pub type AD1ICLRS_R = crate::BitReader;
#[doc = "Field `AD1ICLRS` writer - AD1ICLRS"]
pub type AD1ICLRS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRG` reader - AD1ICLRG"]
pub type AD1ICLRG_R = crate::BitReader;
#[doc = "Field `AD1ICLRG` writer - AD1ICLRG"]
pub type AD1ICLRG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRC` reader - AD1ICLRC"]
pub type AD1ICLRC_R = crate::BitReader;
#[doc = "Field `AD1ICLRC` writer - AD1ICLRC"]
pub type AD1ICLRC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRHS` reader - AD1ICLRHS"]
pub type AD1ICLRHS_R = crate::BitReader;
#[doc = "Field `AD1ICLRHS` writer - AD1ICLRHS"]
pub type AD1ICLRHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRHG` reader - AD1ICLRHG"]
pub type AD1ICLRHG_R = crate::BitReader;
#[doc = "Field `AD1ICLRHG` writer - AD1ICLRHG"]
pub type AD1ICLRHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRHC` reader - AD1ICLRHC"]
pub type AD1ICLRHC_R = crate::BitReader;
#[doc = "Field `AD1ICLRHC` writer - AD1ICLRHC"]
pub type AD1ICLRHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRL` reader - AD1ICLRL"]
pub type AD1ICLRL_R = crate::BitReader;
#[doc = "Field `AD1ICLRL` writer - AD1ICLRL"]
pub type AD1ICLRL_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRU` reader - AD1ICLRU"]
pub type AD1ICLRU_R = crate::BitReader;
#[doc = "Field `AD1ICLRU` writer - AD1ICLRU"]
pub type AD1ICLRU_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRO` reader - AD1ICLRO"]
pub type AD1ICLRO_R = crate::BitReader;
#[doc = "Field `AD1ICLRO` writer - AD1ICLRO"]
pub type AD1ICLRO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
#[doc = "Field `AD1ICLRHO` reader - AD1ICLRHO"]
pub type AD1ICLRHO_R = crate::BitReader;
#[doc = "Field `AD1ICLRHO` writer - AD1ICLRHO"]
pub type AD1ICLRHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1ICLR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1ICLRS"]
    #[inline(always)]
    pub fn ad1iclrs(&self) -> AD1ICLRS_R {
        AD1ICLRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1ICLRG"]
    #[inline(always)]
    pub fn ad1iclrg(&self) -> AD1ICLRG_R {
        AD1ICLRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1ICLRC"]
    #[inline(always)]
    pub fn ad1iclrc(&self) -> AD1ICLRC_R {
        AD1ICLRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD1ICLRHS"]
    #[inline(always)]
    pub fn ad1iclrhs(&self) -> AD1ICLRHS_R {
        AD1ICLRHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD1ICLRHG"]
    #[inline(always)]
    pub fn ad1iclrhg(&self) -> AD1ICLRHG_R {
        AD1ICLRHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD1ICLRHC"]
    #[inline(always)]
    pub fn ad1iclrhc(&self) -> AD1ICLRHC_R {
        AD1ICLRHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD1ICLRL"]
    #[inline(always)]
    pub fn ad1iclrl(&self) -> AD1ICLRL_R {
        AD1ICLRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD1ICLRU"]
    #[inline(always)]
    pub fn ad1iclru(&self) -> AD1ICLRU_R {
        AD1ICLRU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD1ICLRO"]
    #[inline(always)]
    pub fn ad1iclro(&self) -> AD1ICLRO_R {
        AD1ICLRO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD1ICLRHO"]
    #[inline(always)]
    pub fn ad1iclrho(&self) -> AD1ICLRHO_R {
        AD1ICLRHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1ICLRS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrs(&mut self) -> AD1ICLRS_W<0> {
        AD1ICLRS_W::new(self)
    }
    #[doc = "Bit 1 - AD1ICLRG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrg(&mut self) -> AD1ICLRG_W<1> {
        AD1ICLRG_W::new(self)
    }
    #[doc = "Bit 2 - AD1ICLRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrc(&mut self) -> AD1ICLRC_W<2> {
        AD1ICLRC_W::new(self)
    }
    #[doc = "Bit 8 - AD1ICLRHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrhs(&mut self) -> AD1ICLRHS_W<8> {
        AD1ICLRHS_W::new(self)
    }
    #[doc = "Bit 9 - AD1ICLRHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrhg(&mut self) -> AD1ICLRHG_W<9> {
        AD1ICLRHG_W::new(self)
    }
    #[doc = "Bit 10 - AD1ICLRHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrhc(&mut self) -> AD1ICLRHC_W<10> {
        AD1ICLRHC_W::new(self)
    }
    #[doc = "Bit 16 - AD1ICLRL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrl(&mut self) -> AD1ICLRL_W<16> {
        AD1ICLRL_W::new(self)
    }
    #[doc = "Bit 17 - AD1ICLRU"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclru(&mut self) -> AD1ICLRU_W<17> {
        AD1ICLRU_W::new(self)
    }
    #[doc = "Bit 24 - AD1ICLRO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclro(&mut self) -> AD1ICLRO_W<24> {
        AD1ICLRO_W::new(self)
    }
    #[doc = "Bit 25 - AD1ICLRHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iclrho(&mut self) -> AD1ICLRHO_W<25> {
        AD1ICLRHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1iclr](index.html) module"]
pub struct ADC1ICLR_SPEC;
impl crate::RegisterSpec for ADC1ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1iclr::R](R) reader structure"]
impl crate::Readable for ADC1ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1iclr::W](W) writer structure"]
impl crate::Writable for ADC1ICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1ICLR to value 0"]
impl crate::Resettable for ADC1ICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
