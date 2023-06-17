#[doc = "Register `I2C_IER` reader"]
pub struct R(crate::R<I2C_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C_IER` writer"]
pub struct W(crate::W<I2C_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_IER_SPEC>;
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
impl From<crate::W<I2C_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAIE` reader - STAIE"]
pub type STAIE_R = crate::BitReader;
#[doc = "Field `STAIE` writer - STAIE"]
pub type STAIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `STOIE` reader - STOIE"]
pub type STOIE_R = crate::BitReader;
#[doc = "Field `STOIE` writer - STOIE"]
pub type STOIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `ADRSIE` reader - ADRSIE"]
pub type ADRSIE_R = crate::BitReader;
#[doc = "Field `ADRSIE` writer - ADRSIE"]
pub type ADRSIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `GCSIE` reader - GCSIE"]
pub type GCSIE_R = crate::BitReader;
#[doc = "Field `GCSIE` writer - GCSIE"]
pub type GCSIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `ARBLOSIE` reader - ARBLOSIE"]
pub type ARBLOSIE_R = crate::BitReader;
#[doc = "Field `ARBLOSIE` writer - ARBLOSIE"]
pub type ARBLOSIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `RXNACKIE` reader - RXNACKIE"]
pub type RXNACKIE_R = crate::BitReader;
#[doc = "Field `RXNACKIE` writer - RXNACKIE"]
pub type RXNACKIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `BUSERRIE` reader - BUSERRIE"]
pub type BUSERRIE_R = crate::BitReader;
#[doc = "Field `BUSERRIE` writer - BUSERRIE"]
pub type BUSERRIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `TOUTIE` reader - TOUTIE"]
pub type TOUTIE_R = crate::BitReader;
#[doc = "Field `TOUTIE` writer - TOUTIE"]
pub type TOUTIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `RXDNEIE` reader - RXDNEIE"]
pub type RXDNEIE_R = crate::BitReader;
#[doc = "Field `RXDNEIE` writer - RXDNEIE"]
pub type RXDNEIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `TXDEIE` reader - TXDEIE"]
pub type TXDEIE_R = crate::BitReader;
#[doc = "Field `TXDEIE` writer - TXDEIE"]
pub type TXDEIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
#[doc = "Field `RXBFIE` reader - RXBFIE"]
pub type RXBFIE_R = crate::BitReader;
#[doc = "Field `RXBFIE` writer - RXBFIE"]
pub type RXBFIE_W<'a, const O: u8> = crate::BitWriter<'a, I2C_IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - STAIE"]
    #[inline(always)]
    pub fn staie(&self) -> STAIE_R {
        STAIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - STOIE"]
    #[inline(always)]
    pub fn stoie(&self) -> STOIE_R {
        STOIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADRSIE"]
    #[inline(always)]
    pub fn adrsie(&self) -> ADRSIE_R {
        ADRSIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GCSIE"]
    #[inline(always)]
    pub fn gcsie(&self) -> GCSIE_R {
        GCSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - ARBLOSIE"]
    #[inline(always)]
    pub fn arblosie(&self) -> ARBLOSIE_R {
        ARBLOSIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXNACKIE"]
    #[inline(always)]
    pub fn rxnackie(&self) -> RXNACKIE_R {
        RXNACKIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BUSERRIE"]
    #[inline(always)]
    pub fn buserrie(&self) -> BUSERRIE_R {
        BUSERRIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TOUTIE"]
    #[inline(always)]
    pub fn toutie(&self) -> TOUTIE_R {
        TOUTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - RXDNEIE"]
    #[inline(always)]
    pub fn rxdneie(&self) -> RXDNEIE_R {
        RXDNEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&self) -> TXDEIE_R {
        TXDEIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - RXBFIE"]
    #[inline(always)]
    pub fn rxbfie(&self) -> RXBFIE_R {
        RXBFIE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STAIE"]
    #[inline(always)]
    #[must_use]
    pub fn staie(&mut self) -> STAIE_W<0> {
        STAIE_W::new(self)
    }
    #[doc = "Bit 1 - STOIE"]
    #[inline(always)]
    #[must_use]
    pub fn stoie(&mut self) -> STOIE_W<1> {
        STOIE_W::new(self)
    }
    #[doc = "Bit 2 - ADRSIE"]
    #[inline(always)]
    #[must_use]
    pub fn adrsie(&mut self) -> ADRSIE_W<2> {
        ADRSIE_W::new(self)
    }
    #[doc = "Bit 3 - GCSIE"]
    #[inline(always)]
    #[must_use]
    pub fn gcsie(&mut self) -> GCSIE_W<3> {
        GCSIE_W::new(self)
    }
    #[doc = "Bit 8 - ARBLOSIE"]
    #[inline(always)]
    #[must_use]
    pub fn arblosie(&mut self) -> ARBLOSIE_W<8> {
        ARBLOSIE_W::new(self)
    }
    #[doc = "Bit 9 - RXNACKIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxnackie(&mut self) -> RXNACKIE_W<9> {
        RXNACKIE_W::new(self)
    }
    #[doc = "Bit 10 - BUSERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn buserrie(&mut self) -> BUSERRIE_W<10> {
        BUSERRIE_W::new(self)
    }
    #[doc = "Bit 11 - TOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn toutie(&mut self) -> TOUTIE_W<11> {
        TOUTIE_W::new(self)
    }
    #[doc = "Bit 16 - RXDNEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdneie(&mut self) -> RXDNEIE_W<16> {
        RXDNEIE_W::new(self)
    }
    #[doc = "Bit 17 - TXDEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txdeie(&mut self) -> TXDEIE_W<17> {
        TXDEIE_W::new(self)
    }
    #[doc = "Bit 18 - RXBFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbfie(&mut self) -> RXBFIE_W<18> {
        RXBFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ier](index.html) module"]
pub struct I2C_IER_SPEC;
impl crate::RegisterSpec for I2C_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ier::R](R) reader structure"]
impl crate::Readable for I2C_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_ier::W](W) writer structure"]
impl crate::Writable for I2C_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C_IER to value 0"]
impl crate::Resettable for I2C_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
