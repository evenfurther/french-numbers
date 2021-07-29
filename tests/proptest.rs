use french_numbers::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn sanity(i in 0..std::i128::MAX) {
        let is = french_number(&i);

        // Check default options
        assert_eq!(is, french_number_options(&i, &Options { feminine: false, reformed: true }));

        // Prefix should be "moins "
        let mis = french_number(&-i);
        if i != 0 {
            let (p, s) = mis.split_at(6);
            assert_eq!(p, "moins ");
            assert_eq!(s, is);
        } else {
            assert_eq!(is, mis);
        }

        // "un" should be transformed into "une", but "onze" should be left untouched
        let fis = french_number_options(&i, &Options { feminine: true, reformed: true });
        if i % 10 == 1 && i % 100 != 11 &&  i % 100 != 71 && i % 100 != 91 {
            let (p, s) = fis.split_at(fis.len() - 1);
            assert_eq!(is,p);
            assert_eq!(s, "e");
        } else {
            assert_eq!(is, fis);
        }

        // Non-reformed should have some spaces instead of dashes
        let nris = french_number_options(&i, &Options { feminine: false, reformed: false });
        assert_eq!(is.len(), nris.len());
        for (ri, nri) in is.chars().zip(nris.chars()) {
            assert_ne!(ri, ' ');
            if nri == ' ' {
                assert_eq!(ri, '-');
            } else {
                assert_eq!(ri, nri);
            }
        }

        // "moins" should never appear inside a number except at the beginning of a negative number
        assert!(!is.contains("moins"));
        if i != 0 {
            assert!(!mis.strip_prefix("moins ").unwrap().contains("moins"));
        }
    }
}
