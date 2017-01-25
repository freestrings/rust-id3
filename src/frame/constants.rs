// 2.2 mapping
// BUF
// CNT - PCNT
// COM - COMM
// CRA - AENC
// CRM -
// ETC - ETCO
// EQU - EQUA
// GEO - GEOB
// IPL - IPLS
// LNK - LINK
// MCI - MCDI
// MLL - MLLT
// PIC
// POP - POPM
// REV - RVRB
// RVA - RVAD
// SLT - SYLT
// STC - SYTC
// TAL - TALB
// TBP - TBPM
// TCM - TCOM
// TCO - TCON
// TCR - TCOP
// TDA - TDAT
// TDY - TDLY
// TEN - TENC
// TFT - TFLT
// TIM - TIME
// TKE - TKEY
// TLA - TLAN
// TLE - TLEN
// TMT - TMED
// TOA - TOPE
// TOF - TOFN
// TOL - TOLY
// TOR - TORY
// TOT - TOAL
// TP1 - TPE1
// TP2 - TPE2
// TP3 - TPE3
// TP4 - TPE4
// TPA - TPOS
// TPB - TPUB
// TRC - TSRC
// TRD - TRDA
// TRK - TRCK
// TSI - TSIZ
// TSS - TSSE
// TT1 - TIT1
// TT2 - TIT2
// TT3 - TIT1
// TXT - TEXT
// TXX - TXXX
// TYE - TYER
// UFI - UFID
// ULT - USLT
// WAF - WOAF
// WAR - WOAR
// WAS - WOAS
// WCM - WCOM
// WCP - WCOP
// WPB - WPUB
// WXX - WXXX
pub mod id {
    //
    // 2.2
    //
    pub const BUF_STR: &'static str = "BUF";
    pub const CNT_STR: &'static str = "CNT";
    pub const COM_STR: &'static str = "COM";
    pub const CRA_STR: &'static str = "CRA";
    pub const CRM_STR: &'static str = "CRM";
    pub const ETC_STR: &'static str = "ETC";
    pub const EQU_STR: &'static str = "EQU";
    pub const GEO_STR: &'static str = "GEO";
    pub const IPL_STR: &'static str = "IPL";
    pub const LNK_STR: &'static str = "LNK";
    pub const MCI_STR: &'static str = "MCI";
    pub const MLL_STR: &'static str = "MLL";
    pub const PIC_STR: &'static str = "PIC";
    pub const POP_STR: &'static str = "POP";
    pub const REV_STR: &'static str = "REV";
    pub const RVA_STR: &'static str = "RVA";
    pub const SLT_STR: &'static str = "SLT";
    pub const STC_STR: &'static str = "STC";
    pub const TAL_STR: &'static str = "TAL";
    pub const TBP_STR: &'static str = "TBP";
    pub const TCM_STR: &'static str = "TCM";
    pub const TCO_STR: &'static str = "TCO";
    pub const TCR_STR: &'static str = "TCR";
    pub const TDA_STR: &'static str = "TDA";
    pub const TDY_STR: &'static str = "TDY";
    pub const TEN_STR: &'static str = "TEN";
    pub const TFT_STR: &'static str = "TFT";
    pub const TIM_STR: &'static str = "TIM";
    pub const TKE_STR: &'static str = "TKE";
    pub const TLA_STR: &'static str = "TLA";
    pub const TLE_STR: &'static str = "TLE";
    pub const TMT_STR: &'static str = "TMT";
    pub const TOA_STR: &'static str = "TOA";
    pub const TOF_STR: &'static str = "TOF";
    pub const TOL_STR: &'static str = "TOL";
    pub const TOR_STR: &'static str = "TOR";
    pub const TOT_STR: &'static str = "TOT";
    pub const TP1_STR: &'static str = "TP1";
    pub const TP2_STR: &'static str = "TP2";
    pub const TP3_STR: &'static str = "TP3";
    pub const TP4_STR: &'static str = "TP4";
    pub const TPA_STR: &'static str = "TPA";
    pub const TPB_STR: &'static str = "TPB";
    pub const TRC_STR: &'static str = "TRC";
    pub const TRD_STR: &'static str = "TRD";
    pub const TRK_STR: &'static str = "TRK";
    pub const TSI_STR: &'static str = "TSI";
    pub const TSS_STR: &'static str = "TSS";
    pub const TT1_STR: &'static str = "TT1";
    pub const TT2_STR: &'static str = "TT2";
    pub const TT3_STR: &'static str = "TT3";
    pub const TXT_STR: &'static str = "TXT";
    pub const TXX_STR: &'static str = "TXX";
    pub const TYE_STR: &'static str = "TYE";
    pub const UFI_STR: &'static str = "UFI";
    pub const ULT_STR: &'static str = "ULT";
    pub const WAF_STR: &'static str = "WAF";
    pub const WAR_STR: &'static str = "WAR";
    pub const WAS_STR: &'static str = "WAS";
    pub const WCM_STR: &'static str = "WCM";
    pub const WCP_STR: &'static str = "WCP";
    pub const WPB_STR: &'static str = "WPB";
    pub const WXX_STR: &'static str = "WXX";

    //
    // 2.3 & 2.4
    //
    pub const AENC_STR: &'static str = "AENC";
    pub const APIC_STR: &'static str = "APIC";
    pub const ASPI_STR: &'static str = "ASPI";
    pub const COMM_STR: &'static str = "COMM";
    pub const COMR_STR: &'static str = "COMR";
    pub const ENCR_STR: &'static str = "ENCR";
    pub const EQU2_STR: &'static str = "EQU2";
    // 2.3 only
    pub const EQUA_STR: &'static str = "EQUA";
    pub const ETCO_STR: &'static str = "ETCO";
    pub const GEOB_STR: &'static str = "GEOB";
    pub const GRID_STR: &'static str = "GRID";
    // 2.3 only
    pub const IPLS_STR: &'static str = "IPLS";
    pub const LINK_STR: &'static str = "LINK";
    pub const MCDI_STR: &'static str = "MCDI";
    pub const MLLT_STR: &'static str = "MLLT";
    pub const OWNE_STR: &'static str = "OWNE";
    pub const PRIV_STR: &'static str = "PRIV";
    pub const PCNT_STR: &'static str = "PCNT";
    pub const POPM_STR: &'static str = "POPM";
    pub const POSS_STR: &'static str = "POSS";
    pub const RBUF_STR: &'static str = "RBUF";
    // 2.3 only
    pub const RVAD_STR: &'static str = "RVAD";
    pub const RVA2_STR: &'static str = "RVA2";
    pub const RVRB_STR: &'static str = "RVRB";
    pub const SEEK_STR: &'static str = "SEEK";
    pub const SIGN_STR: &'static str = "SIGN";
    pub const SYLT_STR: &'static str = "SYLT";
    pub const SYTC_STR: &'static str = "SYTC";
    pub const TALB_STR: &'static str = "TALB";
    pub const TBPM_STR: &'static str = "TBPM";
    pub const TCOM_STR: &'static str = "TCOM";
    pub const TCON_STR: &'static str = "TCON";
    pub const TCOP_STR: &'static str = "TCOP";
    // 2.3 only
    pub const TDAT_STR: &'static str = "TDAT";
    pub const TDEN_STR: &'static str = "TDEN";
    pub const TDLY_STR: &'static str = "TDLY";
    pub const TDOR_STR: &'static str = "TDOR";
    pub const TDRC_STR: &'static str = "TDRC";
    pub const TDTG_STR: &'static str = "TDTG";
    pub const TDRL_STR: &'static str = "TDRL";
    pub const TENC_STR: &'static str = "TENC";
    pub const TEXT_STR: &'static str = "TEXT";
    pub const TFLT_STR: &'static str = "TFLT";
    // 2.3 only
    pub const TIME_STR: &'static str = "TIME";
    pub const TIPL_STR: &'static str = "TIPL";
    pub const TIT1_STR: &'static str = "TIT1";
    pub const TIT2_STR: &'static str = "TIT2";
    pub const TIT3_STR: &'static str = "TIT3";
    pub const TKEY_STR: &'static str = "TKEY";
    pub const TLAN_STR: &'static str = "TLAN";
    pub const TLEN_STR: &'static str = "TLEN";
    pub const TMCL_STR: &'static str = "TMCL";
    pub const TMED_STR: &'static str = "TMED";
    pub const TMOO_STR: &'static str = "TMOO";
    pub const TOAL_STR: &'static str = "TOAL";
    pub const TOFN_STR: &'static str = "TOFN";
    pub const TOLY_STR: &'static str = "TOLY";
    pub const TOPE_STR: &'static str = "TOPE";
    pub const TORY_STR: &'static str = "TORY";
    pub const TOWN_STR: &'static str = "TOWN";
    pub const TPE1_STR: &'static str = "TPE1";
    pub const TPE2_STR: &'static str = "TPE2";
    pub const TPE3_STR: &'static str = "TPE3";
    pub const TPE4_STR: &'static str = "TPE4";
    pub const TPOS_STR: &'static str = "TPOS";
    pub const TPRO_STR: &'static str = "TPRO";
    pub const TPUB_STR: &'static str = "TPUB";
    pub const TRCK_STR: &'static str = "TRCK";
    pub const TRDA_STR: &'static str = "TRDA";
    pub const TRSN_STR: &'static str = "TRSN";
    pub const TRSO_STR: &'static str = "TRSO";
    // 2.3 only
    pub const TSIZ_STR: &'static str = "TSIZ";
    pub const TSOA_STR: &'static str = "TSOA";
    pub const TSOP_STR: &'static str = "TSOP";
    pub const TSOT_STR: &'static str = "TSOT";
    pub const TSRC_STR: &'static str = "TSRC";
    pub const TSSE_STR: &'static str = "TSSE";
    // 2.3 only
    pub const TYER_STR: &'static str = "TYER";
    pub const TSST_STR: &'static str = "TSST";
    pub const TXXX_STR: &'static str = "TXXX";
    pub const UFID_STR: &'static str = "UFID";
    pub const USER_STR: &'static str = "USER";
    pub const USLT_STR: &'static str = "USLT";
    pub const WCOM_STR: &'static str = "WCOM";
    pub const WCOP_STR: &'static str = "WCOP";
    pub const WOAF_STR: &'static str = "WOAF";
    pub const WOAR_STR: &'static str = "WOAR";
    pub const WOAS_STR: &'static str = "WOAS";
    pub const WORS_STR: &'static str = "WORS";
    pub const WPAY_STR: &'static str = "WPAY";
    pub const WPUB_STR: &'static str = "WPUB";
    pub const WXXX_STR: &'static str = "WXXX";
}

#[derive(Debug, PartialEq)]
pub enum TextEncoding {
    Iso8859_1,
    UTF16LE,
    UTF16BE,
    UTF8
}

use ::frame::*;

#[derive(Debug, PartialEq)]
pub enum FrameData {
    //2.2 only
    BUF(BUF),
    //2.2 only
    CRM(CRM),
    //2.2 only
    PIC(PIC),

    AENC(AENC),
    APIC(APIC),
    ASPI(ASPI),
    COMM(COMM),
    COMR(COMR),
    ENCR(ENCR),
    // 2.3 only
    EQUA(EQUA),
    EQU2(EQU2),
    ETCO(ETCO),
    GEOB(GEOB),
    GRID(GRID),
    // 2.3 only
    IPLS(IPLS),
    LINK(LINK),
    MCDI(MCDI),
    MLLT(MLLT),
    OWNE(OWNE),
    PRIV(PRIV),
    PCNT(PCNT),
    POPM(POPM),
    POSS(POSS),
    RBUF(RBUF),
    // 2.3 only
    RVAD(RVA2),
    RVA2(RVA2),
    RVRB(RVRB),
    SEEK(SEEK),
    SIGN(SIGN),
    SYLT(SYLT),
    SYTC(SYTC),
    TALB(TEXT),
    TBPM(TEXT),
    TCOM(TEXT),
    TCON(TEXT),
    TCOP(TEXT),
    // 2.3 only
    TDAT(TEXT),
    TDEN(TEXT),
    TDLY(TEXT),
    TDOR(TEXT),
    TDRC(TEXT),
    TDRL(TEXT),
    TDTG(TEXT),
    TENC(TEXT),
    TEXT(TEXT),
    TFLT(TEXT),
    // 2.3 only
    TIME(TEXT),
    TIPL(TEXT),
    TIT1(TEXT),
    TIT2(TEXT),
    TIT3(TEXT),
    TKEY(TEXT),
    TLAN(TEXT),
    TLEN(TEXT),
    TMCL(TEXT),
    TMED(TEXT),
    TMOO(TEXT),
    TOAL(TEXT),
    TOFN(TEXT),
    TOLY(TEXT),
    TOPE(TEXT),
    TORY(TEXT),
    TOWN(TEXT),
    TPE1(TEXT),
    TPE2(TEXT),
    TPE3(TEXT),
    TPE4(TEXT),
    TPOS(TEXT),
    TPRO(TEXT),
    TPUB(TEXT),
    TRCK(TEXT),
    // 2.3 only
    TRDA(TEXT),
    TRSN(TEXT),
    TRSO(TEXT),
    // 2.3 only
    TSIZ(TEXT),
    TSOA(TEXT),
    TSOP(TEXT),
    TSOT(TEXT),
    TSRC(TEXT),
    TSSE(TEXT),
    // 2.3 only
    TYER(TEXT),
    TSST(TEXT),
    TXXX(TXXX),
    UFID(UFID),
    USER(USER),
    USLT(USLT),
    WCOM(LINK),
    WCOP(LINK),
    WOAF(LINK),
    WOAR(LINK),
    WOAS(LINK),
    WORS(LINK),
    WPAY(LINK),
    WPUB(LINK),
    WXXX(WXXX),
    SKIP(String),
    INVALID(String),
}

#[derive(Debug, PartialEq)]
pub enum PictureType {
    Other,
    FileIcon,
    OtherFileIcon,
    CoverFront,
    CoverBack,
    LeafletPage,
    Media,
    LeadArtist,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    MovieScreenCapture,
    BrightColouredFish,
    Illustration,
    BandLogotype,
    PublisherLogoType
}

#[derive(Debug, PartialEq)]
pub enum ReceivedAs {
    Other,
    StandardCDAlbum,
    CompressedAudioOnCD,
    FileOverInternet,
    StreamOverInternet,
    AsNoteSheets,
    AsNoteSheetsInBook,
    MusicOnMedia,
    NonMusicalMerchandise
}

#[derive(Debug, PartialEq)]
pub enum InterpolationMethod {
    Band,
    Linear
}

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Other,
    Lyrics,
    TextTranscription,
    MovementName,
    Events,
    Chord,
    Trivia,
    UrlsToWebpages,
    UrlsToImages
}

#[derive(Debug, PartialEq)]
pub enum TimestampFormat {
    MpecFrames,
    Milliseconds
}

#[derive(Debug, PartialEq)]
pub enum EventTimingCode {
    Padding(u32),
    EndOfInitialSilence(u32),
    IntroStart(u32),
    MainPartStart(u32),
    OutroStart(u32),
    OutroEnd(u32),
    VerseStart(u32),
    RefrainStart(u32),
    InterludeStart(u32),
    ThemeStart(u32),
    VariationStart(u32),
    KeyChange(u32),
    TimeChange(u32),
    MomentaryUnwantedNoise(u32),
    SustainedNoise(u32),
    SustainedNoiseEnd(u32),
    IntroEnd(u32),
    MainPartEnd(u32),
    VerseEnd(u32),
    RefrainEnd(u32),
    ThemeEnd(u32),
    Profanity(u32),
    ProfanityEnd(u32),
    ReservedForFutureUse(u32),
    NotPredefinedSynch(u32),
    AudioEnd(u32),
    AudioFileEnds(u32),
    OneMoreByteOfEventsFollows(u32)
}

#[derive(Debug, PartialEq)]
pub enum FrameHeaderFlag {
    TagAlter,
    FileAlter,
    ReadOnly,
    Compression,
    Encryption,
    GroupIdentity,
    //2.4 only
    Unsynchronisation,
    //2.4 only
    DataLength
}

#[derive(Debug, PartialEq)]
pub enum HeadFlag {
    Unsynchronisation,
    Compression,
    ExtendedHeader,
    ExperimentalIndicator,
    FooterPresent
}