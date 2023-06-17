#[doc = "Register `PCLOCKR` reader"]
pub struct R(crate::R<PCLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLOCKR` writer"]
pub struct W(crate::W<PCLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLOCKR_SPEC>;
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
impl From<crate::W<PCLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCLOCK0` reader - PCLOCK0"]
pub type PCLOCK0_R = crate::BitReader;
#[doc = "Field `PCLOCK0` writer - PCLOCK0"]
pub type PCLOCK0_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK1` reader - PCLOCK1"]
pub type PCLOCK1_R = crate::BitReader;
#[doc = "Field `PCLOCK1` writer - PCLOCK1"]
pub type PCLOCK1_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK2` reader - PCLOCK2"]
pub type PCLOCK2_R = crate::BitReader;
#[doc = "Field `PCLOCK2` writer - PCLOCK2"]
pub type PCLOCK2_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK3` reader - PCLOCK3"]
pub type PCLOCK3_R = crate::BitReader;
#[doc = "Field `PCLOCK3` writer - PCLOCK3"]
pub type PCLOCK3_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK4` reader - PCLOCK4"]
pub type PCLOCK4_R = crate::BitReader;
#[doc = "Field `PCLOCK4` writer - PCLOCK4"]
pub type PCLOCK4_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK5` reader - PCLOCK5"]
pub type PCLOCK5_R = crate::BitReader;
#[doc = "Field `PCLOCK5` writer - PCLOCK5"]
pub type PCLOCK5_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK6` reader - PCLOCK6"]
pub type PCLOCK6_R = crate::BitReader;
#[doc = "Field `PCLOCK6` writer - PCLOCK6"]
pub type PCLOCK6_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK7` reader - PCLOCK7"]
pub type PCLOCK7_R = crate::BitReader;
#[doc = "Field `PCLOCK7` writer - PCLOCK7"]
pub type PCLOCK7_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK8` reader - PCLOCK8"]
pub type PCLOCK8_R = crate::BitReader;
#[doc = "Field `PCLOCK8` writer - PCLOCK8"]
pub type PCLOCK8_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK9` reader - PCLOCK9"]
pub type PCLOCK9_R = crate::BitReader;
#[doc = "Field `PCLOCK9` writer - PCLOCK9"]
pub type PCLOCK9_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK10` reader - PCLOCK10"]
pub type PCLOCK10_R = crate::BitReader;
#[doc = "Field `PCLOCK10` writer - PCLOCK10"]
pub type PCLOCK10_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK11` reader - PCLOCK11"]
pub type PCLOCK11_R = crate::BitReader;
#[doc = "Field `PCLOCK11` writer - PCLOCK11"]
pub type PCLOCK11_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK12` reader - PCLOCK12"]
pub type PCLOCK12_R = crate::BitReader;
#[doc = "Field `PCLOCK12` writer - PCLOCK12"]
pub type PCLOCK12_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK13` reader - PCLOCK13"]
pub type PCLOCK13_R = crate::BitReader;
#[doc = "Field `PCLOCK13` writer - PCLOCK13"]
pub type PCLOCK13_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK14` reader - PCLOCK14"]
pub type PCLOCK14_R = crate::BitReader;
#[doc = "Field `PCLOCK14` writer - PCLOCK14"]
pub type PCLOCK14_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLOCK15` reader - PCLOCK15"]
pub type PCLOCK15_R = crate::BitReader;
#[doc = "Field `PCLOCK15` writer - PCLOCK15"]
pub type PCLOCK15_W<'a, const O: u8> = crate::BitWriter<'a, PCLOCKR_SPEC, O>;
#[doc = "Field `PCLKEY` reader - PCLKEY"]
pub type PCLKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PCLKEY` writer - PCLKEY"]
pub type PCLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PCLOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - PCLOCK0"]
    #[inline(always)]
    pub fn pclock0(&self) -> PCLOCK0_R {
        PCLOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCLOCK1"]
    #[inline(always)]
    pub fn pclock1(&self) -> PCLOCK1_R {
        PCLOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCLOCK2"]
    #[inline(always)]
    pub fn pclock2(&self) -> PCLOCK2_R {
        PCLOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCLOCK3"]
    #[inline(always)]
    pub fn pclock3(&self) -> PCLOCK3_R {
        PCLOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PCLOCK4"]
    #[inline(always)]
    pub fn pclock4(&self) -> PCLOCK4_R {
        PCLOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PCLOCK5"]
    #[inline(always)]
    pub fn pclock5(&self) -> PCLOCK5_R {
        PCLOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PCLOCK6"]
    #[inline(always)]
    pub fn pclock6(&self) -> PCLOCK6_R {
        PCLOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PCLOCK7"]
    #[inline(always)]
    pub fn pclock7(&self) -> PCLOCK7_R {
        PCLOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PCLOCK8"]
    #[inline(always)]
    pub fn pclock8(&self) -> PCLOCK8_R {
        PCLOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCLOCK9"]
    #[inline(always)]
    pub fn pclock9(&self) -> PCLOCK9_R {
        PCLOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PCLOCK10"]
    #[inline(always)]
    pub fn pclock10(&self) -> PCLOCK10_R {
        PCLOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PCLOCK11"]
    #[inline(always)]
    pub fn pclock11(&self) -> PCLOCK11_R {
        PCLOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PCLOCK12"]
    #[inline(always)]
    pub fn pclock12(&self) -> PCLOCK12_R {
        PCLOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PCLOCK13"]
    #[inline(always)]
    pub fn pclock13(&self) -> PCLOCK13_R {
        PCLOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PCLOCK14"]
    #[inline(always)]
    pub fn pclock14(&self) -> PCLOCK14_R {
        PCLOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCLOCK15"]
    #[inline(always)]
    pub fn pclock15(&self) -> PCLOCK15_R {
        PCLOCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PCLKEY"]
    #[inline(always)]
    pub fn pclkey(&self) -> PCLKEY_R {
        PCLKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PCLOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn pclock0(&mut self) -> PCLOCK0_W<0> {
        PCLOCK0_W::new(self)
    }
    #[doc = "Bit 1 - PCLOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn pclock1(&mut self) -> PCLOCK1_W<1> {
        PCLOCK1_W::new(self)
    }
    #[doc = "Bit 2 - PCLOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn pclock2(&mut self) -> PCLOCK2_W<2> {
        PCLOCK2_W::new(self)
    }
    #[doc = "Bit 3 - PCLOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn pclock3(&mut self) -> PCLOCK3_W<3> {
        PCLOCK3_W::new(self)
    }
    #[doc = "Bit 4 - PCLOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn pclock4(&mut self) -> PCLOCK4_W<4> {
        PCLOCK4_W::new(self)
    }
    #[doc = "Bit 5 - PCLOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn pclock5(&mut self) -> PCLOCK5_W<5> {
        PCLOCK5_W::new(self)
    }
    #[doc = "Bit 6 - PCLOCK6"]
    #[inline(always)]
    #[must_use]
    pub fn pclock6(&mut self) -> PCLOCK6_W<6> {
        PCLOCK6_W::new(self)
    }
    #[doc = "Bit 7 - PCLOCK7"]
    #[inline(always)]
    #[must_use]
    pub fn pclock7(&mut self) -> PCLOCK7_W<7> {
        PCLOCK7_W::new(self)
    }
    #[doc = "Bit 8 - PCLOCK8"]
    #[inline(always)]
    #[must_use]
    pub fn pclock8(&mut self) -> PCLOCK8_W<8> {
        PCLOCK8_W::new(self)
    }
    #[doc = "Bit 9 - PCLOCK9"]
    #[inline(always)]
    #[must_use]
    pub fn pclock9(&mut self) -> PCLOCK9_W<9> {
        PCLOCK9_W::new(self)
    }
    #[doc = "Bit 10 - PCLOCK10"]
    #[inline(always)]
    #[must_use]
    pub fn pclock10(&mut self) -> PCLOCK10_W<10> {
        PCLOCK10_W::new(self)
    }
    #[doc = "Bit 11 - PCLOCK11"]
    #[inline(always)]
    #[must_use]
    pub fn pclock11(&mut self) -> PCLOCK11_W<11> {
        PCLOCK11_W::new(self)
    }
    #[doc = "Bit 12 - PCLOCK12"]
    #[inline(always)]
    #[must_use]
    pub fn pclock12(&mut self) -> PCLOCK12_W<12> {
        PCLOCK12_W::new(self)
    }
    #[doc = "Bit 13 - PCLOCK13"]
    #[inline(always)]
    #[must_use]
    pub fn pclock13(&mut self) -> PCLOCK13_W<13> {
        PCLOCK13_W::new(self)
    }
    #[doc = "Bit 14 - PCLOCK14"]
    #[inline(always)]
    #[must_use]
    pub fn pclock14(&mut self) -> PCLOCK14_W<14> {
        PCLOCK14_W::new(self)
    }
    #[doc = "Bit 15 - PCLOCK15"]
    #[inline(always)]
    #[must_use]
    pub fn pclock15(&mut self) -> PCLOCK15_W<15> {
        PCLOCK15_W::new(self)
    }
    #[doc = "Bits 16:31 - PCLKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pclkey(&mut self) -> PCLKEY_W<16> {
        PCLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCLOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclockr](index.html) module"]
pub struct PCLOCKR_SPEC;
impl crate::RegisterSpec for PCLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclockr::R](R) reader structure"]
impl crate::Readable for PCLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclockr::W](W) writer structure"]
impl crate::Writable for PCLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLOCKR to value 0"]
impl crate::Resettable for PCLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
