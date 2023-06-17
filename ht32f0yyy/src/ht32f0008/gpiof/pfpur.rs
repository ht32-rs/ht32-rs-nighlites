#[doc = "Register `PFPUR` reader"]
pub struct R(crate::R<PFPUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFPUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFPUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFPUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFPUR` writer"]
pub struct W(crate::W<PFPUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFPUR_SPEC>;
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
impl From<crate::W<PFPUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFPUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFPU0` reader - PFPU0"]
pub type PFPU0_R = crate::BitReader;
#[doc = "Field `PFPU0` writer - PFPU0"]
pub type PFPU0_W<'a, const O: u8> = crate::BitWriter<'a, PFPUR_SPEC, O>;
#[doc = "Field `PFPU1` reader - PFPU1"]
pub type PFPU1_R = crate::BitReader;
#[doc = "Field `PFPU1` writer - PFPU1"]
pub type PFPU1_W<'a, const O: u8> = crate::BitWriter<'a, PFPUR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFPU0"]
    #[inline(always)]
    pub fn pfpu0(&self) -> PFPU0_R {
        PFPU0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFPU1"]
    #[inline(always)]
    pub fn pfpu1(&self) -> PFPU1_R {
        PFPU1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFPU0"]
    #[inline(always)]
    #[must_use]
    pub fn pfpu0(&mut self) -> PFPU0_W<0> {
        PFPU0_W::new(self)
    }
    #[doc = "Bit 1 - PFPU1"]
    #[inline(always)]
    #[must_use]
    pub fn pfpu1(&mut self) -> PFPU1_W<1> {
        PFPU1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFPUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfpur](index.html) module"]
pub struct PFPUR_SPEC;
impl crate::RegisterSpec for PFPUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfpur::R](R) reader structure"]
impl crate::Readable for PFPUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfpur::W](W) writer structure"]
impl crate::Writable for PFPUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFPUR to value 0"]
impl crate::Resettable for PFPUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
