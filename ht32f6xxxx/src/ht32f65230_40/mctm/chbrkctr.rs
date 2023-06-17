#[doc = "Register `CHBRKCTR` reader"]
pub struct R(crate::R<CHBRKCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHBRKCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHBRKCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHBRKCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHBRKCTR` writer"]
pub struct W(crate::W<CHBRKCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHBRKCTR_SPEC>;
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
impl From<crate::W<CHBRKCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHBRKCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BK0E` reader - BK0E"]
pub type BK0E_R = crate::BitReader;
#[doc = "Field `BK0E` writer - BK0E"]
pub type BK0E_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `BK0P` reader - BK0P"]
pub type BK0P_R = crate::BitReader;
#[doc = "Field `BK0P` writer - BK0P"]
pub type BK0P_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `BK1E` reader - BK1E"]
pub type BK1E_R = crate::BitReader;
#[doc = "Field `BK1E` writer - BK1E"]
pub type BK1E_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `BK1P` reader - BK1P"]
pub type BK1P_R = crate::BitReader;
#[doc = "Field `BK1P` writer - BK1P"]
pub type BK1P_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `CHMOE` reader - CHMOE"]
pub type CHMOE_R = crate::BitReader;
#[doc = "Field `CHMOE` writer - CHMOE"]
pub type CHMOE_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `CHAOE` reader - CHAOE"]
pub type CHAOE_R = crate::BitReader;
#[doc = "Field `CHAOE` writer - CHAOE"]
pub type CHAOE_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `BK0FN` reader - BK0FN"]
pub type BK0FN_R = crate::FieldReader;
#[doc = "Field `BK0FN` writer - BK0FN"]
pub type BK0FN_W<'a, const O: u8> = crate::FieldWriter<'a, CHBRKCTR_SPEC, 2, O>;
#[doc = "Field `BK0FF` reader - BK0FF"]
pub type BK0FF_R = crate::FieldReader;
#[doc = "Field `BK0FF` writer - BK0FF"]
pub type BK0FF_W<'a, const O: u8> = crate::FieldWriter<'a, CHBRKCTR_SPEC, 2, O>;
#[doc = "Field `BK1FN` reader - BK1FN"]
pub type BK1FN_R = crate::FieldReader;
#[doc = "Field `BK1FN` writer - BK1FN"]
pub type BK1FN_W<'a, const O: u8> = crate::FieldWriter<'a, CHBRKCTR_SPEC, 2, O>;
#[doc = "Field `BK1FF` reader - BK1FF"]
pub type BK1FF_R = crate::FieldReader;
#[doc = "Field `BK1FF` writer - BK1FF"]
pub type BK1FF_W<'a, const O: u8> = crate::FieldWriter<'a, CHBRKCTR_SPEC, 2, O>;
#[doc = "Field `LOCKLV` reader - LOCKLV"]
pub type LOCKLV_R = crate::FieldReader;
#[doc = "Field `LOCKLV` writer - LOCKLV"]
pub type LOCKLV_W<'a, const O: u8> = crate::FieldWriter<'a, CHBRKCTR_SPEC, 2, O>;
#[doc = "Field `GFSEL` reader - GFSEL"]
pub type GFSEL_R = crate::BitReader;
#[doc = "Field `GFSEL` writer - GFSEL"]
pub type GFSEL_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `CHOSSI` reader - CHOSSI"]
pub type CHOSSI_R = crate::BitReader;
#[doc = "Field `CHOSSI` writer - CHOSSI"]
pub type CHOSSI_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `CHOSSR` reader - CHOSSR"]
pub type CHOSSR_R = crate::BitReader;
#[doc = "Field `CHOSSR` writer - CHOSSR"]
pub type CHOSSR_W<'a, const O: u8> = crate::BitWriter<'a, CHBRKCTR_SPEC, O>;
#[doc = "Field `CHDTG` reader - CHDTG"]
pub type CHDTG_R = crate::FieldReader;
#[doc = "Field `CHDTG` writer - CHDTG"]
pub type CHDTG_W<'a, const O: u8> = crate::FieldWriter<'a, CHBRKCTR_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - BK0E"]
    #[inline(always)]
    pub fn bk0e(&self) -> BK0E_R {
        BK0E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BK0P"]
    #[inline(always)]
    pub fn bk0p(&self) -> BK0P_R {
        BK0P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BK1E"]
    #[inline(always)]
    pub fn bk1e(&self) -> BK1E_R {
        BK1E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BK1P"]
    #[inline(always)]
    pub fn bk1p(&self) -> BK1P_R {
        BK1P_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    pub fn chmoe(&self) -> CHMOE_R {
        CHMOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    pub fn chaoe(&self) -> CHAOE_R {
        CHAOE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - BK0FN"]
    #[inline(always)]
    pub fn bk0fn(&self) -> BK0FN_R {
        BK0FN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - BK0FF"]
    #[inline(always)]
    pub fn bk0ff(&self) -> BK0FF_R {
        BK0FF_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - BK1FN"]
    #[inline(always)]
    pub fn bk1fn(&self) -> BK1FN_R {
        BK1FN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - BK1FF"]
    #[inline(always)]
    pub fn bk1ff(&self) -> BK1FF_R {
        BK1FF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    pub fn locklv(&self) -> LOCKLV_R {
        LOCKLV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - GFSEL"]
    #[inline(always)]
    pub fn gfsel(&self) -> GFSEL_R {
        GFSEL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    pub fn chossi(&self) -> CHOSSI_R {
        CHOSSI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    pub fn chossr(&self) -> CHOSSR_R {
        CHOSSR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    pub fn chdtg(&self) -> CHDTG_R {
        CHDTG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - BK0E"]
    #[inline(always)]
    #[must_use]
    pub fn bk0e(&mut self) -> BK0E_W<0> {
        BK0E_W::new(self)
    }
    #[doc = "Bit 1 - BK0P"]
    #[inline(always)]
    #[must_use]
    pub fn bk0p(&mut self) -> BK0P_W<1> {
        BK0P_W::new(self)
    }
    #[doc = "Bit 2 - BK1E"]
    #[inline(always)]
    #[must_use]
    pub fn bk1e(&mut self) -> BK1E_W<2> {
        BK1E_W::new(self)
    }
    #[doc = "Bit 3 - BK1P"]
    #[inline(always)]
    #[must_use]
    pub fn bk1p(&mut self) -> BK1P_W<3> {
        BK1P_W::new(self)
    }
    #[doc = "Bit 4 - CHMOE"]
    #[inline(always)]
    #[must_use]
    pub fn chmoe(&mut self) -> CHMOE_W<4> {
        CHMOE_W::new(self)
    }
    #[doc = "Bit 5 - CHAOE"]
    #[inline(always)]
    #[must_use]
    pub fn chaoe(&mut self) -> CHAOE_W<5> {
        CHAOE_W::new(self)
    }
    #[doc = "Bits 8:9 - BK0FN"]
    #[inline(always)]
    #[must_use]
    pub fn bk0fn(&mut self) -> BK0FN_W<8> {
        BK0FN_W::new(self)
    }
    #[doc = "Bits 10:11 - BK0FF"]
    #[inline(always)]
    #[must_use]
    pub fn bk0ff(&mut self) -> BK0FF_W<10> {
        BK0FF_W::new(self)
    }
    #[doc = "Bits 12:13 - BK1FN"]
    #[inline(always)]
    #[must_use]
    pub fn bk1fn(&mut self) -> BK1FN_W<12> {
        BK1FN_W::new(self)
    }
    #[doc = "Bits 14:15 - BK1FF"]
    #[inline(always)]
    #[must_use]
    pub fn bk1ff(&mut self) -> BK1FF_W<14> {
        BK1FF_W::new(self)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    #[must_use]
    pub fn locklv(&mut self) -> LOCKLV_W<16> {
        LOCKLV_W::new(self)
    }
    #[doc = "Bit 18 - GFSEL"]
    #[inline(always)]
    #[must_use]
    pub fn gfsel(&mut self) -> GFSEL_W<18> {
        GFSEL_W::new(self)
    }
    #[doc = "Bit 20 - CHOSSI"]
    #[inline(always)]
    #[must_use]
    pub fn chossi(&mut self) -> CHOSSI_W<20> {
        CHOSSI_W::new(self)
    }
    #[doc = "Bit 21 - CHOSSR"]
    #[inline(always)]
    #[must_use]
    pub fn chossr(&mut self) -> CHOSSR_W<21> {
        CHOSSR_W::new(self)
    }
    #[doc = "Bits 24:31 - CHDTG"]
    #[inline(always)]
    #[must_use]
    pub fn chdtg(&mut self) -> CHDTG_W<24> {
        CHDTG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHBRKCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbrkctr](index.html) module"]
pub struct CHBRKCTR_SPEC;
impl crate::RegisterSpec for CHBRKCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chbrkctr::R](R) reader structure"]
impl crate::Readable for CHBRKCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chbrkctr::W](W) writer structure"]
impl crate::Writable for CHBRKCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHBRKCTR to value 0"]
impl crate::Resettable for CHBRKCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
