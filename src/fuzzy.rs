use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub fn fuzzy_filter(names: &[String], query: &str) -> Vec<String> {
    if query.is_empty() {
        return names.to_vec();
    }
    let matcher = SkimMatcherV2::default();
    let mut scored: Vec<(i64, &String)> = names
        .iter()
        .filter_map(|n| matcher.fuzzy_match(n, query).map(|score| (score, n)))
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, n)| n.clone()).collect()
}
