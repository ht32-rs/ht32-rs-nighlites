#[doc = "Register `LCD_LCDFCR` reader"]
pub struct R(crate::R<LCD_LCDFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCD_LCDFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCD_LCDFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCD_LCDFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCD_LCDFCR` writer"]
pub struct W(crate::W<LCD_LCDFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCD_LCDFCR_SPEC>;
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
impl From<crate::W<LCD_LCDFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCD_LCDFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDEN` reader - HDEN"]
pub type HDEN_R = crate::BitReader;
#[doc = "Field `HDEN` writer - HDEN"]
pub type HDEN_W<'a, const O: u8> = crate::BitWriter<'a, LCD_LCDFCR_SPEC, O>;
#[doc = "Field `HDD` reader - HDD"]
pub type HDD_R = crate::FieldReader;
#[doc = "Field `HDD` writer - HDD"]
pub type HDD_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDFCR_SPEC, 3, O>;
#[doc = "Field `DEAD` reader - DEAD"]
pub type DEAD_R = crate::FieldReader;
#[doc = "Field `DEAD` writer - DEAD"]
pub type DEAD_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDFCR_SPEC, 3, O>;
#[doc = "Field `CPVS` reader - CPVS"]
pub type CPVS_R = crate::FieldReader;
#[doc = "Field `CPVS` writer - CPVS"]
pub type CPVS_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDFCR_SPEC, 3, O>;
#[doc = "Field `BLINKF` reader - BLINKF"]
pub type BLINKF_R = crate::FieldReader;
#[doc = "Field `BLINKF` writer - BLINKF"]
pub type BLINKF_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDFCR_SPEC, 3, O>;
#[doc = "Field `BLINK` reader - BLINK"]
pub type BLINK_R = crate::FieldReader;
#[doc = "Field `BLINK` writer - BLINK"]
pub type BLINK_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDFCR_SPEC, 2, O>;
#[doc = "Field `LCDDIV` reader - LCDDIV"]
pub type LCDDIV_R = crate::FieldReader;
#[doc = "Field `LCDDIV` writer - LCDDIV"]
pub type LCDDIV_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDFCR_SPEC, 4, O>;
#[doc = "Field `LCDPS` reader - LCDPS"]
pub type LCDPS_R = crate::FieldReader;
#[doc = "Field `LCDPS` writer - LCDPS"]
pub type LCDPS_W<'a, const O: u8> = crate::FieldWriter<'a, LCD_LCDFCR_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - HDEN"]
    #[inline(always)]
    pub fn hden(&self) -> HDEN_R {
        HDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - HDD"]
    #[inline(always)]
    pub fn hdd(&self) -> HDD_R {
        HDD_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - DEAD"]
    #[inline(always)]
    pub fn dead(&self) -> DEAD_R {
        DEAD_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - CPVS"]
    #[inline(always)]
    pub fn cpvs(&self) -> CPVS_R {
        CPVS_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - BLINKF"]
    #[inline(always)]
    pub fn blinkf(&self) -> BLINKF_R {
        BLINKF_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - BLINK"]
    #[inline(always)]
    pub fn blink(&self) -> BLINK_R {
        BLINK_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - LCDDIV"]
    #[inline(always)]
    pub fn lcddiv(&self) -> LCDDIV_R {
        LCDDIV_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:25 - LCDPS"]
    #[inline(always)]
    pub fn lcdps(&self) -> LCDPS_R {
        LCDPS_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HDEN"]
    #[inline(always)]
    #[must_use]
    pub fn hden(&mut self) -> HDEN_W<0> {
        HDEN_W::new(self)
    }
    #[doc = "Bits 4:6 - HDD"]
    #[inline(always)]
    #[must_use]
    pub fn hdd(&mut self) -> HDD_W<4> {
        HDD_W::new(self)
    }
    #[doc = "Bits 7:9 - DEAD"]
    #[inline(always)]
    #[must_use]
    pub fn dead(&mut self) -> DEAD_W<7> {
        DEAD_W::new(self)
    }
    #[doc = "Bits 10:12 - CPVS"]
    #[inline(always)]
    #[must_use]
    pub fn cpvs(&mut self) -> CPVS_W<10> {
        CPVS_W::new(self)
    }
    #[doc = "Bits 13:15 - BLINKF"]
    #[inline(always)]
    #[must_use]
    pub fn blinkf(&mut self) -> BLINKF_W<13> {
        BLINKF_W::new(self)
    }
    #[doc = "Bits 16:17 - BLINK"]
    #[inline(always)]
    #[must_use]
    pub fn blink(&mut self) -> BLINK_W<16> {
        BLINK_W::new(self)
    }
    #[doc = "Bits 18:21 - LCDDIV"]
    #[inline(always)]
    #[must_use]
    pub fn lcddiv(&mut self) -> LCDDIV_W<18> {
        LCDDIV_W::new(self)
    }
    #[doc = "Bits 22:25 - LCDPS"]
    #[inline(always)]
    #[must_use]
    pub fn lcdps(&mut self) -> LCDPS_W<22> {
        LCDPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD_LCDFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcd_lcdfcr](index.html) module"]
pub struct LCD_LCDFCR_SPEC;
impl crate::RegisterSpec for LCD_LCDFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcd_lcdfcr::R](R) reader structure"]
impl crate::Readable for LCD_LCDFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcd_lcdfcr::W](W) writer structure"]
impl crate::Writable for LCD_LCDFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCD_LCDFCR to value 0"]
impl crate::Resettable for LCD_LCDFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
