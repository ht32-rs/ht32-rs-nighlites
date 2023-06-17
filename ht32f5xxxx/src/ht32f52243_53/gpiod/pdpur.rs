#[doc = "Register `PDPUR` reader"]
pub struct R(crate::R<PDPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDPUR` writer"]
pub struct W(crate::W<PDPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDPUR_SPEC>;
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
impl From<crate::W<PDPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDPU0` reader - PDPU0"]
pub type PDPU0_R = crate::BitReader;
#[doc = "Field `PDPU0` writer - PDPU0"]
pub type PDPU0_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU1` reader - PDPU1"]
pub type PDPU1_R = crate::BitReader;
#[doc = "Field `PDPU1` writer - PDPU1"]
pub type PDPU1_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU2` reader - PDPU2"]
pub type PDPU2_R = crate::BitReader;
#[doc = "Field `PDPU2` writer - PDPU2"]
pub type PDPU2_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
#[doc = "Field `PDPU3` reader - PDPU3"]
pub type PDPU3_R = crate::BitReader;
#[doc = "Field `PDPU3` writer - PDPU3"]
pub type PDPU3_W<'a, const O: u8> = crate::BitWriter<'a, PDPUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDPU0"]
    #[inline(always)]
    pub fn pdpu0(&self) -> PDPU0_R {
        PDPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDPU1"]
    #[inline(always)]
    pub fn pdpu1(&self) -> PDPU1_R {
        PDPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDPU2"]
    #[inline(always)]
    pub fn pdpu2(&self) -> PDPU2_R {
        PDPU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDPU3"]
    #[inline(always)]
    pub fn pdpu3(&self) -> PDPU3_R {
        PDPU3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDPU0"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu0(&mut self) -> PDPU0_W<0> {
        PDPU0_W::new(self)
    }
    #[doc = "Bit 1 - PDPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu1(&mut self) -> PDPU1_W<1> {
        PDPU1_W::new(self)
    }
    #[doc = "Bit 2 - PDPU2"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu2(&mut self) -> PDPU2_W<2> {
        PDPU2_W::new(self)
    }
    #[doc = "Bit 3 - PDPU3"]
    #[inline(always)]
    #[must_use]
    pub fn pdpu3(&mut self) -> PDPU3_W<3> {
        PDPU3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdpur](index.html) module"]
pub struct PDPUR_SPEC;
impl crate::RegisterSpec for PDPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdpur::R](R) reader structure"]
impl crate::Readable for PDPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdpur::W](W) writer structure"]
impl crate::Writable for PDPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDPUR to value 0"]
impl crate::Resettable for PDPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
