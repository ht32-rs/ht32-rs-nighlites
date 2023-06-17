#[doc = "Register `GPIOE_INER` reader"]
pub struct R(crate::R<GPIOE_INER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOE_INER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOE_INER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOE_INER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOE_INER` writer"]
pub struct W(crate::W<GPIOE_INER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOE_INER_SPEC>;
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
impl From<crate::W<GPIOE_INER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOE_INER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEN0` reader - INEN0"]
pub type INEN0_R = crate::BitReader;
#[doc = "Field `INEN0` writer - INEN0"]
pub type INEN0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_INER_SPEC, O>;
#[doc = "Field `INEN1` reader - INEN1"]
pub type INEN1_R = crate::BitReader;
#[doc = "Field `INEN1` writer - INEN1"]
pub type INEN1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_INER_SPEC, O>;
#[doc = "Field `INEN2` reader - INEN2"]
pub type INEN2_R = crate::BitReader;
#[doc = "Field `INEN2` writer - INEN2"]
pub type INEN2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_INER_SPEC, O>;
#[doc = "Field `INEN3` reader - INEN3"]
pub type INEN3_R = crate::BitReader;
#[doc = "Field `INEN3` writer - INEN3"]
pub type INEN3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOE_INER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    pub fn inen0(&self) -> INEN0_R {
        INEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    pub fn inen1(&self) -> INEN1_R {
        INEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    pub fn inen2(&self) -> INEN2_R {
        INEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    pub fn inen3(&self) -> INEN3_R {
        INEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INEN0"]
    #[inline(always)]
    #[must_use]
    pub fn inen0(&mut self) -> INEN0_W<0> {
        INEN0_W::new(self)
    }
    #[doc = "Bit 1 - INEN1"]
    #[inline(always)]
    #[must_use]
    pub fn inen1(&mut self) -> INEN1_W<1> {
        INEN1_W::new(self)
    }
    #[doc = "Bit 2 - INEN2"]
    #[inline(always)]
    #[must_use]
    pub fn inen2(&mut self) -> INEN2_W<2> {
        INEN2_W::new(self)
    }
    #[doc = "Bit 3 - INEN3"]
    #[inline(always)]
    #[must_use]
    pub fn inen3(&mut self) -> INEN3_W<3> {
        INEN3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOE_INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_iner](index.html) module"]
pub struct GPIOE_INER_SPEC;
impl crate::RegisterSpec for GPIOE_INER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioe_iner::R](R) reader structure"]
impl crate::Readable for GPIOE_INER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpioe_iner::W](W) writer structure"]
impl crate::Writable for GPIOE_INER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOE_INER to value 0"]
impl crate::Resettable for GPIOE_INER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
