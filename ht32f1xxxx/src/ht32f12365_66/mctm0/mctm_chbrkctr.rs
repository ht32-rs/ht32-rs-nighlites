#[doc = "Register `MCTM_CHBRKCTR` reader"]
pub struct R(crate::R<MCTM_CHBRKCTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTM_CHBRKCTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTM_CHBRKCTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTM_CHBRKCTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTM_CHBRKCTR` writer"]
pub struct W(crate::W<MCTM_CHBRKCTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTM_CHBRKCTR_SPEC>;
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
impl From<crate::W<MCTM_CHBRKCTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTM_CHBRKCTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKE0` reader - BKE0"]
pub type BKE0_R = crate::BitReader;
#[doc = "Field `BKE0` writer - BKE0"]
pub type BKE0_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `BKP0` reader - BKP0"]
pub type BKP0_R = crate::BitReader;
#[doc = "Field `BKP0` writer - BKP0"]
pub type BKP0_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `BKE1` reader - BKE1"]
pub type BKE1_R = crate::BitReader;
#[doc = "Field `BKE1` writer - BKE1"]
pub type BKE1_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `BKP1` reader - BKP1"]
pub type BKP1_R = crate::BitReader;
#[doc = "Field `BKP1` writer - BKP1"]
pub type BKP1_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `CHMOE` reader - CHMOE"]
pub type CHMOE_R = crate::BitReader;
#[doc = "Field `CHMOE` writer - CHMOE"]
pub type CHMOE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `CHAOE` reader - CHAOE"]
pub type CHAOE_R = crate::BitReader;
#[doc = "Field `CHAOE` writer - CHAOE"]
pub type CHAOE_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `BK1SEL` reader - BK1SEL"]
pub type BK1SEL_R = crate::BitReader;
#[doc = "Field `BK1SEL` writer - BK1SEL"]
pub type BK1SEL_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `BKF0` reader - BKF0"]
pub type BKF0_R = crate::FieldReader;
#[doc = "Field `BKF0` writer - BKF0"]
pub type BKF0_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CHBRKCTR_SPEC, 4, O>;
#[doc = "Field `BKF1` reader - BKF1"]
pub type BKF1_R = crate::FieldReader;
#[doc = "Field `BKF1` writer - BKF1"]
pub type BKF1_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CHBRKCTR_SPEC, 4, O>;
#[doc = "Field `LOCKLV` reader - LOCKLV"]
pub type LOCKLV_R = crate::FieldReader;
#[doc = "Field `LOCKLV` writer - LOCKLV"]
pub type LOCKLV_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CHBRKCTR_SPEC, 2, O>;
#[doc = "Field `GFSEL0` reader - GFSEL0"]
pub type GFSEL0_R = crate::BitReader;
#[doc = "Field `GFSEL0` writer - GFSEL0"]
pub type GFSEL0_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `GFSEL1` reader - GFSEL1"]
pub type GFSEL1_R = crate::BitReader;
#[doc = "Field `GFSEL1` writer - GFSEL1"]
pub type GFSEL1_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `CHOSSI` reader - CHOSSI"]
pub type CHOSSI_R = crate::BitReader;
#[doc = "Field `CHOSSI` writer - CHOSSI"]
pub type CHOSSI_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `CHOSSR` reader - CHOSSR"]
pub type CHOSSR_R = crate::BitReader;
#[doc = "Field `CHOSSR` writer - CHOSSR"]
pub type CHOSSR_W<'a, const O: u8> = crate::BitWriter<'a, MCTM_CHBRKCTR_SPEC, O>;
#[doc = "Field `CHDTG` reader - CHDTG"]
pub type CHDTG_R = crate::FieldReader;
#[doc = "Field `CHDTG` writer - CHDTG"]
pub type CHDTG_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_CHBRKCTR_SPEC, 8, O>;
impl R {
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    pub fn bke0(&self) -> BKE0_R {
        BKE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    pub fn bkp0(&self) -> BKP0_R {
        BKP0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BKE1"]
    #[inline(always)]
    pub fn bke1(&self) -> BKE1_R {
        BKE1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BKP1"]
    #[inline(always)]
    pub fn bkp1(&self) -> BKP1_R {
        BKP1_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 6 - BK1SEL"]
    #[inline(always)]
    pub fn bk1sel(&self) -> BK1SEL_R {
        BK1SEL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    pub fn bkf0(&self) -> BKF0_R {
        BKF0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - BKF1"]
    #[inline(always)]
    pub fn bkf1(&self) -> BKF1_R {
        BKF1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    pub fn locklv(&self) -> LOCKLV_R {
        LOCKLV_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    pub fn gfsel0(&self) -> GFSEL0_R {
        GFSEL0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - GFSEL1"]
    #[inline(always)]
    pub fn gfsel1(&self) -> GFSEL1_R {
        GFSEL1_R::new(((self.bits >> 19) & 1) != 0)
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
    #[doc = "Bit 0 - BKE0"]
    #[inline(always)]
    #[must_use]
    pub fn bke0(&mut self) -> BKE0_W<0> {
        BKE0_W::new(self)
    }
    #[doc = "Bit 1 - BKP0"]
    #[inline(always)]
    #[must_use]
    pub fn bkp0(&mut self) -> BKP0_W<1> {
        BKP0_W::new(self)
    }
    #[doc = "Bit 2 - BKE1"]
    #[inline(always)]
    #[must_use]
    pub fn bke1(&mut self) -> BKE1_W<2> {
        BKE1_W::new(self)
    }
    #[doc = "Bit 3 - BKP1"]
    #[inline(always)]
    #[must_use]
    pub fn bkp1(&mut self) -> BKP1_W<3> {
        BKP1_W::new(self)
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
    #[doc = "Bit 6 - BK1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn bk1sel(&mut self) -> BK1SEL_W<6> {
        BK1SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - BKF0"]
    #[inline(always)]
    #[must_use]
    pub fn bkf0(&mut self) -> BKF0_W<8> {
        BKF0_W::new(self)
    }
    #[doc = "Bits 12:15 - BKF1"]
    #[inline(always)]
    #[must_use]
    pub fn bkf1(&mut self) -> BKF1_W<12> {
        BKF1_W::new(self)
    }
    #[doc = "Bits 16:17 - LOCKLV"]
    #[inline(always)]
    #[must_use]
    pub fn locklv(&mut self) -> LOCKLV_W<16> {
        LOCKLV_W::new(self)
    }
    #[doc = "Bit 18 - GFSEL0"]
    #[inline(always)]
    #[must_use]
    pub fn gfsel0(&mut self) -> GFSEL0_W<18> {
        GFSEL0_W::new(self)
    }
    #[doc = "Bit 19 - GFSEL1"]
    #[inline(always)]
    #[must_use]
    pub fn gfsel1(&mut self) -> GFSEL1_W<19> {
        GFSEL1_W::new(self)
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
#[doc = "MCTM_CHBRKCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_chbrkctr](index.html) module"]
pub struct MCTM_CHBRKCTR_SPEC;
impl crate::RegisterSpec for MCTM_CHBRKCTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctm_chbrkctr::R](R) reader structure"]
impl crate::Readable for MCTM_CHBRKCTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctm_chbrkctr::W](W) writer structure"]
impl crate::Writable for MCTM_CHBRKCTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTM_CHBRKCTR to value 0"]
impl crate::Resettable for MCTM_CHBRKCTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
