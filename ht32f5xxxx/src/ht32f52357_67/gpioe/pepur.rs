#[doc = "Register `PEPUR` reader"]
pub struct R(crate::R<PEPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEPUR` writer"]
pub struct W(crate::W<PEPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEPUR_SPEC>;
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
impl From<crate::W<PEPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEPU0` reader - PEPU0"]
pub type PEPU0_R = crate::BitReader;
#[doc = "Field `PEPU0` writer - PEPU0"]
pub type PEPU0_W<'a, const O: u8> = crate::BitWriter<'a, PEPUR_SPEC, O>;
#[doc = "Field `PEPU1` reader - PEPU1"]
pub type PEPU1_R = crate::BitReader;
#[doc = "Field `PEPU1` writer - PEPU1"]
pub type PEPU1_W<'a, const O: u8> = crate::BitWriter<'a, PEPUR_SPEC, O>;
#[doc = "Field `PEPU2` reader - PEPU2"]
pub type PEPU2_R = crate::BitReader;
#[doc = "Field `PEPU2` writer - PEPU2"]
pub type PEPU2_W<'a, const O: u8> = crate::BitWriter<'a, PEPUR_SPEC, O>;
#[doc = "Field `PEPU3` reader - PEPU3"]
pub type PEPU3_R = crate::BitReader;
#[doc = "Field `PEPU3` writer - PEPU3"]
pub type PEPU3_W<'a, const O: u8> = crate::BitWriter<'a, PEPUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PEPU0"]
    #[inline(always)]
    pub fn pepu0(&self) -> PEPU0_R {
        PEPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEPU1"]
    #[inline(always)]
    pub fn pepu1(&self) -> PEPU1_R {
        PEPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEPU2"]
    #[inline(always)]
    pub fn pepu2(&self) -> PEPU2_R {
        PEPU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEPU3"]
    #[inline(always)]
    pub fn pepu3(&self) -> PEPU3_R {
        PEPU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEPU0"]
    #[inline(always)]
    #[must_use]
    pub fn pepu0(&mut self) -> PEPU0_W<0> {
        PEPU0_W::new(self)
    }
    #[doc = "Bit 1 - PEPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pepu1(&mut self) -> PEPU1_W<1> {
        PEPU1_W::new(self)
    }
    #[doc = "Bit 2 - PEPU2"]
    #[inline(always)]
    #[must_use]
    pub fn pepu2(&mut self) -> PEPU2_W<2> {
        PEPU2_W::new(self)
    }
    #[doc = "Bit 3 - PEPU3"]
    #[inline(always)]
    #[must_use]
    pub fn pepu3(&mut self) -> PEPU3_W<3> {
        PEPU3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pepur](index.html) module"]
pub struct PEPUR_SPEC;
impl crate::RegisterSpec for PEPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pepur::R](R) reader structure"]
impl crate::Readable for PEPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pepur::W](W) writer structure"]
impl crate::Writable for PEPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEPUR to value 0"]
impl crate::Resettable for PEPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
