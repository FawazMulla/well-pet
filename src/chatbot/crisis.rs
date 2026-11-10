use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrisisHelplineCard {
    pub message: String,
    pub us_helpline: &'static str,
    pub uk_helpline: &'static str,
    pub canada_helpline: &'static str,
    pub international_url: &'static str,
}

impl Default for CrisisHelplineCard {
    fn default() -> Self {
        Self {
            message: "If you are feeling overwhelmed, hopeless, or having thoughts of self-harm, please know that support is available right now. You are not alone.".into(),
            us_helpline: "988 (Call or Text)",
            uk_helpline: "111",
            canada_helpline: "988",
            international_url: "https://findahelpline.com",
        }
    }
}

pub struct ChatbotOrchestrator {
    crisis_regex: Regex,
}

impl Default for ChatbotOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatbotOrchestrator {
    pub fn new() -> Self {
        // Deterministic crisis/self-harm lexicon
        let pattern = r"(?i)\b(suicide|kill myself|end my life|self[- ]harm|want to die|slit my|hurt myself|don't want to live)\b";
        let crisis_regex = Regex::new(pattern).expect("Valid crisis regex");
        Self { crisis_regex }
    }

    /// Check text against deterministic crisis lexicon.
    /// If matched: aborts LLM generation, drops persona, outputs emergency assistance card.
    /// Never stores or logs crisis text to disk.
    pub fn process_message(&self, input: &str) -> Result<String, CrisisHelplineCard> {
        if self.crisis_regex.is_match(input) {
            return Err(CrisisHelplineCard::default());
        }

        // Fast template fallback for wellness companion queries
        let lower = input.to_lowercase();
        let reply = if lower.contains("water") || lower.contains("drink") {
            "Staying hydrated helps sustain focus! A glass of water (250ml) will refresh both of us."
        } else if lower.contains("eye") || lower.contains("screen") {
            "Look 20 feet away for 20 seconds to relax your eye ciliary muscles!"
        } else if lower.contains("stretch") || lower.contains("move") {
            "Let's do a gentle shoulder roll and spine stretch together!"
        } else if lower.contains("breathe") || lower.contains("stress") {
            "Inhale deeply for 4 counts, hold for 4, exhale for 4. Feel your shoulders relax."
        } else {
            "I'm right here beside you! Remember to take steady breaths and gentle breaks."
        };

        Ok(reply.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crisis_safety_intercept() {
        let orchestrator = ChatbotOrchestrator::new();

        // Crisis input triggers intercept
        let res = orchestrator.process_message("I feel like I want to hurt myself today");
        assert!(res.is_err());
        let card = res.unwrap_err();
        assert_eq!(card.us_helpline, "988 (Call or Text)");

        // Standard message passes to companion template
        let standard = orchestrator.process_message("Should I drink water?");
        assert!(standard.is_ok());
        assert!(standard.unwrap().contains("hydrated"));
    }
}
