#[doc = "Register `PDODR` reader"]
pub struct R(crate::R<PDODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDODR` writer"]
pub struct W(crate::W<PDODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDODR_SPEC>;
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
impl From<crate::W<PDODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDOD0` reader - PDOD0"]
pub type PDOD0_R = crate::BitReader;
#[doc = "Field `PDOD0` writer - PDOD0"]
pub type PDOD0_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD1` reader - PDOD1"]
pub type PDOD1_R = crate::BitReader;
#[doc = "Field `PDOD1` writer - PDOD1"]
pub type PDOD1_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD2` reader - PDOD2"]
pub type PDOD2_R = crate::BitReader;
#[doc = "Field `PDOD2` writer - PDOD2"]
pub type PDOD2_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
#[doc = "Field `PDOD3` reader - PDOD3"]
pub type PDOD3_R = crate::BitReader;
#[doc = "Field `PDOD3` writer - PDOD3"]
pub type PDOD3_W<'a, const O: u8> = crate::BitWriter<'a, PDODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDOD0"]
    #[inline(always)]
    pub fn pdod0(&self) -> PDOD0_R {
        PDOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDOD1"]
    #[inline(always)]
    pub fn pdod1(&self) -> PDOD1_R {
        PDOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDOD2"]
    #[inline(always)]
    pub fn pdod2(&self) -> PDOD2_R {
        PDOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDOD3"]
    #[inline(always)]
    pub fn pdod3(&self) -> PDOD3_R {
        PDOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDOD0"]
    #[inline(always)]
    #[must_use]
    pub fn pdod0(&mut self) -> PDOD0_W<0> {
        PDOD0_W::new(self)
    }
    #[doc = "Bit 1 - PDOD1"]
    #[inline(always)]
    #[must_use]
    pub fn pdod1(&mut self) -> PDOD1_W<1> {
        PDOD1_W::new(self)
    }
    #[doc = "Bit 2 - PDOD2"]
    #[inline(always)]
    #[must_use]
    pub fn pdod2(&mut self) -> PDOD2_W<2> {
        PDOD2_W::new(self)
    }
    #[doc = "Bit 3 - PDOD3"]
    #[inline(always)]
    #[must_use]
    pub fn pdod3(&mut self) -> PDOD3_W<3> {
        PDOD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdodr](index.html) module"]
pub struct PDODR_SPEC;
impl crate::RegisterSpec for PDODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdodr::R](R) reader structure"]
impl crate::Readable for PDODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdodr::W](W) writer structure"]
impl crate::Writable for PDODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDODR to value 0"]
impl crate::Resettable for PDODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
