//! Instrument (symbol) names known to the OANDA v20 API.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

macro_rules! instrument_names {
    ($($(#[$meta:meta])* $variant:ident => $symbol:literal,)+) => {
        /// The name of an instrument (symbol) in `BASE_QUOTE` format.
        ///
        /// All symbols available on OANDA's fxPractice environment are
        /// represented as dedicated variants; anything else (new listings,
        /// division-specific CFDs) round-trips through
        /// [`InstrumentName::Other`], so parsing and deserialization never
        /// fail. Known strings always normalize to their variant:
        /// `InstrumentName::from("EUR_USD") == InstrumentName::EurUsd`.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[non_exhaustive]
        pub enum InstrumentName {
            $($(#[$meta])* $variant,)+
            /// An instrument not (yet) in the known symbol table.
            Other(String),
        }

        impl InstrumentName {
            /// The canonical wire representation, e.g. `EUR_USD`.
            pub fn as_str(&self) -> &str {
                match self {
                    $(InstrumentName::$variant => $symbol,)+
                    InstrumentName::Other(s) => s,
                }
            }

            /// Every known instrument, in alphabetical order.
            pub const KNOWN: &'static [InstrumentName] = &[
                $(InstrumentName::$variant,)+
            ];

            /// Whether this is a known symbol (i.e. not [`InstrumentName::Other`]).
            pub fn is_known(&self) -> bool {
                !matches!(self, InstrumentName::Other(_))
            }
        }

        impl FromStr for InstrumentName {
            type Err = std::convert::Infallible;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $($symbol => InstrumentName::$variant,)+
                    other => InstrumentName::Other(other.to_owned()),
                })
            }
        }
    };
}

instrument_names! {
    /// Australia 200 (`AU200_AUD`, cfd)
    Au200Aud => "AU200_AUD",
    /// AUD/CAD (`AUD_CAD`, currency)
    AudCad => "AUD_CAD",
    /// AUD/CHF (`AUD_CHF`, currency)
    AudChf => "AUD_CHF",
    /// AUD/HKD (`AUD_HKD`, currency)
    AudHkd => "AUD_HKD",
    /// AUD/JPY (`AUD_JPY`, currency)
    AudJpy => "AUD_JPY",
    /// AUD/NZD (`AUD_NZD`, currency)
    AudNzd => "AUD_NZD",
    /// AUD/SGD (`AUD_SGD`, currency)
    AudSgd => "AUD_SGD",
    /// AUD/USD (`AUD_USD`, currency)
    AudUsd => "AUD_USD",
    /// Brent Crude Oil (`BCO_USD`, cfd)
    BcoUsd => "BCO_USD",
    /// CAD/CHF (`CAD_CHF`, currency)
    CadChf => "CAD_CHF",
    /// CAD/HKD (`CAD_HKD`, currency)
    CadHkd => "CAD_HKD",
    /// CAD/JPY (`CAD_JPY`, currency)
    CadJpy => "CAD_JPY",
    /// CAD/SGD (`CAD_SGD`, currency)
    CadSgd => "CAD_SGD",
    /// Switzerland 20 (`CH20_CHF`, cfd)
    Ch20Chf => "CH20_CHF",
    /// CHF/HKD (`CHF_HKD`, currency)
    ChfHkd => "CHF_HKD",
    /// CHF/JPY (`CHF_JPY`, currency)
    ChfJpy => "CHF_JPY",
    /// CHF/ZAR (`CHF_ZAR`, currency)
    ChfZar => "CHF_ZAR",
    /// China H Shares (`CHINAH_HKD`, cfd)
    ChinahHkd => "CHINAH_HKD",
    /// China A50 (`CN50_USD`, cfd)
    Cn50Usd => "CN50_USD",
    /// Corn (`CORN_USD`, cfd)
    CornUsd => "CORN_USD",
    /// Bund (`DE10YB_EUR`, cfd)
    De10YbEur => "DE10YB_EUR",
    /// Germany 30 (`DE30_EUR`, cfd)
    De30Eur => "DE30_EUR",
    /// Spain 35 (`ESPIX_EUR`, cfd)
    EspixEur => "ESPIX_EUR",
    /// Europe 50 (`EU50_EUR`, cfd)
    Eu50Eur => "EU50_EUR",
    /// EUR/AUD (`EUR_AUD`, currency)
    EurAud => "EUR_AUD",
    /// EUR/CAD (`EUR_CAD`, currency)
    EurCad => "EUR_CAD",
    /// EUR/CHF (`EUR_CHF`, currency)
    EurChf => "EUR_CHF",
    /// EUR/CZK (`EUR_CZK`, currency)
    EurCzk => "EUR_CZK",
    /// EUR/DKK (`EUR_DKK`, currency)
    EurDkk => "EUR_DKK",
    /// EUR/GBP (`EUR_GBP`, currency)
    EurGbp => "EUR_GBP",
    /// EUR/HKD (`EUR_HKD`, currency)
    EurHkd => "EUR_HKD",
    /// EUR/HUF (`EUR_HUF`, currency)
    EurHuf => "EUR_HUF",
    /// EUR/JPY (`EUR_JPY`, currency)
    EurJpy => "EUR_JPY",
    /// EUR/NOK (`EUR_NOK`, currency)
    EurNok => "EUR_NOK",
    /// EUR/NZD (`EUR_NZD`, currency)
    EurNzd => "EUR_NZD",
    /// EUR/PLN (`EUR_PLN`, currency)
    EurPln => "EUR_PLN",
    /// EUR/SEK (`EUR_SEK`, currency)
    EurSek => "EUR_SEK",
    /// EUR/SGD (`EUR_SGD`, currency)
    EurSgd => "EUR_SGD",
    /// EUR/TRY (`EUR_TRY`, currency)
    EurTry => "EUR_TRY",
    /// EUR/USD (`EUR_USD`, currency)
    EurUsd => "EUR_USD",
    /// EUR/ZAR (`EUR_ZAR`, currency)
    EurZar => "EUR_ZAR",
    /// France 40 (`FR40_EUR`, cfd)
    Fr40Eur => "FR40_EUR",
    /// GBP/AUD (`GBP_AUD`, currency)
    GbpAud => "GBP_AUD",
    /// GBP/CAD (`GBP_CAD`, currency)
    GbpCad => "GBP_CAD",
    /// GBP/CHF (`GBP_CHF`, currency)
    GbpChf => "GBP_CHF",
    /// GBP/HKD (`GBP_HKD`, currency)
    GbpHkd => "GBP_HKD",
    /// GBP/JPY (`GBP_JPY`, currency)
    GbpJpy => "GBP_JPY",
    /// GBP/NZD (`GBP_NZD`, currency)
    GbpNzd => "GBP_NZD",
    /// GBP/PLN (`GBP_PLN`, currency)
    GbpPln => "GBP_PLN",
    /// GBP/SGD (`GBP_SGD`, currency)
    GbpSgd => "GBP_SGD",
    /// GBP/USD (`GBP_USD`, currency)
    GbpUsd => "GBP_USD",
    /// GBP/ZAR (`GBP_ZAR`, currency)
    GbpZar => "GBP_ZAR",
    /// Hong Kong 33 (`HK33_HKD`, cfd)
    Hk33Hkd => "HK33_HKD",
    /// HKD/JPY (`HKD_JPY`, currency)
    HkdJpy => "HKD_JPY",
    /// Japan 225 (JPY) (`JP225Y_JPY`, cfd)
    Jp225YJpy => "JP225Y_JPY",
    /// Japan 225 (`JP225_USD`, cfd)
    Jp225Usd => "JP225_USD",
    /// US Nas 100 (`NAS100_USD`, cfd)
    Nas100Usd => "NAS100_USD",
    /// Natural Gas (`NATGAS_USD`, cfd)
    NatgasUsd => "NATGAS_USD",
    /// Netherlands 25 (`NL25_EUR`, cfd)
    Nl25Eur => "NL25_EUR",
    /// NZD/CAD (`NZD_CAD`, currency)
    NzdCad => "NZD_CAD",
    /// NZD/CHF (`NZD_CHF`, currency)
    NzdChf => "NZD_CHF",
    /// NZD/HKD (`NZD_HKD`, currency)
    NzdHkd => "NZD_HKD",
    /// NZD/JPY (`NZD_JPY`, currency)
    NzdJpy => "NZD_JPY",
    /// NZD/SGD (`NZD_SGD`, currency)
    NzdSgd => "NZD_SGD",
    /// NZD/USD (`NZD_USD`, currency)
    NzdUsd => "NZD_USD",
    /// Singapore 30 (`SG30_SGD`, cfd)
    Sg30Sgd => "SG30_SGD",
    /// SGD/CHF (`SGD_CHF`, currency)
    SgdChf => "SGD_CHF",
    /// SGD/JPY (`SGD_JPY`, currency)
    SgdJpy => "SGD_JPY",
    /// Soybeans (`SOYBN_USD`, cfd)
    SoybnUsd => "SOYBN_USD",
    /// US SPX 500 (`SPX500_USD`, cfd)
    Spx500Usd => "SPX500_USD",
    /// Sugar (`SUGAR_USD`, cfd)
    SugarUsd => "SUGAR_USD",
    /// TRY/JPY (`TRY_JPY`, currency)
    TryJpy => "TRY_JPY",
    /// UK 100 (`UK100_GBP`, cfd)
    Uk100Gbp => "UK100_GBP",
    /// UK 10Y Gilt (`UK10YB_GBP`, cfd)
    Uk10YbGbp => "UK10YB_GBP",
    /// US Russ 2000 (`US2000_USD`, cfd)
    Us2000Usd => "US2000_USD",
    /// US Wall St 30 (`US30_USD`, cfd)
    Us30Usd => "US30_USD",
    /// US 2Y T-Note (`USB02Y_USD`, cfd)
    Usb02YUsd => "USB02Y_USD",
    /// US 5Y T-Note (`USB05Y_USD`, cfd)
    Usb05YUsd => "USB05Y_USD",
    /// US 10Y T-Note (`USB10Y_USD`, cfd)
    Usb10YUsd => "USB10Y_USD",
    /// US T-Bond (`USB30Y_USD`, cfd)
    Usb30YUsd => "USB30Y_USD",
    /// USD/CAD (`USD_CAD`, currency)
    UsdCad => "USD_CAD",
    /// USD/CHF (`USD_CHF`, currency)
    UsdChf => "USD_CHF",
    /// USD/CNH (`USD_CNH`, currency)
    UsdCnh => "USD_CNH",
    /// USD/CZK (`USD_CZK`, currency)
    UsdCzk => "USD_CZK",
    /// USD/DKK (`USD_DKK`, currency)
    UsdDkk => "USD_DKK",
    /// USD/HKD (`USD_HKD`, currency)
    UsdHkd => "USD_HKD",
    /// USD/HUF (`USD_HUF`, currency)
    UsdHuf => "USD_HUF",
    /// USD/JPY (`USD_JPY`, currency)
    UsdJpy => "USD_JPY",
    /// USD/MXN (`USD_MXN`, currency)
    UsdMxn => "USD_MXN",
    /// USD/NOK (`USD_NOK`, currency)
    UsdNok => "USD_NOK",
    /// USD/PLN (`USD_PLN`, currency)
    UsdPln => "USD_PLN",
    /// USD/SEK (`USD_SEK`, currency)
    UsdSek => "USD_SEK",
    /// USD/SGD (`USD_SGD`, currency)
    UsdSgd => "USD_SGD",
    /// USD/THB (`USD_THB`, currency)
    UsdThb => "USD_THB",
    /// USD/TRY (`USD_TRY`, currency)
    UsdTry => "USD_TRY",
    /// USD/ZAR (`USD_ZAR`, currency)
    UsdZar => "USD_ZAR",
    /// Wheat (`WHEAT_USD`, cfd)
    WheatUsd => "WHEAT_USD",
    /// West Texas Oil (`WTICO_USD`, cfd)
    WticoUsd => "WTICO_USD",
    /// Silver/AUD (`XAG_AUD`, metal)
    XagAud => "XAG_AUD",
    /// Silver/CAD (`XAG_CAD`, metal)
    XagCad => "XAG_CAD",
    /// Silver/CHF (`XAG_CHF`, metal)
    XagChf => "XAG_CHF",
    /// Silver/EUR (`XAG_EUR`, metal)
    XagEur => "XAG_EUR",
    /// Silver/GBP (`XAG_GBP`, metal)
    XagGbp => "XAG_GBP",
    /// Silver/HKD (`XAG_HKD`, metal)
    XagHkd => "XAG_HKD",
    /// Silver/JPY (`XAG_JPY`, metal)
    XagJpy => "XAG_JPY",
    /// Silver/NZD (`XAG_NZD`, metal)
    XagNzd => "XAG_NZD",
    /// Silver/SGD (`XAG_SGD`, metal)
    XagSgd => "XAG_SGD",
    /// Silver (`XAG_USD`, metal)
    XagUsd => "XAG_USD",
    /// Gold/AUD (`XAU_AUD`, metal)
    XauAud => "XAU_AUD",
    /// Gold/CAD (`XAU_CAD`, metal)
    XauCad => "XAU_CAD",
    /// Gold/CHF (`XAU_CHF`, metal)
    XauChf => "XAU_CHF",
    /// Gold/EUR (`XAU_EUR`, metal)
    XauEur => "XAU_EUR",
    /// Gold/GBP (`XAU_GBP`, metal)
    XauGbp => "XAU_GBP",
    /// Gold/HKD (`XAU_HKD`, metal)
    XauHkd => "XAU_HKD",
    /// Gold/JPY (`XAU_JPY`, metal)
    XauJpy => "XAU_JPY",
    /// Gold/NZD (`XAU_NZD`, metal)
    XauNzd => "XAU_NZD",
    /// Gold/SGD (`XAU_SGD`, metal)
    XauSgd => "XAU_SGD",
    /// Gold (`XAU_USD`, metal)
    XauUsd => "XAU_USD",
    /// Gold/Silver (`XAU_XAG`, metal)
    XauXag => "XAU_XAG",
    /// Copper (`XCU_USD`, cfd)
    XcuUsd => "XCU_USD",
    /// Palladium (`XPD_USD`, cfd)
    XpdUsd => "XPD_USD",
    /// Platinum (`XPT_USD`, cfd)
    XptUsd => "XPT_USD",
    /// ZAR/JPY (`ZAR_JPY`, currency)
    ZarJpy => "ZAR_JPY",
}

impl fmt::Display for InstrumentName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<&str> for InstrumentName {
    fn from(value: &str) -> Self {
        value.parse().expect("infallible")
    }
}

impl From<String> for InstrumentName {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl Serialize for InstrumentName {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for InstrumentName {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(s.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_symbols_normalize_to_variants() {
        assert_eq!(InstrumentName::from("EUR_USD"), InstrumentName::EurUsd);
        assert_eq!(InstrumentName::from("XAU_USD"), InstrumentName::XauUsd);
        assert_eq!(
            InstrumentName::from("SPX500_USD"),
            InstrumentName::Spx500Usd
        );
    }

    #[test]
    fn unknown_symbols_fall_back_to_other() {
        let i = InstrumentName::from("FOO_BAR");
        assert_eq!(i, InstrumentName::Other("FOO_BAR".to_owned()));
        assert!(!i.is_known());
        assert_eq!(i.as_str(), "FOO_BAR");
    }

    #[test]
    fn display_matches_wire_format() {
        assert_eq!(InstrumentName::EurUsd.to_string(), "EUR_USD");
        assert_eq!(InstrumentName::Usb02YUsd.to_string(), "USB02Y_USD");
        assert_eq!(InstrumentName::Jp225YJpy.to_string(), "JP225Y_JPY");
    }

    #[test]
    fn serde_roundtrips_as_plain_string() {
        let json = serde_json::to_string(&InstrumentName::EurUsd).unwrap();
        assert_eq!(json, r#""EUR_USD""#);
        let back: InstrumentName = serde_json::from_str(&json).unwrap();
        assert_eq!(back, InstrumentName::EurUsd);
        let other: InstrumentName = serde_json::from_str(r#""NEW_SYMBOL""#).unwrap();
        assert_eq!(other, InstrumentName::Other("NEW_SYMBOL".to_owned()));
    }

    #[test]
    fn known_table_is_consistent() {
        assert_eq!(InstrumentName::KNOWN.len(), 123);
        for known in InstrumentName::KNOWN {
            assert!(known.is_known());
            assert_eq!(&InstrumentName::from(known.as_str()), known);
        }
    }
}
