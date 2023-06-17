#[doc = "Register `PDLOCKR` reader"]
pub struct R(crate::R<PDLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDLOCKR` writer"]
pub struct W(crate::W<PDLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDLOCKR_SPEC>;
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
impl From<crate::W<PDLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDLOCK0` reader - PDLOCK0"]
pub type PDLOCK0_R = crate::BitReader;
#[doc = "Field `PDLOCK0` writer - PDLOCK0"]
pub type PDLOCK0_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK1` reader - PDLOCK1"]
pub type PDLOCK1_R = crate::BitReader;
#[doc = "Field `PDLOCK1` writer - PDLOCK1"]
pub type PDLOCK1_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK2` reader - PDLOCK2"]
pub type PDLOCK2_R = crate::BitReader;
#[doc = "Field `PDLOCK2` writer - PDLOCK2"]
pub type PDLOCK2_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK3` reader - PDLOCK3"]
pub type PDLOCK3_R = crate::BitReader;
#[doc = "Field `PDLOCK3` writer - PDLOCK3"]
pub type PDLOCK3_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK4` reader - PDLOCK4"]
pub type PDLOCK4_R = crate::BitReader;
#[doc = "Field `PDLOCK4` writer - PDLOCK4"]
pub type PDLOCK4_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK5` reader - PDLOCK5"]
pub type PDLOCK5_R = crate::BitReader;
#[doc = "Field `PDLOCK5` writer - PDLOCK5"]
pub type PDLOCK5_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK6` reader - PDLOCK6"]
pub type PDLOCK6_R = crate::BitReader;
#[doc = "Field `PDLOCK6` writer - PDLOCK6"]
pub type PDLOCK6_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK7` reader - PDLOCK7"]
pub type PDLOCK7_R = crate::BitReader;
#[doc = "Field `PDLOCK7` writer - PDLOCK7"]
pub type PDLOCK7_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK8` reader - PDLOCK8"]
pub type PDLOCK8_R = crate::BitReader;
#[doc = "Field `PDLOCK8` writer - PDLOCK8"]
pub type PDLOCK8_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK9` reader - PDLOCK9"]
pub type PDLOCK9_R = crate::BitReader;
#[doc = "Field `PDLOCK9` writer - PDLOCK9"]
pub type PDLOCK9_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK10` reader - PDLOCK10"]
pub type PDLOCK10_R = crate::BitReader;
#[doc = "Field `PDLOCK10` writer - PDLOCK10"]
pub type PDLOCK10_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK11` reader - PDLOCK11"]
pub type PDLOCK11_R = crate::BitReader;
#[doc = "Field `PDLOCK11` writer - PDLOCK11"]
pub type PDLOCK11_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK12` reader - PDLOCK12"]
pub type PDLOCK12_R = crate::BitReader;
#[doc = "Field `PDLOCK12` writer - PDLOCK12"]
pub type PDLOCK12_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK13` reader - PDLOCK13"]
pub type PDLOCK13_R = crate::BitReader;
#[doc = "Field `PDLOCK13` writer - PDLOCK13"]
pub type PDLOCK13_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK14` reader - PDLOCK14"]
pub type PDLOCK14_R = crate::BitReader;
#[doc = "Field `PDLOCK14` writer - PDLOCK14"]
pub type PDLOCK14_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLOCK15` reader - PDLOCK15"]
pub type PDLOCK15_R = crate::BitReader;
#[doc = "Field `PDLOCK15` writer - PDLOCK15"]
pub type PDLOCK15_W<'a, const O: u8> = crate::BitWriter<'a, PDLOCKR_SPEC, O>;
#[doc = "Field `PDLKEY` reader - PDLKEY"]
pub type PDLKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PDLKEY` writer - PDLKEY"]
pub type PDLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PDLOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - PDLOCK0"]
    #[inline(always)]
    pub fn pdlock0(&self) -> PDLOCK0_R {
        PDLOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDLOCK1"]
    #[inline(always)]
    pub fn pdlock1(&self) -> PDLOCK1_R {
        PDLOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDLOCK2"]
    #[inline(always)]
    pub fn pdlock2(&self) -> PDLOCK2_R {
        PDLOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDLOCK3"]
    #[inline(always)]
    pub fn pdlock3(&self) -> PDLOCK3_R {
        PDLOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDLOCK4"]
    #[inline(always)]
    pub fn pdlock4(&self) -> PDLOCK4_R {
        PDLOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDLOCK5"]
    #[inline(always)]
    pub fn pdlock5(&self) -> PDLOCK5_R {
        PDLOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDLOCK6"]
    #[inline(always)]
    pub fn pdlock6(&self) -> PDLOCK6_R {
        PDLOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDLOCK7"]
    #[inline(always)]
    pub fn pdlock7(&self) -> PDLOCK7_R {
        PDLOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PDLOCK8"]
    #[inline(always)]
    pub fn pdlock8(&self) -> PDLOCK8_R {
        PDLOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PDLOCK9"]
    #[inline(always)]
    pub fn pdlock9(&self) -> PDLOCK9_R {
        PDLOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PDLOCK10"]
    #[inline(always)]
    pub fn pdlock10(&self) -> PDLOCK10_R {
        PDLOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PDLOCK11"]
    #[inline(always)]
    pub fn pdlock11(&self) -> PDLOCK11_R {
        PDLOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PDLOCK12"]
    #[inline(always)]
    pub fn pdlock12(&self) -> PDLOCK12_R {
        PDLOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PDLOCK13"]
    #[inline(always)]
    pub fn pdlock13(&self) -> PDLOCK13_R {
        PDLOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PDLOCK14"]
    #[inline(always)]
    pub fn pdlock14(&self) -> PDLOCK14_R {
        PDLOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PDLOCK15"]
    #[inline(always)]
    pub fn pdlock15(&self) -> PDLOCK15_R {
        PDLOCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PDLKEY"]
    #[inline(always)]
    pub fn pdlkey(&self) -> PDLKEY_R {
        PDLKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PDLOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock0(&mut self) -> PDLOCK0_W<0> {
        PDLOCK0_W::new(self)
    }
    #[doc = "Bit 1 - PDLOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock1(&mut self) -> PDLOCK1_W<1> {
        PDLOCK1_W::new(self)
    }
    #[doc = "Bit 2 - PDLOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock2(&mut self) -> PDLOCK2_W<2> {
        PDLOCK2_W::new(self)
    }
    #[doc = "Bit 3 - PDLOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock3(&mut self) -> PDLOCK3_W<3> {
        PDLOCK3_W::new(self)
    }
    #[doc = "Bit 4 - PDLOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock4(&mut self) -> PDLOCK4_W<4> {
        PDLOCK4_W::new(self)
    }
    #[doc = "Bit 5 - PDLOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock5(&mut self) -> PDLOCK5_W<5> {
        PDLOCK5_W::new(self)
    }
    #[doc = "Bit 6 - PDLOCK6"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock6(&mut self) -> PDLOCK6_W<6> {
        PDLOCK6_W::new(self)
    }
    #[doc = "Bit 7 - PDLOCK7"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock7(&mut self) -> PDLOCK7_W<7> {
        PDLOCK7_W::new(self)
    }
    #[doc = "Bit 8 - PDLOCK8"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock8(&mut self) -> PDLOCK8_W<8> {
        PDLOCK8_W::new(self)
    }
    #[doc = "Bit 9 - PDLOCK9"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock9(&mut self) -> PDLOCK9_W<9> {
        PDLOCK9_W::new(self)
    }
    #[doc = "Bit 10 - PDLOCK10"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock10(&mut self) -> PDLOCK10_W<10> {
        PDLOCK10_W::new(self)
    }
    #[doc = "Bit 11 - PDLOCK11"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock11(&mut self) -> PDLOCK11_W<11> {
        PDLOCK11_W::new(self)
    }
    #[doc = "Bit 12 - PDLOCK12"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock12(&mut self) -> PDLOCK12_W<12> {
        PDLOCK12_W::new(self)
    }
    #[doc = "Bit 13 - PDLOCK13"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock13(&mut self) -> PDLOCK13_W<13> {
        PDLOCK13_W::new(self)
    }
    #[doc = "Bit 14 - PDLOCK14"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock14(&mut self) -> PDLOCK14_W<14> {
        PDLOCK14_W::new(self)
    }
    #[doc = "Bit 15 - PDLOCK15"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock15(&mut self) -> PDLOCK15_W<15> {
        PDLOCK15_W::new(self)
    }
    #[doc = "Bits 16:31 - PDLKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pdlkey(&mut self) -> PDLKEY_W<16> {
        PDLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDLOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdlockr](index.html) module"]
pub struct PDLOCKR_SPEC;
impl crate::RegisterSpec for PDLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdlockr::R](R) reader structure"]
impl crate::Readable for PDLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdlockr::W](W) writer structure"]
impl crate::Writable for PDLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDLOCKR to value 0"]
impl crate::Resettable for PDLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
