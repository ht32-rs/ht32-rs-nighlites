#[doc = "Register `GPIOD_LOCKR` reader"]
pub struct R(crate::R<GPIOD_LOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOD_LOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOD_LOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOD_LOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOD_LOCKR` writer"]
pub struct W(crate::W<GPIOD_LOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOD_LOCKR_SPEC>;
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
impl From<crate::W<GPIOD_LOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOD_LOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK0` reader - LOCK0"]
pub type LOCK0_R = crate::BitReader;
#[doc = "Field `LOCK0` writer - LOCK0"]
pub type LOCK0_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_LOCKR_SPEC, O>;
#[doc = "Field `LOCK1` reader - LOCK1"]
pub type LOCK1_R = crate::BitReader;
#[doc = "Field `LOCK1` writer - LOCK1"]
pub type LOCK1_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_LOCKR_SPEC, O>;
#[doc = "Field `LOCK2` reader - LOCK2"]
pub type LOCK2_R = crate::BitReader;
#[doc = "Field `LOCK2` writer - LOCK2"]
pub type LOCK2_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_LOCKR_SPEC, O>;
#[doc = "Field `LOCK3` reader - LOCK3"]
pub type LOCK3_R = crate::BitReader;
#[doc = "Field `LOCK3` writer - LOCK3"]
pub type LOCK3_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_LOCKR_SPEC, O>;
#[doc = "Field `LOCK4` reader - LOCK4"]
pub type LOCK4_R = crate::BitReader;
#[doc = "Field `LOCK4` writer - LOCK4"]
pub type LOCK4_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_LOCKR_SPEC, O>;
#[doc = "Field `LOCK5` reader - LOCK5"]
pub type LOCK5_R = crate::BitReader;
#[doc = "Field `LOCK5` writer - LOCK5"]
pub type LOCK5_W<'a, const O: u8> = crate::BitWriter<'a, GPIOD_LOCKR_SPEC, O>;
#[doc = "Field `LKEY` reader - LKEY"]
pub type LKEY_R = crate::FieldReader<u16>;
#[doc = "Field `LKEY` writer - LKEY"]
pub type LKEY_W<'a, const O: u8> = crate::FieldWriter<'a, GPIOD_LOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    pub fn lkey(&self) -> LKEY_R {
        LKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - LOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn lock0(&mut self) -> LOCK0_W<0> {
        LOCK0_W::new(self)
    }
    #[doc = "Bit 1 - LOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn lock1(&mut self) -> LOCK1_W<1> {
        LOCK1_W::new(self)
    }
    #[doc = "Bit 2 - LOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn lock2(&mut self) -> LOCK2_W<2> {
        LOCK2_W::new(self)
    }
    #[doc = "Bit 3 - LOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn lock3(&mut self) -> LOCK3_W<3> {
        LOCK3_W::new(self)
    }
    #[doc = "Bit 4 - LOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn lock4(&mut self) -> LOCK4_W<4> {
        LOCK4_W::new(self)
    }
    #[doc = "Bit 5 - LOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn lock5(&mut self) -> LOCK5_W<5> {
        LOCK5_W::new(self)
    }
    #[doc = "Bits 16:31 - LKEY"]
    #[inline(always)]
    #[must_use]
    pub fn lkey(&mut self) -> LKEY_W<16> {
        LKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIOD_LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_lockr](index.html) module"]
pub struct GPIOD_LOCKR_SPEC;
impl crate::RegisterSpec for GPIOD_LOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiod_lockr::R](R) reader structure"]
impl crate::Readable for GPIOD_LOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiod_lockr::W](W) writer structure"]
impl crate::Writable for GPIOD_LOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIOD_LOCKR to value 0"]
impl crate::Resettable for GPIOD_LOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
