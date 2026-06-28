#![allow(dead_code)]

use crate::kernel::models::{ModelRoute, ThinkingLevel};

pub const DEEPSEEK_AUTO_LABEL: &str = "DeepSeek Auto";
pub const DEEPSEEK_FLASH_MODEL: &str = "deepseek-v4-flash";
pub const DEEPSEEK_PRO_MODEL: &str = "deepseek-v4-pro";

pub fn effective_model(route: ModelRoute, thinking: ThinkingLevel) -> &'static str {
    match route {
        ModelRoute::Flash => DEEPSEEK_FLASH_MODEL,
        ModelRoute::Pro => DEEPSEEK_PRO_MODEL,
        ModelRoute::Auto => match thinking {
            ThinkingLevel::Fast => DEEPSEEK_FLASH_MODEL,
            ThinkingLevel::Auto | ThinkingLevel::Standard | ThinkingLevel::Deep => {
                DEEPSEEK_PRO_MODEL
            }
        },
    }
}

pub fn thinking_budget_name(thinking: ThinkingLevel) -> &'static str {
    match thinking {
        ThinkingLevel::Auto => "auto",
        ThinkingLevel::Fast => "none",
        ThinkingLevel::Standard => "high",
        ThinkingLevel::Deep => "max",
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel::models::{ModelRoute, ThinkingLevel};

    use super::{effective_model, thinking_budget_name, DEEPSEEK_FLASH_MODEL, DEEPSEEK_PRO_MODEL};

    #[test]
    fn auto_fast_uses_flash_model() {
        assert_eq!(
            effective_model(ModelRoute::Auto, ThinkingLevel::Fast),
            DEEPSEEK_FLASH_MODEL
        );
    }

    #[test]
    fn auto_deep_uses_pro_model() {
        assert_eq!(
            effective_model(ModelRoute::Auto, ThinkingLevel::Deep),
            DEEPSEEK_PRO_MODEL
        );
    }

    #[test]
    fn thinking_levels_map_to_budget_names() {
        assert_eq!(thinking_budget_name(ThinkingLevel::Auto), "auto");
        assert_eq!(thinking_budget_name(ThinkingLevel::Fast), "none");
        assert_eq!(thinking_budget_name(ThinkingLevel::Standard), "high");
        assert_eq!(thinking_budget_name(ThinkingLevel::Deep), "max");
    }
}
