#[doc = "Register `GPIOD_RR` reader"]
pub struct R(crate::R<GPIOD_RR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_RR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_RR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_RR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOD_RR` writer"]
pub struct W(crate::W<GPIOD_RR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOD_RR_SPEC>;
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
impl From<crate::W<GPIOD_RR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOD_RR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RST0` reader - RST0"]
pub type RST0_R = crate::BitReader;
#[doc = "Field `RST0` writer - RST0"]
pub type RST0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_RR_SPEC, O>;
#[doc = "Field `RST1` reader - RST1"]
pub type RST1_R = crate::BitReader;
#[doc = "Field `RST1` writer - RST1"]
pub type RST1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_RR_SPEC, O>;
#[doc = "Field `RST2` reader - RST2"]
pub type RST2_R = crate::BitReader;
#[doc = "Field `RST2` writer - RST2"]
pub type RST2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_RR_SPEC, O>;
#[doc = "Field `RST3` reader - RST3"]
pub type RST3_R = crate::BitReader;
#[doc = "Field `RST3` writer - RST3"]
pub type RST3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_RR_SPEC, O>;
#[doc = "Field `RST4` reader - RST4"]
pub type RST4_R = crate::BitReader;
#[doc = "Field `RST4` writer - RST4"]
pub type RST4_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_RR_SPEC, O>;
#[doc = "Field `RST5` reader - RST5"]
pub type RST5_R = crate::BitReader;
#[doc = "Field `RST5` writer - RST5"]
pub type RST5_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_RR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    pub fn rst2(&self) -> RST2_R {
        RST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    pub fn rst3(&self) -> RST3_R {
        RST3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    pub fn rst4(&self) -> RST4_R {
        RST4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    pub fn rst5(&self) -> RST5_R {
        RST5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RST0"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<0> {
        RST0_W::new(self)
    }
    #[doc = "Bit 1 - RST1"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<1> {
        RST1_W::new(self)
    }
    #[doc = "Bit 2 - RST2"]
    #[inline(always)]
    #[must_use]
    pub fn rst2(&mut self) -> RST2_W<2> {
        RST2_W::new(self)
    }
    #[doc = "Bit 3 - RST3"]
    #[inline(always)]
    #[must_use]
    pub fn rst3(&mut self) -> RST3_W<3> {
        RST3_W::new(self)
    }
    #[doc = "Bit 4 - RST4"]
    #[inline(always)]
    #[must_use]
    pub fn rst4(&mut self) -> RST4_W<4> {
        RST4_W::new(self)
    }
    #[doc = "Bit 5 - RST5"]
    #[inline(always)]
    #[must_use]
    pub fn rst5(&mut self) -> RST5_W<5> {
        RST5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOD_RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_rr](index.html) module"]
pub struct GPIOD_RR_SPEC;
impl crate::RegisterSpec for GPIOD_RR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiod_rr::R](R) reader structure"]
impl crate::Readable for GPIOD_RR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiod_rr::W](W) writer structure"]
impl crate::Writable for GPIOD_RR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOD_RR to value 0"]
impl crate::Resettable for GPIOD_RR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
