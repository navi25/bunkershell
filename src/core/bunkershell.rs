// External crates
use anyhow::{Context, Result};
use tokio::runtime::Runtime;

use crate::ai::llm_manager::LLMManager;
use crate::core::input_processor::InputProcessor;

/// Main BunkerShell struct that coordinates the shell's functionality
pub struct BunkerShell {
    input_processor: InputProcessor,
}

impl BunkerShell {

    pub fn new() -> Self {
        let input_processor = InputProcessor::new(LLMManager::new());
        BunkerShell { input_processor }
    }
    

    pub fn run(&self) -> Result<()> {
        let rt = Runtime::new().context("Failed to create Tokio runtime")?;
        
        rt.block_on(async {
            println!("BunkerShell v0.1.0");
            println!("Type 'exit' or press Ctrl+C to quit\n");
            
            let result: Result<()> = async {
                loop {
                    print!(">> ");
                    
                    let mut input = String::new();
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {  
                            let input = input.trim();
                            
                            if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
                                break;
                            }
                            
                            if input.is_empty() {
                                continue;
                            }
                            
                            let response = self.input_processor.process(input).await?;
                            println!("\n{}\n", response);
                        }
                        Err(error) => {
                            return Err(anyhow::anyhow!("Error reading input: {}", error));
                        }
                    }
                }
                Ok(())
            }.await;
            
            println!("\nExiting BunkerShell. Goodbye!");
            result
        })
    }
}
