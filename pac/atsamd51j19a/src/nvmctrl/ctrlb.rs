#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u16, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_AW {
    #[doc = "0: Erase Page - Only supported in the USER and AUX pages."]
    EP,
    #[doc = "1: Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    EB,
    #[doc = "3: Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    WP,
    #[doc = "4: Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    WQW,
    #[doc = "16: Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    SWRST,
    #[doc = "17: Lock Region - Locks the region containing the address location in the ADDR register."]
    LR,
    #[doc = "18: Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    UR,
    #[doc = "19: Sets the power reduction mode."]
    SPRM,
    #[doc = "20: Clears the power reduction mode."]
    CPRM,
    #[doc = "21: Page Buffer Clear - Clears the page buffer."]
    PBC,
    #[doc = "22: Set Security Bit"]
    SSB,
    #[doc = "23: Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    BKSWRST,
    #[doc = "24: Chip Erase Lock - DSU.CE command is not available"]
    CELCK,
    #[doc = "25: Chip Erase Unlock - DSU.CE command is available"]
    CEULCK,
    #[doc = "26: Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    SBPDIS,
    #[doc = "27: Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    CBPDIS,
    #[doc = "48: Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    ASEES0,
    #[doc = "49: Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    ASEES1,
    #[doc = "50: Starts SmartEEPROM sector reallocation algorithm"]
    SEERALOC,
    #[doc = "51: Flush SMEE data when in buffered mode"]
    SEEFLUSH,
    #[doc = "52: Lock access to SmartEEPROM data from any mean"]
    LSEE,
    #[doc = "53: Unlock access to SmartEEPROM data"]
    USEE,
    #[doc = "54: Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    LSEER,
    #[doc = "55: Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    USEER,
}
impl From<CMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_AW) -> Self {
        match variant {
            CMD_AW::EP => 0,
            CMD_AW::EB => 1,
            CMD_AW::WP => 3,
            CMD_AW::WQW => 4,
            CMD_AW::SWRST => 16,
            CMD_AW::LR => 17,
            CMD_AW::UR => 18,
            CMD_AW::SPRM => 19,
            CMD_AW::CPRM => 20,
            CMD_AW::PBC => 21,
            CMD_AW::SSB => 22,
            CMD_AW::BKSWRST => 23,
            CMD_AW::CELCK => 24,
            CMD_AW::CEULCK => 25,
            CMD_AW::SBPDIS => 26,
            CMD_AW::CBPDIS => 27,
            CMD_AW::ASEES0 => 48,
            CMD_AW::ASEES1 => 49,
            CMD_AW::SEERALOC => 50,
            CMD_AW::SEEFLUSH => 51,
            CMD_AW::LSEE => 52,
            CMD_AW::USEE => 53,
            CMD_AW::LSEER => 54,
            CMD_AW::USEER => 55,
        }
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Erase Page - Only supported in the USER and AUX pages."]
    #[inline(always)]
    pub fn ep(self) -> &'a mut W {
        self.variant(CMD_AW::EP)
    }
    #[doc = "Erase Block - Erases the block addressed by the ADDR register, not supported in the user page"]
    #[inline(always)]
    pub fn eb(self) -> &'a mut W {
        self.variant(CMD_AW::EB)
    }
    #[doc = "Write Page - Writes the contents of the page buffer to the page addressed by the ADDR register, not supported in the user page"]
    #[inline(always)]
    pub fn wp(self) -> &'a mut W {
        self.variant(CMD_AW::WP)
    }
    #[doc = "Write Quad Word - Writes a 128-bit word at the location addressed by the ADDR register."]
    #[inline(always)]
    pub fn wqw(self) -> &'a mut W {
        self.variant(CMD_AW::WQW)
    }
    #[doc = "Software Reset - Power-Cycle the NVM memory and replay the device automatic calibration procedure and resets the module configuration registers"]
    #[inline(always)]
    pub fn swrst(self) -> &'a mut W {
        self.variant(CMD_AW::SWRST)
    }
    #[doc = "Lock Region - Locks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn lr(self) -> &'a mut W {
        self.variant(CMD_AW::LR)
    }
    #[doc = "Unlock Region - Unlocks the region containing the address location in the ADDR register."]
    #[inline(always)]
    pub fn ur(self) -> &'a mut W {
        self.variant(CMD_AW::UR)
    }
    #[doc = "Sets the power reduction mode."]
    #[inline(always)]
    pub fn sprm(self) -> &'a mut W {
        self.variant(CMD_AW::SPRM)
    }
    #[doc = "Clears the power reduction mode."]
    #[inline(always)]
    pub fn cprm(self) -> &'a mut W {
        self.variant(CMD_AW::CPRM)
    }
    #[doc = "Page Buffer Clear - Clears the page buffer."]
    #[inline(always)]
    pub fn pbc(self) -> &'a mut W {
        self.variant(CMD_AW::PBC)
    }
    #[doc = "Set Security Bit"]
    #[inline(always)]
    pub fn ssb(self) -> &'a mut W {
        self.variant(CMD_AW::SSB)
    }
    #[doc = "Bank swap and system reset, if SMEE is used also reallocate SMEE data into the opposite BANK"]
    #[inline(always)]
    pub fn bkswrst(self) -> &'a mut W {
        self.variant(CMD_AW::BKSWRST)
    }
    #[doc = "Chip Erase Lock - DSU.CE command is not available"]
    #[inline(always)]
    pub fn celck(self) -> &'a mut W {
        self.variant(CMD_AW::CELCK)
    }
    #[doc = "Chip Erase Unlock - DSU.CE command is available"]
    #[inline(always)]
    pub fn ceulck(self) -> &'a mut W {
        self.variant(CMD_AW::CEULCK)
    }
    #[doc = "Sets STATUS.BPDIS, Boot loader protection is discarded until CBPDIS is issued or next start-up sequence"]
    #[inline(always)]
    pub fn sbpdis(self) -> &'a mut W {
        self.variant(CMD_AW::SBPDIS)
    }
    #[doc = "Clears STATUS.BPDIS, Boot loader protection is not discarded"]
    #[inline(always)]
    pub fn cbpdis(self) -> &'a mut W {
        self.variant(CMD_AW::CBPDIS)
    }
    #[doc = "Activate SmartEEPROM Sector 0, deactivate Sector 1"]
    #[inline(always)]
    pub fn asees0(self) -> &'a mut W {
        self.variant(CMD_AW::ASEES0)
    }
    #[doc = "Activate SmartEEPROM Sector 1, deactivate Sector 0"]
    #[inline(always)]
    pub fn asees1(self) -> &'a mut W {
        self.variant(CMD_AW::ASEES1)
    }
    #[doc = "Starts SmartEEPROM sector reallocation algorithm"]
    #[inline(always)]
    pub fn seeraloc(self) -> &'a mut W {
        self.variant(CMD_AW::SEERALOC)
    }
    #[doc = "Flush SMEE data when in buffered mode"]
    #[inline(always)]
    pub fn seeflush(self) -> &'a mut W {
        self.variant(CMD_AW::SEEFLUSH)
    }
    #[doc = "Lock access to SmartEEPROM data from any mean"]
    #[inline(always)]
    pub fn lsee(self) -> &'a mut W {
        self.variant(CMD_AW::LSEE)
    }
    #[doc = "Unlock access to SmartEEPROM data"]
    #[inline(always)]
    pub fn usee(self) -> &'a mut W {
        self.variant(CMD_AW::USEE)
    }
    #[doc = "Lock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline(always)]
    pub fn lseer(self) -> &'a mut W {
        self.variant(CMD_AW::LSEER)
    }
    #[doc = "Unlock access to the SmartEEPROM Register Address Space (above 64KB)"]
    #[inline(always)]
    pub fn useer(self) -> &'a mut W {
        self.variant(CMD_AW::USEER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u16) & 0x7f);
        self.w
    }
}
#[doc = "Command Execution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDEX_AW {
    #[doc = "165: Execution Key"]
    KEY,
}
impl From<CMDEX_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMDEX_AW) -> Self {
        match variant {
            CMDEX_AW::KEY => 165,
        }
    }
}
#[doc = "Write proxy for field `CMDEX`"]
pub struct CMDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDEX_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Execution Key"]
    #[inline(always)]
    pub fn key(self) -> &'a mut W {
        self.variant(CMDEX_AW::KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u16) & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:6 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bits 8:15 - Command Execution"]
    #[inline(always)]
    pub fn cmdex(&mut self) -> CMDEX_W {
        CMDEX_W { w: self }
    }
}
