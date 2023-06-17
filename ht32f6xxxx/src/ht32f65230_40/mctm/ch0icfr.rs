#[doc = "Register `CH0ICFR` reader"]
pub struct R(crate::R<CH0ICFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0ICFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0ICFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0ICFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0ICFR` writer"]
pub struct W(crate::W<CH0ICFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0ICFR_SPEC>;
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
impl From<crate::W<CH0ICFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0ICFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI0FN` reader - TI0FN"]
pub type TI0FN_R = crate::FieldReader;
#[doc = "Field `TI0FN` writer - TI0FN"]
pub type TI0FN_W<'a, const O: u8> = crate::FieldWriter<'a, CH0ICFR_SPEC, 3, O>;
#[doc = "Field `TI0FF` reader - TI0FF"]
pub type TI0FF_R = crate::FieldReader;
#[doc = "Field `TI0FF` writer - TI0FF"]
pub type TI0FF_W<'a, const O: u8> = crate::FieldWriter<'a, CH0ICFR_SPEC, 3, O>;
#[doc = "Field `CH0CCS` reader - CH0CCS"]
pub type CH0CCS_R = crate::FieldReader;
#[doc = "Field `CH0CCS` writer - CH0CCS"]
pub type CH0CCS_W<'a, const O: u8> = crate::FieldWriter<'a, CH0ICFR_SPEC, 2, O>;
#[doc = "Field `CH0PSC` reader - CH0PSC"]
pub type CH0PSC_R = crate::FieldReader;
#[doc = "Field `CH0PSC` writer - CH0PSC"]
pub type CH0PSC_W<'a, const O: u8> = crate::FieldWriter<'a, CH0ICFR_SPEC, 2, O>;
#[doc = "Field `TI0SRC` reader - TI0SRC"]
pub type TI0SRC_R = crate::BitReader;
#[doc = "Field `TI0SRC` writer - TI0SRC"]
pub type TI0SRC_W<'a, const O: u8> = crate::BitWriter<'a, CH0ICFR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - TI0FN"]
    #[inline(always)]
    pub fn ti0fn(&self) -> TI0FN_R {
        TI0FN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - TI0FF"]
    #[inline(always)]
    pub fn ti0ff(&self) -> TI0FF_R {
        TI0FF_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    pub fn ch0ccs(&self) -> CH0CCS_R {
        CH0CCS_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    pub fn ch0psc(&self) -> CH0PSC_R {
        CH0PSC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    pub fn ti0src(&self) -> TI0SRC_R {
        TI0SRC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - TI0FN"]
    #[inline(always)]
    #[must_use]
    pub fn ti0fn(&mut self) -> TI0FN_W<0> {
        TI0FN_W::new(self)
    }
    #[doc = "Bits 4:6 - TI0FF"]
    #[inline(always)]
    #[must_use]
    pub fn ti0ff(&mut self) -> TI0FF_W<4> {
        TI0FF_W::new(self)
    }
    #[doc = "Bits 16:17 - CH0CCS"]
    #[inline(always)]
    #[must_use]
    pub fn ch0ccs(&mut self) -> CH0CCS_W<16> {
        CH0CCS_W::new(self)
    }
    #[doc = "Bits 18:19 - CH0PSC"]
    #[inline(always)]
    #[must_use]
    pub fn ch0psc(&mut self) -> CH0PSC_W<18> {
        CH0PSC_W::new(self)
    }
    #[doc = "Bit 31 - TI0SRC"]
    #[inline(always)]
    #[must_use]
    pub fn ti0src(&mut self) -> TI0SRC_W<31> {
        TI0SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH0ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0icfr](index.html) module"]
pub struct CH0ICFR_SPEC;
impl crate::RegisterSpec for CH0ICFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0icfr::R](R) reader structure"]
impl crate::Readable for CH0ICFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0icfr::W](W) writer structure"]
impl crate::Writable for CH0ICFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0ICFR to value 0"]
impl crate::Resettable for CH0ICFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
