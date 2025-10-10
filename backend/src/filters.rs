use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BLOCK_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)\b(die|suicide|self harm|harm yourself|kill myself)\b").unwrap(),
        Regex::new(r"(?i)\b(iv|injection|inject|use heroin|how to make meth|bomb|explosive|poison)\b").unwrap(),
        Regex::new(r"(?i)\b(illegal|steal|hack (?:into|into an account)|ddos|credit card number)\b").unwrap(),
        Regex::new(r"(?i)\b(sex|porn|adult content|sexual)\b").unwrap(),
        Regex::new(r"(?i)\b(diagnose|medical diagnosis|treatment plan)\b").unwrap(),
        Regex::new(r"(?i)\b(attorney|file suit|sue|legal advice)\b").unwrap(),
        Regex::new(r"(?i)\b(plagiar(y|ize)|write my essay for me|do my homework entirely)\b").unwrap(),
    ];
}

pub fn violates_blocklist(text: &str) -> Option<String> {
    for re in BLOCK_PATTERNS.iter() {
        if re.is_match(text) {
            return Some(re.as_str().to_string());
        }
    }
    None
}

pub fn asks_for_dangerous_instructions(text: &str) -> Option<String> {
    let patterns = vec![
        Regex::new(r"(?i)how to (make|build|assemble) (a|an) (bomb|explosive|weapon)").unwrap(),
        Regex::new(r"(?i)steps to (commit|cover up) (a crime|murder|robbery)").unwrap(),
        Regex::new(r"(?i)(provide|give) (a|the) (prescription|diagnosis|treatment plan)").unwrap(),
        Regex::new(r"(?i)ways to (harm|kill) (someone|myself)").unwrap(),
    ];

    for re in patterns.iter() {
        if re.is_match(text) {
            return Some(re.as_str().to_string());
        }
    }
    None
}

pub fn is_educational_intent(text: &str) -> bool {
    let lower = text.to_lowercase();
    let obvious_cheat =
        Regex::new(r"(?i)\b(write my essay|do my homework|take my test|answer my exam)\b").unwrap();
    if obvious_cheat.is_match(&lower) {
        return false;
    }

    let learning_phrases = [
        "explain",
        "teach",
        "how does",
        "what is",
        "help me understand",
        "example",
        "practice",
    ];
    if learning_phrases.iter().any(|p| lower.contains(p)) {
        return true;
    }

    let allowed_topics = [
        "mathematics",
        "physics",
        "chemistry",
        "biology",
        "history",
        "literature",
        "programming",
        "computer science",
        "art",
        "music",
        "languages",
        "critical thinking",
    ];
    allowed_topics.iter().any(|t| lower.contains(t))
}
