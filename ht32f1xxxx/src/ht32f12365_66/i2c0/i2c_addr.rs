#[doc = "Register `I2C_ADDR` reader"]
pub struct R(crate::R<I2C_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_ADDR` writer"]
pub struct W(crate::W<I2C_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_ADDR_SPEC>;
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
impl From<crate::W<I2C_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - ADDR"]
pub type ADDR_R = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - ADDR"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, I2C_ADDR_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - ADDR"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - ADDR"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_addr](index.html) module"]
pub struct I2C_ADDR_SPEC;
impl crate::RegisterSpec for I2C_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_addr::R](R) reader structure"]
impl crate::Readable for I2C_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_addr::W](W) writer structure"]
impl crate::Writable for I2C_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_ADDR to value 0"]
impl crate::Resettable for I2C_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
