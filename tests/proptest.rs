use french_numbers::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn sanity(i in 0..std::i128::MAX) {
        let is = french_number(&i);

        // Check default options
        assert_eq!(is, french_number_options(&i, &Options { feminine: false, reformed: true }));
        let is = is.as_bytes();

        // Prefix should be "moins "
        let mis = french_number(&-i);
        let mis = mis.as_bytes();
        if i != 0 {
            assert_eq!(&mis[..6], b"moins ");
            assert_eq!(&mis[6..], is);
        } else {
            assert_eq!(is, mis);
        }

        // "un" should be transformed into "une", but "onze" should be left untouched
        let fis = french_number_options(&i, &Options { feminine: true, reformed: true });
        let fis = fis.as_bytes();
        if i % 10 == 1 && i % 100 != 11 &&  i % 100 != 71 && i % 100 != 91 {
            assert_eq!(is, &fis[..fis.len()-1]);
            assert_eq!(fis[fis.len()-1], b'e');
        } else {
            assert_eq!(is, fis);
        }

        // non-reformed should have some spaces instead of dashes
        let nris = french_number_options(&i, &Options { feminine: false, reformed: false });
        let nris = nris.as_bytes();
        assert_eq!(is.len(), nris.len());
        for k in 0..is.len() {
            assert!((is[k] == b'-' && nris[k] == b' ') ^ (is[k] != b' ' && is[k] == nris[k]));
        }
    }
}
