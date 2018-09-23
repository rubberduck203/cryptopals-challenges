use std::collections::HashMap;

pub fn english(message: &str) -> bool {
    let expected_counts: HashMap<char, f32> = frequencies()
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

    let chi_statistic = chi_statistic(actual_counts, expected_counts);
    if cfg!(debug_assertions) {
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
fn chi_statistic(observed: HashMap<char, isize>, expected: HashMap<char, f32>) -> f32 {
    observed
        .into_iter()
        .map(|(key, obs)| {
            let exp = match expected.get(&key) {
                Some(x) => x.clone() as f32,
                None => 0.0000001, //non-zero, but tiny possibility
            };

            (obs as f32 - exp).powi(2) / exp
        }).sum()
}

/// Letter frequency taken from this dataset.
/// http://www.fitaly.com/board/domper3/posts/136.html
fn frequencies() -> HashMap<u8, f32> {
    // TODO: Use some code gen to clean this up
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#case-study-code-generation
    [
        (32, 17.1660),
        (101, 8.5771),
        (116, 6.3700),
        (111, 5.7701),
        (97, 5.1880),
        (110, 4.9701),
        (105, 4.9019),
        (115, 4.3686),
        (114, 4.2586),
        (108, 3.1750),
        (104, 2.7444),
        (100, 2.5071),
        (99, 2.1129),
        (117, 2.0999),
        (109, 1.6437),
        (103, 1.5597),
        (112, 1.5482),
        (46, 1.5124),
        (45, 1.3734),
        (102, 1.3725),
        (119, 1.3034),
        (121, 1.1330),
        (98, 1.0195),
        (118, 0.8462),
        (44, 0.7384),
        (107, 0.6753),
        (149, 0.6410),
        (48, 0.5516),
        (49, 0.4594),
        (58, 0.4354),
        (83, 0.4003),
        (67, 0.3906),
        (77, 0.3529),
        (50, 0.3322),
        (84, 0.3322),
        (73, 0.3211),
        (68, 0.3151),
        (65, 0.3132),
        (69, 0.2673),
        (80, 0.2614),
        (87, 0.2527),
        (82, 0.2519),
        (39, 0.2447),
        (34, 0.2442),
        (72, 0.2321),
        (41, 0.2233),
        (40, 0.2178),
        (66, 0.2163),
        (78, 0.2085),
        (120, 0.1950),
        (76, 0.1884),
        (71, 0.1876),
        (51, 0.1847),
        (79, 0.1842),
        (74, 0.1726),
        (53, 0.1663),
        (47, 0.1549),
        (63, 0.1474),
        (70, 0.1416),
        (52, 0.1348),
        (62, 0.1242),
        (60, 0.1225),
        (59, 0.1214),
        (95, 0.1159),
        (54, 0.1153),
        (56, 0.1054),
        (55, 0.1030),
        (57, 0.1024),
        (86, 0.0892),
        (106, 0.0867),
        (85, 0.0814),
        (113, 0.0747),
        (75, 0.0687),
        (42, 0.0628),
        (122, 0.0596),
        (36, 0.0561),
        (88, 0.0343),
        (81, 0.0316),
        (89, 0.0304),
        (61, 0.0227),
        (38, 0.0226),
        (43, 0.0215),
        (35, 0.0179),
        (37, 0.0160),
        (93, 0.0088),
        (91, 0.0086),
        (90, 0.0076),
        (64, 0.0073),
        (33, 0.0072),
        (9, 0.0057),
        (123, 0.0026),
        (125, 0.0026),
        (92, 0.0016),
        (183, 0.0010),
        (96, 0.0009),
        (124, 0.0007),
        (94, 0.0003),
        (126, 0.0003),
    ]
        .iter()
        .cloned()
        .collect()
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
