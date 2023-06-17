#[doc = "Register `ADC0ICLR` reader"]
pub struct R(crate::R<ADC0ICLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0ICLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0ICLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0ICLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0ICLR` writer"]
pub struct W(crate::W<ADC0ICLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0ICLR_SPEC>;
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
impl From<crate::W<ADC0ICLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0ICLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0ICLRS` reader - AD0ICLRS"]
pub type AD0ICLRS_R = crate::BitReader;
#[doc = "Field `AD0ICLRS` writer - AD0ICLRS"]
pub type AD0ICLRS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRG` reader - AD0ICLRG"]
pub type AD0ICLRG_R = crate::BitReader;
#[doc = "Field `AD0ICLRG` writer - AD0ICLRG"]
pub type AD0ICLRG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRC` reader - AD0ICLRC"]
pub type AD0ICLRC_R = crate::BitReader;
#[doc = "Field `AD0ICLRC` writer - AD0ICLRC"]
pub type AD0ICLRC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRHS` reader - AD0ICLRHS"]
pub type AD0ICLRHS_R = crate::BitReader;
#[doc = "Field `AD0ICLRHS` writer - AD0ICLRHS"]
pub type AD0ICLRHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRHG` reader - AD0ICLRHG"]
pub type AD0ICLRHG_R = crate::BitReader;
#[doc = "Field `AD0ICLRHG` writer - AD0ICLRHG"]
pub type AD0ICLRHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRHC` reader - AD0ICLRHC"]
pub type AD0ICLRHC_R = crate::BitReader;
#[doc = "Field `AD0ICLRHC` writer - AD0ICLRHC"]
pub type AD0ICLRHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRL` reader - AD0ICLRL"]
pub type AD0ICLRL_R = crate::BitReader;
#[doc = "Field `AD0ICLRL` writer - AD0ICLRL"]
pub type AD0ICLRL_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRU` reader - AD0ICLRU"]
pub type AD0ICLRU_R = crate::BitReader;
#[doc = "Field `AD0ICLRU` writer - AD0ICLRU"]
pub type AD0ICLRU_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRO` reader - AD0ICLRO"]
pub type AD0ICLRO_R = crate::BitReader;
#[doc = "Field `AD0ICLRO` writer - AD0ICLRO"]
pub type AD0ICLRO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
#[doc = "Field `AD0ICLRHO` reader - AD0ICLRHO"]
pub type AD0ICLRHO_R = crate::BitReader;
#[doc = "Field `AD0ICLRHO` writer - AD0ICLRHO"]
pub type AD0ICLRHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0ICLR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0ICLRS"]
    #[inline(always)]
    pub fn ad0iclrs(&self) -> AD0ICLRS_R {
        AD0ICLRS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0ICLRG"]
    #[inline(always)]
    pub fn ad0iclrg(&self) -> AD0ICLRG_R {
        AD0ICLRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0ICLRC"]
    #[inline(always)]
    pub fn ad0iclrc(&self) -> AD0ICLRC_R {
        AD0ICLRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD0ICLRHS"]
    #[inline(always)]
    pub fn ad0iclrhs(&self) -> AD0ICLRHS_R {
        AD0ICLRHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD0ICLRHG"]
    #[inline(always)]
    pub fn ad0iclrhg(&self) -> AD0ICLRHG_R {
        AD0ICLRHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD0ICLRHC"]
    #[inline(always)]
    pub fn ad0iclrhc(&self) -> AD0ICLRHC_R {
        AD0ICLRHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD0ICLRL"]
    #[inline(always)]
    pub fn ad0iclrl(&self) -> AD0ICLRL_R {
        AD0ICLRL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD0ICLRU"]
    #[inline(always)]
    pub fn ad0iclru(&self) -> AD0ICLRU_R {
        AD0ICLRU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD0ICLRO"]
    #[inline(always)]
    pub fn ad0iclro(&self) -> AD0ICLRO_R {
        AD0ICLRO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD0ICLRHO"]
    #[inline(always)]
    pub fn ad0iclrho(&self) -> AD0ICLRHO_R {
        AD0ICLRHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0ICLRS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrs(&mut self) -> AD0ICLRS_W<0> {
        AD0ICLRS_W::new(self)
    }
    #[doc = "Bit 1 - AD0ICLRG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrg(&mut self) -> AD0ICLRG_W<1> {
        AD0ICLRG_W::new(self)
    }
    #[doc = "Bit 2 - AD0ICLRC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrc(&mut self) -> AD0ICLRC_W<2> {
        AD0ICLRC_W::new(self)
    }
    #[doc = "Bit 8 - AD0ICLRHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrhs(&mut self) -> AD0ICLRHS_W<8> {
        AD0ICLRHS_W::new(self)
    }
    #[doc = "Bit 9 - AD0ICLRHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrhg(&mut self) -> AD0ICLRHG_W<9> {
        AD0ICLRHG_W::new(self)
    }
    #[doc = "Bit 10 - AD0ICLRHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrhc(&mut self) -> AD0ICLRHC_W<10> {
        AD0ICLRHC_W::new(self)
    }
    #[doc = "Bit 16 - AD0ICLRL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrl(&mut self) -> AD0ICLRL_W<16> {
        AD0ICLRL_W::new(self)
    }
    #[doc = "Bit 17 - AD0ICLRU"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclru(&mut self) -> AD0ICLRU_W<17> {
        AD0ICLRU_W::new(self)
    }
    #[doc = "Bit 24 - AD0ICLRO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclro(&mut self) -> AD0ICLRO_W<24> {
        AD0ICLRO_W::new(self)
    }
    #[doc = "Bit 25 - AD0ICLRHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iclrho(&mut self) -> AD0ICLRHO_W<25> {
        AD0ICLRHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0iclr](index.html) module"]
pub struct ADC0ICLR_SPEC;
impl crate::RegisterSpec for ADC0ICLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0iclr::R](R) reader structure"]
impl crate::Readable for ADC0ICLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0iclr::W](W) writer structure"]
impl crate::Writable for ADC0ICLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0ICLR to value 0"]
impl crate::Resettable for ADC0ICLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
