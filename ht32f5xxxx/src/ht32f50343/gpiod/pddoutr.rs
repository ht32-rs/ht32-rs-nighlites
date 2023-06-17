#[doc = "Register `PDDOUTR` reader"]
pub struct R(crate::R<PDDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDOUTR` writer"]
pub struct W(crate::W<PDDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDOUTR_SPEC>;
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
impl From<crate::W<PDDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDOUT0` reader - PDDOUT0"]
pub type PDDOUT0_R = crate::BitReader;
#[doc = "Field `PDDOUT0` writer - PDDOUT0"]
pub type PDDOUT0_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT1` reader - PDDOUT1"]
pub type PDDOUT1_R = crate::BitReader;
#[doc = "Field `PDDOUT1` writer - PDDOUT1"]
pub type PDDOUT1_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT2` reader - PDDOUT2"]
pub type PDDOUT2_R = crate::BitReader;
#[doc = "Field `PDDOUT2` writer - PDDOUT2"]
pub type PDDOUT2_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT3` reader - PDDOUT3"]
pub type PDDOUT3_R = crate::BitReader;
#[doc = "Field `PDDOUT3` writer - PDDOUT3"]
pub type PDDOUT3_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT4` reader - PDDOUT4"]
pub type PDDOUT4_R = crate::BitReader;
#[doc = "Field `PDDOUT4` writer - PDDOUT4"]
pub type PDDOUT4_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
#[doc = "Field `PDDOUT5` reader - PDDOUT5"]
pub type PDDOUT5_R = crate::BitReader;
#[doc = "Field `PDDOUT5` writer - PDDOUT5"]
pub type PDDOUT5_W<'a, const O: u8> = crate::BitWriter<'a, PDDOUTR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDDOUT0"]
    #[inline(always)]
    pub fn pddout0(&self) -> PDDOUT0_R {
        PDDOUT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDDOUT1"]
    #[inline(always)]
    pub fn pddout1(&self) -> PDDOUT1_R {
        PDDOUT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDDOUT2"]
    #[inline(always)]
    pub fn pddout2(&self) -> PDDOUT2_R {
        PDDOUT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDDOUT3"]
    #[inline(always)]
    pub fn pddout3(&self) -> PDDOUT3_R {
        PDDOUT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDDOUT4"]
    #[inline(always)]
    pub fn pddout4(&self) -> PDDOUT4_R {
        PDDOUT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDDOUT5"]
    #[inline(always)]
    pub fn pddout5(&self) -> PDDOUT5_R {
        PDDOUT5_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDDOUT0"]
    #[inline(always)]
    #[must_use]
    pub fn pddout0(&mut self) -> PDDOUT0_W<0> {
        PDDOUT0_W::new(self)
    }
    #[doc = "Bit 1 - PDDOUT1"]
    #[inline(always)]
    #[must_use]
    pub fn pddout1(&mut self) -> PDDOUT1_W<1> {
        PDDOUT1_W::new(self)
    }
    #[doc = "Bit 2 - PDDOUT2"]
    #[inline(always)]
    #[must_use]
    pub fn pddout2(&mut self) -> PDDOUT2_W<2> {
        PDDOUT2_W::new(self)
    }
    #[doc = "Bit 3 - PDDOUT3"]
    #[inline(always)]
    #[must_use]
    pub fn pddout3(&mut self) -> PDDOUT3_W<3> {
        PDDOUT3_W::new(self)
    }
    #[doc = "Bit 4 - PDDOUT4"]
    #[inline(always)]
    #[must_use]
    pub fn pddout4(&mut self) -> PDDOUT4_W<4> {
        PDDOUT4_W::new(self)
    }
    #[doc = "Bit 5 - PDDOUT5"]
    #[inline(always)]
    #[must_use]
    pub fn pddout5(&mut self) -> PDDOUT5_W<5> {
        PDDOUT5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddoutr](index.html) module"]
pub struct PDDOUTR_SPEC;
impl crate::RegisterSpec for PDDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddoutr::R](R) reader structure"]
impl crate::Readable for PDDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddoutr::W](W) writer structure"]
impl crate::Writable for PDDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDOUTR to value 0"]
impl crate::Resettable for PDDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
