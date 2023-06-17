#[doc = "Register `LCD_LCDIER` reader"]
pub struct R(crate::R<LCD_LCDIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LCDIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LCDIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LCDIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_LCDIER` writer"]
pub struct W(crate::W<LCD_LCDIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LCDIER_SPEC>;
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
impl From<crate::W<LCD_LCDIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LCDIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFIE` reader - SOFIE"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - SOFIE"]
pub type SOFIE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDIER_SPEC, O>;
#[doc = "Field `UDDIE` reader - UDDIE"]
pub type UDDIE_R = crate::BitReader;
#[doc = "Field `UDDIE` writer - UDDIE"]
pub type UDDIE_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDIER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SOFIE"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UDDIE"]
    #[inline(always)]
    pub fn uddie(&self) -> UDDIE_R {
        UDDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<0> {
        SOFIE_W::new(self)
    }
    #[doc = "Bit 1 - UDDIE"]
    #[inline(always)]
    #[must_use]
    pub fn uddie(&mut self) -> UDDIE_W<1> {
        UDDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_LCDIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lcdier](index.html) module"]
pub struct LCD_LCDIER_SPEC;
impl crate::RegisterSpec for LCD_LCDIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lcdier::R](R) reader structure"]
impl crate::Readable for LCD_LCDIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lcdier::W](W) writer structure"]
impl crate::Writable for LCD_LCDIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_LCDIER to value 0"]
impl crate::Resettable for LCD_LCDIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
