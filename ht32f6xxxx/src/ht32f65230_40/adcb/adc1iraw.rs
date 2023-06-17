#[doc = "Register `ADC1IRAW` reader"]
pub struct R(crate::R<ADC1IRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1IRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1IRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1IRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1IRAW` writer"]
pub struct W(crate::W<ADC1IRAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1IRAW_SPEC>;
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
impl From<crate::W<ADC1IRAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1IRAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1IRAWS` reader - AD1IRAWS"]
pub type AD1IRAWS_R = crate::BitReader;
#[doc = "Field `AD1IRAWS` writer - AD1IRAWS"]
pub type AD1IRAWS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWG` reader - AD1IRAWG"]
pub type AD1IRAWG_R = crate::BitReader;
#[doc = "Field `AD1IRAWG` writer - AD1IRAWG"]
pub type AD1IRAWG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWC` reader - AD1IRAWC"]
pub type AD1IRAWC_R = crate::BitReader;
#[doc = "Field `AD1IRAWC` writer - AD1IRAWC"]
pub type AD1IRAWC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWHS` reader - AD1IRAWHS"]
pub type AD1IRAWHS_R = crate::BitReader;
#[doc = "Field `AD1IRAWHS` writer - AD1IRAWHS"]
pub type AD1IRAWHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWHG` reader - AD1IRAWHG"]
pub type AD1IRAWHG_R = crate::BitReader;
#[doc = "Field `AD1IRAWHG` writer - AD1IRAWHG"]
pub type AD1IRAWHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWHC` reader - AD1IRAWHC"]
pub type AD1IRAWHC_R = crate::BitReader;
#[doc = "Field `AD1IRAWHC` writer - AD1IRAWHC"]
pub type AD1IRAWHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWL` reader - AD1IRAWL"]
pub type AD1IRAWL_R = crate::BitReader;
#[doc = "Field `AD1IRAWL` writer - AD1IRAWL"]
pub type AD1IRAWL_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWU` reader - AD1IRAWU"]
pub type AD1IRAWU_R = crate::BitReader;
#[doc = "Field `AD1IRAWU` writer - AD1IRAWU"]
pub type AD1IRAWU_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWO` reader - AD1IRAWO"]
pub type AD1IRAWO_R = crate::BitReader;
#[doc = "Field `AD1IRAWO` writer - AD1IRAWO"]
pub type AD1IRAWO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
#[doc = "Field `AD1IRAWHO` reader - AD1IRAWHO"]
pub type AD1IRAWHO_R = crate::BitReader;
#[doc = "Field `AD1IRAWHO` writer - AD1IRAWHO"]
pub type AD1IRAWHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC1IRAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1IRAWS"]
    #[inline(always)]
    pub fn ad1iraws(&self) -> AD1IRAWS_R {
        AD1IRAWS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1IRAWG"]
    #[inline(always)]
    pub fn ad1irawg(&self) -> AD1IRAWG_R {
        AD1IRAWG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1IRAWC"]
    #[inline(always)]
    pub fn ad1irawc(&self) -> AD1IRAWC_R {
        AD1IRAWC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD1IRAWHS"]
    #[inline(always)]
    pub fn ad1irawhs(&self) -> AD1IRAWHS_R {
        AD1IRAWHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD1IRAWHG"]
    #[inline(always)]
    pub fn ad1irawhg(&self) -> AD1IRAWHG_R {
        AD1IRAWHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD1IRAWHC"]
    #[inline(always)]
    pub fn ad1irawhc(&self) -> AD1IRAWHC_R {
        AD1IRAWHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD1IRAWL"]
    #[inline(always)]
    pub fn ad1irawl(&self) -> AD1IRAWL_R {
        AD1IRAWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD1IRAWU"]
    #[inline(always)]
    pub fn ad1irawu(&self) -> AD1IRAWU_R {
        AD1IRAWU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD1IRAWO"]
    #[inline(always)]
    pub fn ad1irawo(&self) -> AD1IRAWO_R {
        AD1IRAWO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD1IRAWHO"]
    #[inline(always)]
    pub fn ad1irawho(&self) -> AD1IRAWHO_R {
        AD1IRAWHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1IRAWS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1iraws(&mut self) -> AD1IRAWS_W<0> {
        AD1IRAWS_W::new(self)
    }
    #[doc = "Bit 1 - AD1IRAWG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawg(&mut self) -> AD1IRAWG_W<1> {
        AD1IRAWG_W::new(self)
    }
    #[doc = "Bit 2 - AD1IRAWC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawc(&mut self) -> AD1IRAWC_W<2> {
        AD1IRAWC_W::new(self)
    }
    #[doc = "Bit 8 - AD1IRAWHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawhs(&mut self) -> AD1IRAWHS_W<8> {
        AD1IRAWHS_W::new(self)
    }
    #[doc = "Bit 9 - AD1IRAWHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawhg(&mut self) -> AD1IRAWHG_W<9> {
        AD1IRAWHG_W::new(self)
    }
    #[doc = "Bit 10 - AD1IRAWHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawhc(&mut self) -> AD1IRAWHC_W<10> {
        AD1IRAWHC_W::new(self)
    }
    #[doc = "Bit 16 - AD1IRAWL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawl(&mut self) -> AD1IRAWL_W<16> {
        AD1IRAWL_W::new(self)
    }
    #[doc = "Bit 17 - AD1IRAWU"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawu(&mut self) -> AD1IRAWU_W<17> {
        AD1IRAWU_W::new(self)
    }
    #[doc = "Bit 24 - AD1IRAWO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawo(&mut self) -> AD1IRAWO_W<24> {
        AD1IRAWO_W::new(self)
    }
    #[doc = "Bit 25 - AD1IRAWHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad1irawho(&mut self) -> AD1IRAWHO_W<25> {
        AD1IRAWHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1IRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1iraw](index.html) module"]
pub struct ADC1IRAW_SPEC;
impl crate::RegisterSpec for ADC1IRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1iraw::R](R) reader structure"]
impl crate::Readable for ADC1IRAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1iraw::W](W) writer structure"]
impl crate::Writable for ADC1IRAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1IRAW to value 0"]
impl crate::Resettable for ADC1IRAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
