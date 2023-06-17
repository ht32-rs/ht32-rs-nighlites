#[doc = "Register `I2C_TOUT` reader"]
pub struct R(crate::R<I2C_TOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_TOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_TOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_TOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_TOUT` writer"]
pub struct W(crate::W<I2C_TOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_TOUT_SPEC>;
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
impl From<crate::W<I2C_TOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_TOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUT` reader - TOUT"]
pub type TOUT_R = crate::FieldReader<u16>;
#[doc = "Field `TOUT` writer - TOUT"]
pub type TOUT_W<'a, const O: u8> = crate::FieldWriter<'a, I2C_TOUT_SPEC, 16, O, u16>;
#[doc = "Field `PSC` reader - PSC"]
pub type PSC_R = crate::FieldReader;
#[doc = "Field `PSC` writer - PSC"]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, I2C_TOUT_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - TOUT"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> TOUT_W<0> {
        TOUT_W::new(self)
    }
    #[doc = "Bits 16:18 - PSC"]
    #[inline(always)]
    #[must_use]
    pub fn psc(&mut self) -> PSC_W<16> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_TOUT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_tout](index.html) module"]
pub struct I2C_TOUT_SPEC;
impl crate::RegisterSpec for I2C_TOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_tout::R](R) reader structure"]
impl crate::Readable for I2C_TOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_tout::W](W) writer structure"]
impl crate::Writable for I2C_TOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_TOUT to value 0"]
impl crate::Resettable for I2C_TOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
