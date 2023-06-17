#[doc = "Register `GPIOE_SRR` reader"]
pub struct R(crate::R<GPIOE_SRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_SRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_SRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_SRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_SRR` writer"]
pub struct W(crate::W<GPIOE_SRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_SRR_SPEC>;
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
impl From<crate::W<GPIOE_SRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_SRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET0` reader - SET0"]
pub type SET0_R = crate::BitReader;
#[doc = "Field `SET0` writer - SET0"]
pub type SET0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
#[doc = "Field `SET1` reader - SET1"]
pub type SET1_R = crate::BitReader;
#[doc = "Field `SET1` writer - SET1"]
pub type SET1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
#[doc = "Field `SET2` reader - SET2"]
pub type SET2_R = crate::BitReader;
#[doc = "Field `SET2` writer - SET2"]
pub type SET2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
#[doc = "Field `SET3` reader - SET3"]
pub type SET3_R = crate::BitReader;
#[doc = "Field `SET3` writer - SET3"]
pub type SET3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
#[doc = "Field `RST0` reader - RST0"]
pub type RST0_R = crate::BitReader;
#[doc = "Field `RST0` writer - RST0"]
pub type RST0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
#[doc = "Field `RST1` reader - RST1"]
pub type RST1_R = crate::BitReader;
#[doc = "Field `RST1` writer - RST1"]
pub type RST1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
#[doc = "Field `RST2` reader - RST2"]
pub type RST2_R = crate::BitReader;
#[doc = "Field `RST2` writer - RST2"]
pub type RST2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
#[doc = "Field `RST3` reader - RST3"]
pub type RST3_R = crate::BitReader;
#[doc = "Field `RST3` writer - RST3"]
pub type RST3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_SRR_SPEC, O>;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_srr](index.html) module"]
pub struct GPIOE_SRR_SPEC;
impl crate::RegisterSpec for GPIOE_SRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_srr::R](R) reader structure"]
impl crate::Readable for GPIOE_SRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_srr::W](W) writer structure"]
impl crate::Writable for GPIOE_SRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_SRR to value 0"]
impl crate::Resettable for GPIOE_SRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
