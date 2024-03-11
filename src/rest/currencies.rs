use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref DIGITS_BY_CODES: HashMap<&'static str, u32> = {
        let mut map = HashMap::with_capacity(200);
        map.insert("AED", 2);
        map.insert("AFN", 2);
        map.insert("ALL", 2);
        map.insert("AMD", 2);
        map.insert("ANG", 2);
        map.insert("AOA", 2);
        map.insert("ARS", 2);
        map.insert("AUD", 2);
        map.insert("AWG", 2);
        map.insert("AZN", 2);
        map.insert("BAM", 2);
        map.insert("BBD", 2);
        map.insert("BDT", 2);
        map.insert("BGN", 2);
        map.insert("BHD", 3);
        map.insert("BIF", 0);
        map.insert("BMD", 2);
        map.insert("BND", 2);
        map.insert("BOB", 2);
        map.insert("BOV", 2);
        map.insert("BRL", 2);
        map.insert("BSD", 2);
        map.insert("BTN", 2);
        map.insert("BWP", 2);
        map.insert("BYN", 2);
        map.insert("BZD", 2);
        map.insert("CAD", 2);
        map.insert("CDF", 2);
        map.insert("CHE", 2);
        map.insert("CHF", 2);
        map.insert("CHW", 2);
        map.insert("CLF", 4);
        map.insert("CLP", 0);
        map.insert("CNY", 2);
        map.insert("COP", 2);
        map.insert("COU", 2);
        map.insert("CRC", 2);
        map.insert("CUC", 2);
        map.insert("CUP", 2);
        map.insert("CVE", 0);
        map.insert("CZK", 2);
        map.insert("DJF", 0);
        map.insert("DKK", 2);
        map.insert("DOP", 2);
        map.insert("DZD", 2);
        map.insert("EGP", 2);
        map.insert("ERN", 2);
        map.insert("ETB", 2);
        map.insert("EUR", 2);
        map.insert("FJD", 2);
        map.insert("FKP", 2);
        map.insert("GBP", 2);
        map.insert("GEL", 2);
        map.insert("GHS", 2);
        map.insert("GIP", 2);
        map.insert("GMD", 2);
        map.insert("GNF", 0);
        map.insert("GTQ", 2);
        map.insert("GYD", 2);
        map.insert("HKD", 2);
        map.insert("HNL", 2);
        map.insert("HRK", 2);
        map.insert("HTG", 2);
        map.insert("HUF", 2);
        map.insert("IDR", 2);
        map.insert("ILS", 2);
        map.insert("INR", 2);
        map.insert("IQD", 3);
        map.insert("IRR", 2);
        map.insert("ISK", 0);
        map.insert("JMD", 2);
        map.insert("JOD", 3);
        map.insert("JPY", 0);
        map.insert("KES", 2);
        map.insert("KGS", 2);
        map.insert("KHR", 2);
        map.insert("KMF", 0);
        map.insert("KPW", 2);
        map.insert("KRW", 0);
        map.insert("KWD", 3);
        map.insert("KYD", 2);
        map.insert("KZT", 2);
        map.insert("LAK", 2);
        map.insert("LBP", 2);
        map.insert("LKR", 2);
        map.insert("LRD", 2);
        map.insert("LSL", 2);
        map.insert("LYD", 3);
        map.insert("MAD", 2);
        map.insert("MDL", 2);
        map.insert("MGA", 1);
        map.insert("MKD", 2);
        map.insert("MMK", 2);
        map.insert("MNT", 2);
        map.insert("MOP", 2);
        map.insert("MRU", 1);
        map.insert("MUR", 2);
        map.insert("MVR", 2);
        map.insert("MWK", 2);
        map.insert("MXN", 2);
        map.insert("MXV", 2);
        map.insert("MYR", 2);
        map.insert("MZN", 2);
        map.insert("NAD", 2);
        map.insert("NGN", 2);
        map.insert("NIO", 2);
        map.insert("NOK", 2);
        map.insert("NPR", 2);
        map.insert("NZD", 2);
        map.insert("OMR", 3);
        map.insert("PAB", 2);
        map.insert("PEN", 2);
        map.insert("PGK", 2);
        map.insert("PHP", 2);
        map.insert("PKR", 2);
        map.insert("PLN", 2);
        map.insert("PYG", 0);
        map.insert("QAR", 2);
        map.insert("RON", 2);
        map.insert("RSD", 2);
        map.insert("RUB", 2);
        map.insert("RWF", 0);
        map.insert("SAR", 2);
        map.insert("SBD", 2);
        map.insert("SCR", 2);
        map.insert("SDG", 2);
        map.insert("SEK", 2);
        map.insert("SGD", 2);
        map.insert("SHP", 2);
        map.insert("SLL", 2);
        map.insert("SOS", 2);
        map.insert("SRD", 2);
        map.insert("SSP", 2);
        map.insert("STN", 2);
        map.insert("SVC", 2);
        map.insert("SYP", 2);
        map.insert("SZL", 2);
        map.insert("THB", 2);
        map.insert("TJS", 2);
        map.insert("TMT", 2);
        map.insert("TND", 3);
        map.insert("TOP", 2);
        map.insert("TRY", 2);
        map.insert("TTD", 2);
        map.insert("TWD", 2);
        map.insert("TZS", 2);
        map.insert("UAH", 2);
        map.insert("UGX", 0);
        map.insert("USD", 2);
        map.insert("UYI", 0);
        map.insert("UYU", 2);
        map.insert("UYW", 4);
        map.insert("UZS", 2);
        map.insert("VES", 2);
        map.insert("VND", 0);
        map.insert("VUV", 0);
        map.insert("WST", 2);
        map.insert("XAF", 0);
        map.insert("XCD", 2);
        map.insert("XDR", 0);
        map.insert("XOF", 0);
        map.insert("XPF", 0);
        map.insert("XSU", 2);
        map.insert("YER", 2);
        map.insert("ZAR", 2);
        map.insert("ZMW", 2);
        map.insert("ZWL", 2);

        map
    };
}
