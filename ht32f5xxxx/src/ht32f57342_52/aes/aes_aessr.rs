#[doc = "Register `AES_AESSR` reader"]
pub struct R(crate::R<AES_AESSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_AESSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_AESSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_AESSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_AESSR` writer"]
pub struct W(crate::W<AES_AESSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_AESSR_SPEC>;
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
impl From<crate::W<AES_AESSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_AESSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFEMPTY` reader - IFEMPTY"]
pub type IFEMPTY_R = crate::BitReader;
#[doc = "Field `IFEMPTY` writer - IFEMPTY"]
pub type IFEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESSR_SPEC, O>;
#[doc = "Field `IFNFULL` reader - IFNFULL"]
pub type IFNFULL_R = crate::BitReader;
#[doc = "Field `IFNFULL` writer - IFNFULL"]
pub type IFNFULL_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESSR_SPEC, O>;
#[doc = "Field `OFNEMPTY` reader - OFNEMPTY"]
pub type OFNEMPTY_R = crate::BitReader;
#[doc = "Field `OFNEMPTY` writer - OFNEMPTY"]
pub type OFNEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESSR_SPEC, O>;
#[doc = "Field `OFFULL` reader - OFFULL"]
pub type OFFULL_R = crate::BitReader;
#[doc = "Field `OFFULL` writer - OFFULL"]
pub type OFFULL_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESSR_SPEC, O>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - IFEMPTY"]
    #[inline(always)]
    pub fn ifempty(&self) -> IFEMPTY_R {
        IFEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IFNFULL"]
    #[inline(always)]
    pub fn ifnfull(&self) -> IFNFULL_R {
        IFNFULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OFNEMPTY"]
    #[inline(always)]
    pub fn ofnempty(&self) -> OFNEMPTY_R {
        OFNEMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OFFULL"]
    #[inline(always)]
    pub fn offull(&self) -> OFFULL_R {
        OFFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IFEMPTY"]
    #[inline(always)]
    #[must_use]
    pub fn ifempty(&mut self) -> IFEMPTY_W<0> {
        IFEMPTY_W::new(self)
    }
    #[doc = "Bit 1 - IFNFULL"]
    #[inline(always)]
    #[must_use]
    pub fn ifnfull(&mut self) -> IFNFULL_W<1> {
        IFNFULL_W::new(self)
    }
    #[doc = "Bit 2 - OFNEMPTY"]
    #[inline(always)]
    #[must_use]
    pub fn ofnempty(&mut self) -> OFNEMPTY_W<2> {
        OFNEMPTY_W::new(self)
    }
    #[doc = "Bit 3 - OFFULL"]
    #[inline(always)]
    #[must_use]
    pub fn offull(&mut self) -> OFFULL_W<3> {
        OFFULL_W::new(self)
    }
    #[doc = "Bit 4 - BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<4> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_AESSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_aessr](index.html) module"]
pub struct AES_AESSR_SPEC;
impl crate::RegisterSpec for AES_AESSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_aessr::R](R) reader structure"]
impl crate::Readable for AES_AESSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_aessr::W](W) writer structure"]
impl crate::Writable for AES_AESSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AESSR to value 0"]
impl crate::Resettable for AES_AESSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
