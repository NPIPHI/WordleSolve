
#[cfg(test)]
mod tests {
    use crate::guess_matcher::GuessMatcher;
    use crate::word::to_word;

    #[test]
    fn match_same() {
        let ans = to_word("abcde");
        let guess = to_word("abcde");
        let matcher = GuessMatcher::from_guess(guess, ans);


        assert!(matcher.matches(guess));
        assert!(!matcher.matches(to_word("abcdg")));
    }
    #[test]
    fn match_too_many() {
        let ans = to_word("aabbb");
        let guess = to_word("cccaa");
        let matcher = GuessMatcher::from_guess(guess, ans);

        assert!(!matcher.matches(to_word("abbbb")));
        assert!(!matcher.matches(to_word("bbbbb")));
        assert!(!matcher.matches(to_word("bbbba")));
        assert!(!matcher.matches(to_word("bbabb")));
    }
    #[test]
    fn match_too_few() {
        let ans = to_word("abbba");
        let guess = to_word("faaff");
        let matcher = GuessMatcher::from_guess(guess, ans);

        assert!(!matcher.matches(to_word("abbbb")));
        assert!(!matcher.matches(to_word("babbb")));
        assert!(!matcher.matches(to_word("bbbbb")));
        assert!(!matcher.matches(to_word("eeeee")));
    }
    #[test]
    fn match_doesnt_have() {
        let ans = to_word("abcde");
        let guess = to_word("lmnop");
        let matcher = GuessMatcher::from_guess(guess, ans);

        assert!(!matcher.matches(to_word("lrtyu")));
        assert!(!matcher.matches(to_word("mrtyu")));
        assert!(!matcher.matches(to_word("rtnyu")));
        assert!(!matcher.matches(to_word("prtyu")));
    }

    #[test]
    fn match_previous_conflict() {
        let ans = to_word("abcde");
        let guess = to_word("bcdea");
        let matcher = GuessMatcher::from_guess(guess, ans);

        assert!(!matcher.matches(to_word("babcd")));
        assert!(!matcher.matches(to_word("abced")));

        let ans = to_word("aaqqq");
        let guess = to_word("laall");
        let matcher = GuessMatcher::from_guess(guess, ans);

        assert!(!matcher.matches(to_word("aqaqq")));
        assert!(!matcher.matches(to_word("aqqal")));
    }

    #[test]
    fn matcher_from_mask() {
        let m = GuessMatcher::from_guess(to_word("abcde"), to_word("fghij"));
        let m2 = GuessMatcher::from_result(to_word("abcde"), "00000");

        assert_eq!(m, m2);

        let m = GuessMatcher::from_guess(to_word("locus"), to_word("cabal"));
        let m2 = GuessMatcher::from_result(to_word("locus"), "10100");

        assert_eq!(m, m2);

        let m = GuessMatcher::from_guess(to_word("roate"), to_word("aloft"));
        let m2 = GuessMatcher::from_result(to_word("roate"), "01110");

        assert_eq!(m, m2);

        let m = GuessMatcher::from_guess(to_word("bloat"), to_word("aloft"));
        let m2 = GuessMatcher::from_result(to_word("bloat"), "02212");

        assert_eq!(m, m2);

        let m = GuessMatcher::from_guess(to_word("aloft"), to_word("aloft"));
        let m2 = GuessMatcher::from_result(to_word("aloft"), "22222");

        assert_eq!(m, m2);
    }
}