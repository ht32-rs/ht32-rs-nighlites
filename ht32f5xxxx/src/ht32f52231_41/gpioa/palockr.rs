#[doc = "Register `PALOCKR` reader"]
pub struct R(crate::R<PALOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PALOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PALOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PALOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PALOCKR` writer"]
pub struct W(crate::W<PALOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PALOCKR_SPEC>;
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
impl From<crate::W<PALOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PALOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALOCK0` reader - PALOCK0"]
pub type PALOCK0_R = crate::BitReader;
#[doc = "Field `PALOCK0` writer - PALOCK0"]
pub type PALOCK0_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK1` reader - PALOCK1"]
pub type PALOCK1_R = crate::BitReader;
#[doc = "Field `PALOCK1` writer - PALOCK1"]
pub type PALOCK1_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK2` reader - PALOCK2"]
pub type PALOCK2_R = crate::BitReader;
#[doc = "Field `PALOCK2` writer - PALOCK2"]
pub type PALOCK2_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK3` reader - PALOCK3"]
pub type PALOCK3_R = crate::BitReader;
#[doc = "Field `PALOCK3` writer - PALOCK3"]
pub type PALOCK3_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK4` reader - PALOCK4"]
pub type PALOCK4_R = crate::BitReader;
#[doc = "Field `PALOCK4` writer - PALOCK4"]
pub type PALOCK4_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK5` reader - PALOCK5"]
pub type PALOCK5_R = crate::BitReader;
#[doc = "Field `PALOCK5` writer - PALOCK5"]
pub type PALOCK5_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK6` reader - PALOCK6"]
pub type PALOCK6_R = crate::BitReader;
#[doc = "Field `PALOCK6` writer - PALOCK6"]
pub type PALOCK6_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK7` reader - PALOCK7"]
pub type PALOCK7_R = crate::BitReader;
#[doc = "Field `PALOCK7` writer - PALOCK7"]
pub type PALOCK7_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK8` reader - PALOCK8"]
pub type PALOCK8_R = crate::BitReader;
#[doc = "Field `PALOCK8` writer - PALOCK8"]
pub type PALOCK8_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK9` reader - PALOCK9"]
pub type PALOCK9_R = crate::BitReader;
#[doc = "Field `PALOCK9` writer - PALOCK9"]
pub type PALOCK9_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK10` reader - PALOCK10"]
pub type PALOCK10_R = crate::BitReader;
#[doc = "Field `PALOCK10` writer - PALOCK10"]
pub type PALOCK10_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK11` reader - PALOCK11"]
pub type PALOCK11_R = crate::BitReader;
#[doc = "Field `PALOCK11` writer - PALOCK11"]
pub type PALOCK11_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK12` reader - PALOCK12"]
pub type PALOCK12_R = crate::BitReader;
#[doc = "Field `PALOCK12` writer - PALOCK12"]
pub type PALOCK12_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK13` reader - PALOCK13"]
pub type PALOCK13_R = crate::BitReader;
#[doc = "Field `PALOCK13` writer - PALOCK13"]
pub type PALOCK13_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK14` reader - PALOCK14"]
pub type PALOCK14_R = crate::BitReader;
#[doc = "Field `PALOCK14` writer - PALOCK14"]
pub type PALOCK14_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALOCK15` reader - PALOCK15"]
pub type PALOCK15_R = crate::BitReader;
#[doc = "Field `PALOCK15` writer - PALOCK15"]
pub type PALOCK15_W<'a, const O: u8> = crate::BitWriter<'a, PALOCKR_SPEC, O>;
#[doc = "Field `PALKEY` reader - PALKEY"]
pub type PALKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PALKEY` writer - PALKEY"]
pub type PALKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PALOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - PALOCK0"]
    #[inline(always)]
    pub fn palock0(&self) -> PALOCK0_R {
        PALOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PALOCK1"]
    #[inline(always)]
    pub fn palock1(&self) -> PALOCK1_R {
        PALOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PALOCK2"]
    #[inline(always)]
    pub fn palock2(&self) -> PALOCK2_R {
        PALOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PALOCK3"]
    #[inline(always)]
    pub fn palock3(&self) -> PALOCK3_R {
        PALOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PALOCK4"]
    #[inline(always)]
    pub fn palock4(&self) -> PALOCK4_R {
        PALOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PALOCK5"]
    #[inline(always)]
    pub fn palock5(&self) -> PALOCK5_R {
        PALOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PALOCK6"]
    #[inline(always)]
    pub fn palock6(&self) -> PALOCK6_R {
        PALOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PALOCK7"]
    #[inline(always)]
    pub fn palock7(&self) -> PALOCK7_R {
        PALOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PALOCK8"]
    #[inline(always)]
    pub fn palock8(&self) -> PALOCK8_R {
        PALOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PALOCK9"]
    #[inline(always)]
    pub fn palock9(&self) -> PALOCK9_R {
        PALOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PALOCK10"]
    #[inline(always)]
    pub fn palock10(&self) -> PALOCK10_R {
        PALOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PALOCK11"]
    #[inline(always)]
    pub fn palock11(&self) -> PALOCK11_R {
        PALOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PALOCK12"]
    #[inline(always)]
    pub fn palock12(&self) -> PALOCK12_R {
        PALOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PALOCK13"]
    #[inline(always)]
    pub fn palock13(&self) -> PALOCK13_R {
        PALOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PALOCK14"]
    #[inline(always)]
    pub fn palock14(&self) -> PALOCK14_R {
        PALOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PALOCK15"]
    #[inline(always)]
    pub fn palock15(&self) -> PALOCK15_R {
        PALOCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PALKEY"]
    #[inline(always)]
    pub fn palkey(&self) -> PALKEY_R {
        PALKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PALOCK0"]
    #[inline(always)]
    #[must_use]
    pub fn palock0(&mut self) -> PALOCK0_W<0> {
        PALOCK0_W::new(self)
    }
    #[doc = "Bit 1 - PALOCK1"]
    #[inline(always)]
    #[must_use]
    pub fn palock1(&mut self) -> PALOCK1_W<1> {
        PALOCK1_W::new(self)
    }
    #[doc = "Bit 2 - PALOCK2"]
    #[inline(always)]
    #[must_use]
    pub fn palock2(&mut self) -> PALOCK2_W<2> {
        PALOCK2_W::new(self)
    }
    #[doc = "Bit 3 - PALOCK3"]
    #[inline(always)]
    #[must_use]
    pub fn palock3(&mut self) -> PALOCK3_W<3> {
        PALOCK3_W::new(self)
    }
    #[doc = "Bit 4 - PALOCK4"]
    #[inline(always)]
    #[must_use]
    pub fn palock4(&mut self) -> PALOCK4_W<4> {
        PALOCK4_W::new(self)
    }
    #[doc = "Bit 5 - PALOCK5"]
    #[inline(always)]
    #[must_use]
    pub fn palock5(&mut self) -> PALOCK5_W<5> {
        PALOCK5_W::new(self)
    }
    #[doc = "Bit 6 - PALOCK6"]
    #[inline(always)]
    #[must_use]
    pub fn palock6(&mut self) -> PALOCK6_W<6> {
        PALOCK6_W::new(self)
    }
    #[doc = "Bit 7 - PALOCK7"]
    #[inline(always)]
    #[must_use]
    pub fn palock7(&mut self) -> PALOCK7_W<7> {
        PALOCK7_W::new(self)
    }
    #[doc = "Bit 8 - PALOCK8"]
    #[inline(always)]
    #[must_use]
    pub fn palock8(&mut self) -> PALOCK8_W<8> {
        PALOCK8_W::new(self)
    }
    #[doc = "Bit 9 - PALOCK9"]
    #[inline(always)]
    #[must_use]
    pub fn palock9(&mut self) -> PALOCK9_W<9> {
        PALOCK9_W::new(self)
    }
    #[doc = "Bit 10 - PALOCK10"]
    #[inline(always)]
    #[must_use]
    pub fn palock10(&mut self) -> PALOCK10_W<10> {
        PALOCK10_W::new(self)
    }
    #[doc = "Bit 11 - PALOCK11"]
    #[inline(always)]
    #[must_use]
    pub fn palock11(&mut self) -> PALOCK11_W<11> {
        PALOCK11_W::new(self)
    }
    #[doc = "Bit 12 - PALOCK12"]
    #[inline(always)]
    #[must_use]
    pub fn palock12(&mut self) -> PALOCK12_W<12> {
        PALOCK12_W::new(self)
    }
    #[doc = "Bit 13 - PALOCK13"]
    #[inline(always)]
    #[must_use]
    pub fn palock13(&mut self) -> PALOCK13_W<13> {
        PALOCK13_W::new(self)
    }
    #[doc = "Bit 14 - PALOCK14"]
    #[inline(always)]
    #[must_use]
    pub fn palock14(&mut self) -> PALOCK14_W<14> {
        PALOCK14_W::new(self)
    }
    #[doc = "Bit 15 - PALOCK15"]
    #[inline(always)]
    #[must_use]
    pub fn palock15(&mut self) -> PALOCK15_W<15> {
        PALOCK15_W::new(self)
    }
    #[doc = "Bits 16:31 - PALKEY"]
    #[inline(always)]
    #[must_use]
    pub fn palkey(&mut self) -> PALKEY_W<16> {
        PALKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PALOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [palockr](index.html) module"]
pub struct PALOCKR_SPEC;
impl crate::RegisterSpec for PALOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [palockr::R](R) reader structure"]
impl crate::Readable for PALOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [palockr::W](W) writer structure"]
impl crate::Writable for PALOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PALOCKR to value 0"]
impl crate::Resettable for PALOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
