#[doc = "Register `LCD_LCDRAM1` reader"]
pub struct R(crate::R<LCD_LCDRAM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LCDRAM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LCDRAM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LCDRAM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_LCDRAM1` writer"]
pub struct W(crate::W<LCD_LCDRAM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LCDRAM1_SPEC>;
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
impl From<crate::W<LCD_LCDRAM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LCDRAM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEG_DATA` reader - SEG_DATA"]
pub type SEG_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `SEG_DATA` writer - SEG_DATA"]
pub type SEG_DATA_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDRAM1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SEG_DATA"]
    #[inline(always)]
    pub fn seg_data(&self) -> SEG_DATA_R {
        SEG_DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SEG_DATA"]
    #[inline(always)]
    #[must_use]
    pub fn seg_data(&mut self) -> SEG_DATA_W<0> {
        SEG_DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_LCDRAM1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lcdram1](index.html) module"]
pub struct LCD_LCDRAM1_SPEC;
impl crate::RegisterSpec for LCD_LCDRAM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lcdram1::R](R) reader structure"]
impl crate::Readable for LCD_LCDRAM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lcdram1::W](W) writer structure"]
impl crate::Writable for LCD_LCDRAM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_LCDRAM1 to value 0"]
impl crate::Resettable for LCD_LCDRAM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
