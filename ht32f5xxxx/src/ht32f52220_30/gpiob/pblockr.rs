#[doc = "Register `PBLOCKR` reader"]
pub struct R(crate::R<PBLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBLOCKR` writer"]
pub struct W(crate::W<PBLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBLOCKR_SPEC>;
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
impl From<crate::W<PBLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBLOCK0` reader - PBLOCK0"]
pub type PBLOCK0_R = crate::BitReader;
#[doc = "Field `PBLOCK0` writer - PBLOCK0"]
pub type PBLOCK0_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK1` reader - PBLOCK1"]
pub type PBLOCK1_R = crate::BitReader;
#[doc = "Field `PBLOCK1` writer - PBLOCK1"]
pub type PBLOCK1_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK2` reader - PBLOCK2"]
pub type PBLOCK2_R = crate::BitReader;
#[doc = "Field `PBLOCK2` writer - PBLOCK2"]
pub type PBLOCK2_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK3` reader - PBLOCK3"]
pub type PBLOCK3_R = crate::BitReader;
#[doc = "Field `PBLOCK3` writer - PBLOCK3"]
pub type PBLOCK3_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK4` reader - PBLOCK4"]
pub type PBLOCK4_R = crate::BitReader;
#[doc = "Field `PBLOCK4` writer - PBLOCK4"]
pub type PBLOCK4_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK5` reader - PBLOCK5"]
pub type PBLOCK5_R = crate::BitReader;
#[doc = "Field `PBLOCK5` writer - PBLOCK5"]
pub type PBLOCK5_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK6` reader - PBLOCK6"]
pub type PBLOCK6_R = crate::BitReader;
#[doc = "Field `PBLOCK6` writer - PBLOCK6"]
pub type PBLOCK6_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK7` reader - PBLOCK7"]
pub type PBLOCK7_R = crate::BitReader;
#[doc = "Field `PBLOCK7` writer - PBLOCK7"]
pub type PBLOCK7_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK8` reader - PBLOCK8"]
pub type PBLOCK8_R = crate::BitReader;
#[doc = "Field `PBLOCK8` writer - PBLOCK8"]
pub type PBLOCK8_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK9` reader - PBLOCK9"]
pub type PBLOCK9_R = crate::BitReader;
#[doc = "Field `PBLOCK9` writer - PBLOCK9"]
pub type PBLOCK9_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK10` reader - PBLOCK10"]
pub type PBLOCK10_R = crate::BitReader;
#[doc = "Field `PBLOCK10` writer - PBLOCK10"]
pub type PBLOCK10_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK11` reader - PBLOCK11"]
pub type PBLOCK11_R = crate::BitReader;
#[doc = "Field `PBLOCK11` writer - PBLOCK11"]
pub type PBLOCK11_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK12` reader - PBLOCK12"]
pub type PBLOCK12_R = crate::BitReader;
#[doc = "Field `PBLOCK12` writer - PBLOCK12"]
pub type PBLOCK12_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK13` reader - PBLOCK13"]
pub type PBLOCK13_R = crate::BitReader;
#[doc = "Field `PBLOCK13` writer - PBLOCK13"]
pub type PBLOCK13_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLOCK14` reader - PBLOCK14"]
pub type PBLOCK14_R = crate::BitReader;
#[doc = "Field `PBLOCK14` writer - PBLOCK14"]
pub type PBLOCK14_W<'a, const O: u8> = crate::BitWriter<'a, PBLOCKR_SPEC, O>;
#[doc = "Field `PBLKEY` reader - PBLKEY"]
pub type PBLKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PBLKEY` writer - PBLKEY"]
pub type PBLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PBLOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - PBLOCK0"]
    #[inline(always)]
    pub fn pblock0(&self) -> PBLOCK0_R {
        PBLOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PBLOCK1"]
    #[inline(always)]
    pub fn pblock1(&self) -> PBLOCK1_R {
        PBLOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBLOCK2"]
    #[inline(always)]
    pub fn pblock2(&self) -> PBLOCK2_R {
        PBLOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBLOCK3"]
    #[inline(always)]
    pub fn pblock3(&self) -> PBLOCK3_R {
        PBLOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PBLOCK4"]
    #[inline(always)]
    pub fn pblock4(&self) -> PBLOCK4_R {
        PBLOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PBLOCK5"]
    #[inline(always)]
    pub fn pblock5(&self) -> PBLOCK5_R {
        PBLOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PBLOCK6"]
    #[inline(always)]
    pub fn pblock6(&self) -> PBLOCK6_R {
        PBLOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PBLOCK7"]
    #[inline(always)]
    pub fn pblock7(&self) -> PBLOCK7_R {
        PBLOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PBLOCK8"]
    #[inline(always)]
    pub fn pblock8(&self) -> PBLOCK8_R {
        PBLOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PBLOCK9"]
    #[inline(always)]
    pub fn pblock9(&self) -> PBLOCK9_R {
        PBLOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PBLOCK10"]
    #[inline(always)]
    pub fn pblock10(&self) -> PBLOCK10_R {
        PBLOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PBLOCK11"]
    #[inline(always)]
    pub fn pblock11(&self) -> PBLOCK11_R {
        PBLOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PBLOCK12"]
    #[inline(always)]
    pub fn pblock12(&self) -> PBLOCK12_R {
        PBLOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PBLOCK13"]
    #[inline(always)]
    pub fn pblock13(&self) -> PBLOCK13_R {
        PBLOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PBLOCK14"]
    #[inline(always)]
    pub fn pblock14(&self) -> PBLOCK14_R {
        PBLOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PBLKEY"]
    #[inline(always)]
    pub fn pblkey(&self) -> PBLKEY_R {
        PBLKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PBLOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn pblock0(&mut self) -> PBLOCK0_W<0> {
        PBLOCK0_W::new(self)
    }
    #[doc = "Bit 1 - PBLOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn pblock1(&mut self) -> PBLOCK1_W<1> {
        PBLOCK1_W::new(self)
    }
    #[doc = "Bit 2 - PBLOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn pblock2(&mut self) -> PBLOCK2_W<2> {
        PBLOCK2_W::new(self)
    }
    #[doc = "Bit 3 - PBLOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn pblock3(&mut self) -> PBLOCK3_W<3> {
        PBLOCK3_W::new(self)
    }
    #[doc = "Bit 4 - PBLOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn pblock4(&mut self) -> PBLOCK4_W<4> {
        PBLOCK4_W::new(self)
    }
    #[doc = "Bit 5 - PBLOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn pblock5(&mut self) -> PBLOCK5_W<5> {
        PBLOCK5_W::new(self)
    }
    #[doc = "Bit 6 - PBLOCK6"]
    #[inline(always)]
    #[must_use]
    pub fn pblock6(&mut self) -> PBLOCK6_W<6> {
        PBLOCK6_W::new(self)
    }
    #[doc = "Bit 7 - PBLOCK7"]
    #[inline(always)]
    #[must_use]
    pub fn pblock7(&mut self) -> PBLOCK7_W<7> {
        PBLOCK7_W::new(self)
    }
    #[doc = "Bit 8 - PBLOCK8"]
    #[inline(always)]
    #[must_use]
    pub fn pblock8(&mut self) -> PBLOCK8_W<8> {
        PBLOCK8_W::new(self)
    }
    #[doc = "Bit 9 - PBLOCK9"]
    #[inline(always)]
    #[must_use]
    pub fn pblock9(&mut self) -> PBLOCK9_W<9> {
        PBLOCK9_W::new(self)
    }
    #[doc = "Bit 10 - PBLOCK10"]
    #[inline(always)]
    #[must_use]
    pub fn pblock10(&mut self) -> PBLOCK10_W<10> {
        PBLOCK10_W::new(self)
    }
    #[doc = "Bit 11 - PBLOCK11"]
    #[inline(always)]
    #[must_use]
    pub fn pblock11(&mut self) -> PBLOCK11_W<11> {
        PBLOCK11_W::new(self)
    }
    #[doc = "Bit 12 - PBLOCK12"]
    #[inline(always)]
    #[must_use]
    pub fn pblock12(&mut self) -> PBLOCK12_W<12> {
        PBLOCK12_W::new(self)
    }
    #[doc = "Bit 13 - PBLOCK13"]
    #[inline(always)]
    #[must_use]
    pub fn pblock13(&mut self) -> PBLOCK13_W<13> {
        PBLOCK13_W::new(self)
    }
    #[doc = "Bit 14 - PBLOCK14"]
    #[inline(always)]
    #[must_use]
    pub fn pblock14(&mut self) -> PBLOCK14_W<14> {
        PBLOCK14_W::new(self)
    }
    #[doc = "Bits 16:31 - PBLKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pblkey(&mut self) -> PBLKEY_W<16> {
        PBLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBLOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pblockr](index.html) module"]
pub struct PBLOCKR_SPEC;
impl crate::RegisterSpec for PBLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pblockr::R](R) reader structure"]
impl crate::Readable for PBLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pblockr::W](W) writer structure"]
impl crate::Writable for PBLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBLOCKR to value 0"]
impl crate::Resettable for PBLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
