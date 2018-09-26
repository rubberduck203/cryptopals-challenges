use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/english_frequencies.rs"));

pub fn english(message: &str) -> bool {
    let expected_counts: HashMap<char, f32> = english_frequencies()
        .iter()
        .map(|(k, freq)| (k.clone() as char, (freq / 100.0) * (message.len() as f32)))
        .collect();

    let actual_counts = message
        .chars()
        .fold(HashMap::new(), |mut acc: HashMap<char, isize>, c| {
            let count = match acc.get(&c) {
                Some(x) => x.clone() + 1,
                None => 1,
            };

            acc.insert(c, count);
            acc
        });

    let chi_statistic = chi_statistic(&actual_counts, &expected_counts);
    if cfg!(debug_assertions) {
        println!("Expected: {:#?}", expected_counts);
        println!("Actual: {:#?}", actual_counts);
        println!("X-statistic: {}", chi_statistic);
    }

    //  Degrees of freedom = 256 - 1 = 255 (character space)
    //  Usign this table:
    //  https://en.wikibooks.org/wiki/Engineering_Tables/Chi-Squared_Distibution
    //  We can use the approximate value for 250 degrees of fredom.
    //  Given a significance factor (alpha) of 0.05, our critical value is 287.882.
    //  If our chi_statistic is < the critical_value, then we have a match.
    //  See this page for an explanation:
    //  https://en.wikipedia.org/wiki/Chi-squared_distribution#Table_of_%CF%872_values_vs_p-values
    chi_statistic < 287.882
}

/// Calculates Pearson's Cumulative Chi Statistic
/// https://en.wikipedia.org/wiki/Pearson%27s_chi-squared_test#Calculating_the_test-statistic
///
/// This is a slight variation.
/// Technichally, if the expected value is zero and the actual is non-zero, then the statistic is infinite.
/// For the sake of ergonommics, this implementation assumes missing expected values to be small, but non-zero.
/// This allows us to only specify values in the expected frequencies that are statistically
/// significant while allowing for all valid utf-8 characters in the message.
fn chi_statistic(observed: &HashMap<char, isize>, expected: &HashMap<char, f32>) -> f32 {
    observed
        .into_iter()
        .map(|(key, obs)| {
            let exp = match expected.get(&key) {
                Some(x) => x.clone() as f32,
                None => 0.0000001, //non-zero, but tiny possibility
            };

            (*obs as f32 - exp).powi(2) / exp
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bacon_message_is_english() {
        let message = "Cooking MC's like a pound of bacon";
        assert!(english(message));
    }

    #[test]
    fn message_with_new_line_is_english() {
        let message = "Now that the party is jumping\n";
        assert!(english(message));
    }

    #[test]
    fn message_with_unprintable_chars_is_not_english() {
        assert!(!english(
            "\u{7f}SSWUR[\u{1c}q\u{7f}\u{1b}O\u{1c}PUWY\u{1c}]\u{1c}LSIRX\u{1c}SZ\u{1c}^]_SR"
        ));
    }

    #[test]
    fn printable_nonsense_is_not_english() {
        assert!(!english("Yuuqst}:WY=i:vsq\u{7f}:{:juot~:u|:x{yut"));
    }

    #[test]
    fn readable_but_incorrect_is_not_english() {
        assert!(!english(
            "cOOKING\u{0}mc\u{7}S\u{0}LIKE\u{0}A\u{0}POUND\u{0}OF\u{0}BACON"
        ));
    }
}
