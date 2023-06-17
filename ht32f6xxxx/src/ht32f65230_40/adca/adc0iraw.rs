#[doc = "Register `ADC0IRAW` reader"]
pub struct R(crate::R<ADC0IRAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0IRAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0IRAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0IRAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0IRAW` writer"]
pub struct W(crate::W<ADC0IRAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0IRAW_SPEC>;
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
impl From<crate::W<ADC0IRAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0IRAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0IRAWS` reader - AD0IRAWS"]
pub type AD0IRAWS_R = crate::BitReader;
#[doc = "Field `AD0IRAWS` writer - AD0IRAWS"]
pub type AD0IRAWS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWG` reader - AD0IRAWG"]
pub type AD0IRAWG_R = crate::BitReader;
#[doc = "Field `AD0IRAWG` writer - AD0IRAWG"]
pub type AD0IRAWG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWC` reader - AD0IRAWC"]
pub type AD0IRAWC_R = crate::BitReader;
#[doc = "Field `AD0IRAWC` writer - AD0IRAWC"]
pub type AD0IRAWC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWHS` reader - AD0IRAWHS"]
pub type AD0IRAWHS_R = crate::BitReader;
#[doc = "Field `AD0IRAWHS` writer - AD0IRAWHS"]
pub type AD0IRAWHS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWHG` reader - AD0IRAWHG"]
pub type AD0IRAWHG_R = crate::BitReader;
#[doc = "Field `AD0IRAWHG` writer - AD0IRAWHG"]
pub type AD0IRAWHG_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWHC` reader - AD0IRAWHC"]
pub type AD0IRAWHC_R = crate::BitReader;
#[doc = "Field `AD0IRAWHC` writer - AD0IRAWHC"]
pub type AD0IRAWHC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWL` reader - AD0IRAWL"]
pub type AD0IRAWL_R = crate::BitReader;
#[doc = "Field `AD0IRAWL` writer - AD0IRAWL"]
pub type AD0IRAWL_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWU` reader - AD0IRAWU"]
pub type AD0IRAWU_R = crate::BitReader;
#[doc = "Field `AD0IRAWU` writer - AD0IRAWU"]
pub type AD0IRAWU_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWO` reader - AD0IRAWO"]
pub type AD0IRAWO_R = crate::BitReader;
#[doc = "Field `AD0IRAWO` writer - AD0IRAWO"]
pub type AD0IRAWO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
#[doc = "Field `AD0IRAWHO` reader - AD0IRAWHO"]
pub type AD0IRAWHO_R = crate::BitReader;
#[doc = "Field `AD0IRAWHO` writer - AD0IRAWHO"]
pub type AD0IRAWHO_W<'a, const O: u8> = crate::BitWriter<'a, ADC0IRAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0IRAWS"]
    #[inline(always)]
    pub fn ad0iraws(&self) -> AD0IRAWS_R {
        AD0IRAWS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0IRAWG"]
    #[inline(always)]
    pub fn ad0irawg(&self) -> AD0IRAWG_R {
        AD0IRAWG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0IRAWC"]
    #[inline(always)]
    pub fn ad0irawc(&self) -> AD0IRAWC_R {
        AD0IRAWC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - AD0IRAWHS"]
    #[inline(always)]
    pub fn ad0irawhs(&self) -> AD0IRAWHS_R {
        AD0IRAWHS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD0IRAWHG"]
    #[inline(always)]
    pub fn ad0irawhg(&self) -> AD0IRAWHG_R {
        AD0IRAWHG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD0IRAWHC"]
    #[inline(always)]
    pub fn ad0irawhc(&self) -> AD0IRAWHC_R {
        AD0IRAWHC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - AD0IRAWL"]
    #[inline(always)]
    pub fn ad0irawl(&self) -> AD0IRAWL_R {
        AD0IRAWL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD0IRAWU"]
    #[inline(always)]
    pub fn ad0irawu(&self) -> AD0IRAWU_R {
        AD0IRAWU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AD0IRAWO"]
    #[inline(always)]
    pub fn ad0irawo(&self) -> AD0IRAWO_R {
        AD0IRAWO_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD0IRAWHO"]
    #[inline(always)]
    pub fn ad0irawho(&self) -> AD0IRAWHO_R {
        AD0IRAWHO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0IRAWS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0iraws(&mut self) -> AD0IRAWS_W<0> {
        AD0IRAWS_W::new(self)
    }
    #[doc = "Bit 1 - AD0IRAWG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawg(&mut self) -> AD0IRAWG_W<1> {
        AD0IRAWG_W::new(self)
    }
    #[doc = "Bit 2 - AD0IRAWC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawc(&mut self) -> AD0IRAWC_W<2> {
        AD0IRAWC_W::new(self)
    }
    #[doc = "Bit 8 - AD0IRAWHS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawhs(&mut self) -> AD0IRAWHS_W<8> {
        AD0IRAWHS_W::new(self)
    }
    #[doc = "Bit 9 - AD0IRAWHG"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawhg(&mut self) -> AD0IRAWHG_W<9> {
        AD0IRAWHG_W::new(self)
    }
    #[doc = "Bit 10 - AD0IRAWHC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawhc(&mut self) -> AD0IRAWHC_W<10> {
        AD0IRAWHC_W::new(self)
    }
    #[doc = "Bit 16 - AD0IRAWL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawl(&mut self) -> AD0IRAWL_W<16> {
        AD0IRAWL_W::new(self)
    }
    #[doc = "Bit 17 - AD0IRAWU"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawu(&mut self) -> AD0IRAWU_W<17> {
        AD0IRAWU_W::new(self)
    }
    #[doc = "Bit 24 - AD0IRAWO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawo(&mut self) -> AD0IRAWO_W<24> {
        AD0IRAWO_W::new(self)
    }
    #[doc = "Bit 25 - AD0IRAWHO"]
    #[inline(always)]
    #[must_use]
    pub fn ad0irawho(&mut self) -> AD0IRAWHO_W<25> {
        AD0IRAWHO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0IRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0iraw](index.html) module"]
pub struct ADC0IRAW_SPEC;
impl crate::RegisterSpec for ADC0IRAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0iraw::R](R) reader structure"]
impl crate::Readable for ADC0IRAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0iraw::W](W) writer structure"]
impl crate::Writable for ADC0IRAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0IRAW to value 0"]
impl crate::Resettable for ADC0IRAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
