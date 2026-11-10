use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppCategory {
    IDE,
    Browser,
    Document,
    Meeting,
    Media,
    Other,
}

pub struct ProcessCategorizer;

impl ProcessCategorizer {
    /// Categorizes binary executable name into abstract category.
    /// Explicitly strips window titles, browser URLs, file names, etc.
    pub fn categorize(binary_name: &str) -> AppCategory {
        let bin_lower = binary_name.to_lowercase();
        let bin_clean = bin_lower.trim();

        if bin_clean.contains("code")
            || bin_clean.contains("idea")
            || bin_clean.contains("cursor")
            || bin_clean.contains("sublime")
            || bin_clean.contains("nvim")
            || bin_clean.contains("vim")
            || bin_clean.contains("clion")
            || bin_clean.contains("pycharm")
            || bin_clean.contains("rustrover")
            || bin_clean.contains("terminal")
            || bin_clean.contains("wezterm")
            || bin_clean.contains("alacritty")
        {
            AppCategory::IDE
        } else if bin_clean.contains("chrome")
            || bin_clean.contains("firefox")
            || bin_clean.contains("edge")
            || bin_clean.contains("safari")
            || bin_clean.contains("brave")
            || bin_clean.contains("vivaldi")
            || bin_clean.contains("zen")
        {
            AppCategory::Browser
        } else if bin_clean.contains("word")
            || bin_clean.contains("notion")
            || bin_clean.contains("obsidian")
            || bin_clean.contains("acrobat")
            || bin_clean.contains("preview")
            || bin_clean.contains("pages")
            || bin_clean.contains("libreoffice")
        {
            AppCategory::Document
        } else if bin_clean.contains("zoom")
            || bin_clean.contains("teams")
            || bin_clean.contains("meet")
            || bin_clean.contains("slack")
            || bin_clean.contains("discord")
            || bin_clean.contains("webex")
            || bin_clean.contains("facetime")
        {
            AppCategory::Meeting
        } else if bin_clean.contains("spotify")
            || bin_clean.contains("vlc")
            || bin_clean.contains("music")
            || bin_clean.contains("netflix")
            || bin_clean.contains("youtube")
        {
            AppCategory::Media
        } else {
            AppCategory::Other
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_categorization_and_stripping() {
        assert_eq!(ProcessCategorizer::categorize("code.exe"), AppCategory::IDE);
        assert_eq!(ProcessCategorizer::categorize("Google Chrome"), AppCategory::Browser);
        assert_eq!(ProcessCategorizer::categorize("obsidian"), AppCategory::Document);
        assert_eq!(ProcessCategorizer::categorize("zoom.us"), AppCategory::Meeting);
        assert_eq!(ProcessCategorizer::categorize("spotify"), AppCategory::Media);
        assert_eq!(ProcessCategorizer::categorize("calculator"), AppCategory::Other);
    }
}
