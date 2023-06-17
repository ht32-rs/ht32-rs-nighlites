#[doc = "Register `GPIOD_SRR` reader"]
pub struct R(crate::R<GPIOD_SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOD_SRR` writer"]
pub struct W(crate::W<GPIOD_SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOD_SRR_SPEC>;
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
impl From<crate::W<GPIOD_SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOD_SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET0` reader - SET0"]
pub type SET0_R = crate::BitReader;
#[doc = "Field `SET0` writer - SET0"]
pub type SET0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `SET1` reader - SET1"]
pub type SET1_R = crate::BitReader;
#[doc = "Field `SET1` writer - SET1"]
pub type SET1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `SET2` reader - SET2"]
pub type SET2_R = crate::BitReader;
#[doc = "Field `SET2` writer - SET2"]
pub type SET2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `SET3` reader - SET3"]
pub type SET3_R = crate::BitReader;
#[doc = "Field `SET3` writer - SET3"]
pub type SET3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `SET4` reader - SET4"]
pub type SET4_R = crate::BitReader;
#[doc = "Field `SET4` writer - SET4"]
pub type SET4_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `SET5` reader - SET5"]
pub type SET5_R = crate::BitReader;
#[doc = "Field `SET5` writer - SET5"]
pub type SET5_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `RST0` reader - RST0"]
pub type RST0_R = crate::BitReader;
#[doc = "Field `RST0` writer - RST0"]
pub type RST0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `RST1` reader - RST1"]
pub type RST1_R = crate::BitReader;
#[doc = "Field `RST1` writer - RST1"]
pub type RST1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `RST2` reader - RST2"]
pub type RST2_R = crate::BitReader;
#[doc = "Field `RST2` writer - RST2"]
pub type RST2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `RST3` reader - RST3"]
pub type RST3_R = crate::BitReader;
#[doc = "Field `RST3` writer - RST3"]
pub type RST3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `RST4` reader - RST4"]
pub type RST4_R = crate::BitReader;
#[doc = "Field `RST4` writer - RST4"]
pub type RST4_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
#[doc = "Field `RST5` reader - RST5"]
pub type RST5_R = crate::BitReader;
#[doc = "Field `RST5` writer - RST5"]
pub type RST5_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_SRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SET0"]
    #[inline(always)]
    pub fn set0(&self) -> SET0_R {
        SET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SET1"]
    #[inline(always)]
    pub fn set1(&self) -> SET1_R {
        SET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SET2"]
    #[inline(always)]
    pub fn set2(&self) -> SET2_R {
        SET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SET3"]
    #[inline(always)]
    pub fn set3(&self) -> SET3_R {
        SET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SET4"]
    #[inline(always)]
    pub fn set4(&self) -> SET4_R {
        SET4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SET5"]
    #[inline(always)]
    pub fn set5(&self) -> SET5_R {
        SET5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - RST0"]
    #[inline(always)]
    pub fn rst0(&self) -> RST0_R {
        RST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RST1"]
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RST2"]
    #[inline(always)]
    pub fn rst2(&self) -> RST2_R {
        RST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RST3"]
    #[inline(always)]
    pub fn rst3(&self) -> RST3_R {
        RST3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RST4"]
    #[inline(always)]
    pub fn rst4(&self) -> RST4_R {
        RST4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RST5"]
    #[inline(always)]
    pub fn rst5(&self) -> RST5_R {
        RST5_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SET0"]
    #[inline(always)]
    #[must_use]
    pub fn set0(&mut self) -> SET0_W<0> {
        SET0_W::new(self)
    }
    #[doc = "Bit 1 - SET1"]
    #[inline(always)]
    #[must_use]
    pub fn set1(&mut self) -> SET1_W<1> {
        SET1_W::new(self)
    }
    #[doc = "Bit 2 - SET2"]
    #[inline(always)]
    #[must_use]
    pub fn set2(&mut self) -> SET2_W<2> {
        SET2_W::new(self)
    }
    #[doc = "Bit 3 - SET3"]
    #[inline(always)]
    #[must_use]
    pub fn set3(&mut self) -> SET3_W<3> {
        SET3_W::new(self)
    }
    #[doc = "Bit 4 - SET4"]
    #[inline(always)]
    #[must_use]
    pub fn set4(&mut self) -> SET4_W<4> {
        SET4_W::new(self)
    }
    #[doc = "Bit 5 - SET5"]
    #[inline(always)]
    #[must_use]
    pub fn set5(&mut self) -> SET5_W<5> {
        SET5_W::new(self)
    }
    #[doc = "Bit 16 - RST0"]
    #[inline(always)]
    #[must_use]
    pub fn rst0(&mut self) -> RST0_W<16> {
        RST0_W::new(self)
    }
    #[doc = "Bit 17 - RST1"]
    #[inline(always)]
    #[must_use]
    pub fn rst1(&mut self) -> RST1_W<17> {
        RST1_W::new(self)
    }
    #[doc = "Bit 18 - RST2"]
    #[inline(always)]
    #[must_use]
    pub fn rst2(&mut self) -> RST2_W<18> {
        RST2_W::new(self)
    }
    #[doc = "Bit 19 - RST3"]
    #[inline(always)]
    #[must_use]
    pub fn rst3(&mut self) -> RST3_W<19> {
        RST3_W::new(self)
    }
    #[doc = "Bit 20 - RST4"]
    #[inline(always)]
    #[must_use]
    pub fn rst4(&mut self) -> RST4_W<20> {
        RST4_W::new(self)
    }
    #[doc = "Bit 21 - RST5"]
    #[inline(always)]
    #[must_use]
    pub fn rst5(&mut self) -> RST5_W<21> {
        RST5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOD_SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_srr](index.html) module"]
pub struct GPIOD_SRR_SPEC;
impl crate::RegisterSpec for GPIOD_SRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiod_srr::R](R) reader structure"]
impl crate::Readable for GPIOD_SRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiod_srr::W](W) writer structure"]
impl crate::Writable for GPIOD_SRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOD_SRR to value 0"]
impl crate::Resettable for GPIOD_SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
