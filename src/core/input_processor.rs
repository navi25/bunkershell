// External crates
use anyhow::Result;

// Internal imports
use crate::ai::llm_manager::LLMManager;

/// Handles processing of user input, coordinating with the LLM manager
pub struct InputProcessor {
    llm_manager: LLMManager,
}

impl InputProcessor {
    /// Creates a new InputProcessor with an LLMManager instance
    pub fn new(llm_manager: LLMManager) -> Self {
        InputProcessor { llm_manager }
    }

    /// Processes the input string and returns a Result with the processed output
    pub async fn process(&self, input: &str) -> Result<String> {
        self.llm_manager.process_input(input).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_process() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let manager = LLMManager::new();
            let processor = InputProcessor::new(manager);
            let response = processor.process("test").await.unwrap();
            assert_eq!(response, "Dummy response: test");
        });
    }
}