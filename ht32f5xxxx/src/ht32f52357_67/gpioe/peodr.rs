#[doc = "Register `PEODR` reader"]
pub struct R(crate::R<PEODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEODR` writer"]
pub struct W(crate::W<PEODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEODR_SPEC>;
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
impl From<crate::W<PEODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEOD0` reader - PEOD0"]
pub type PEOD0_R = crate::BitReader;
#[doc = "Field `PEOD0` writer - PEOD0"]
pub type PEOD0_W<'a, const O: u8> = crate::BitWriter<'a, PEODR_SPEC, O>;
#[doc = "Field `PEOD1` reader - PEOD1"]
pub type PEOD1_R = crate::BitReader;
#[doc = "Field `PEOD1` writer - PEOD1"]
pub type PEOD1_W<'a, const O: u8> = crate::BitWriter<'a, PEODR_SPEC, O>;
#[doc = "Field `PEOD2` reader - PEOD2"]
pub type PEOD2_R = crate::BitReader;
#[doc = "Field `PEOD2` writer - PEOD2"]
pub type PEOD2_W<'a, const O: u8> = crate::BitWriter<'a, PEODR_SPEC, O>;
#[doc = "Field `PEOD3` reader - PEOD3"]
pub type PEOD3_R = crate::BitReader;
#[doc = "Field `PEOD3` writer - PEOD3"]
pub type PEOD3_W<'a, const O: u8> = crate::BitWriter<'a, PEODR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PEOD0"]
    #[inline(always)]
    pub fn peod0(&self) -> PEOD0_R {
        PEOD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEOD1"]
    #[inline(always)]
    pub fn peod1(&self) -> PEOD1_R {
        PEOD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEOD2"]
    #[inline(always)]
    pub fn peod2(&self) -> PEOD2_R {
        PEOD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEOD3"]
    #[inline(always)]
    pub fn peod3(&self) -> PEOD3_R {
        PEOD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEOD0"]
    #[inline(always)]
    #[must_use]
    pub fn peod0(&mut self) -> PEOD0_W<0> {
        PEOD0_W::new(self)
    }
    #[doc = "Bit 1 - PEOD1"]
    #[inline(always)]
    #[must_use]
    pub fn peod1(&mut self) -> PEOD1_W<1> {
        PEOD1_W::new(self)
    }
    #[doc = "Bit 2 - PEOD2"]
    #[inline(always)]
    #[must_use]
    pub fn peod2(&mut self) -> PEOD2_W<2> {
        PEOD2_W::new(self)
    }
    #[doc = "Bit 3 - PEOD3"]
    #[inline(always)]
    #[must_use]
    pub fn peod3(&mut self) -> PEOD3_W<3> {
        PEOD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peodr](index.html) module"]
pub struct PEODR_SPEC;
impl crate::RegisterSpec for PEODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peodr::R](R) reader structure"]
impl crate::Readable for PEODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peodr::W](W) writer structure"]
impl crate::Writable for PEODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEODR to value 0"]
impl crate::Resettable for PEODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
