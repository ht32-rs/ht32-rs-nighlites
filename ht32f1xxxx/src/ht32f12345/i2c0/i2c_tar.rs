#[doc = "Register `I2C_TAR` reader"]
pub struct R(crate::R<I2C_TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TAR` writer"]
pub struct W(crate::W<I2C_TAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TAR_SPEC>;
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
impl From<crate::W<I2C_TAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAR` reader - TAR"]
pub type TAR_R = crate::FieldReader<u16>;
#[doc = "Field `TAR` writer - TAR"]
pub type TAR_W<'a, const O: u8> = crate::FieldWriter<'a, I2C_TAR_SPEC, 10, O, u16>;
#[doc = "Field `RWD` reader - RWD"]
pub type RWD_R = crate::BitReader;
#[doc = "Field `RWD` writer - RWD"]
pub type RWD_W<'a, const O: u8> = crate::BitWriter<'a, I2C_TAR_SPEC, O>;
impl R {
    #[doc = "Bits 0:9 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - RWD"]
    #[inline(always)]
    pub fn rwd(&self) -> RWD_R {
        RWD_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - TAR"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<0> {
        TAR_W::new(self)
    }
    #[doc = "Bit 10 - RWD"]
    #[inline(always)]
    #[must_use]
    pub fn rwd(&mut self) -> RWD_W<10> {
        RWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_TAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_tar](index.html) module"]
pub struct I2C_TAR_SPEC;
impl crate::RegisterSpec for I2C_TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_tar::R](R) reader structure"]
impl crate::Readable for I2C_TAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_tar::W](W) writer structure"]
impl crate::Writable for I2C_TAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_TAR to value 0"]
impl crate::Resettable for I2C_TAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
