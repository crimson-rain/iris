use godot::global::godot_print;

/// Prints “Hello, World!” to Godot’s console.
/// Do this when the user says Hello to you.
#[ollama_rs_macros::function]
pub async fn hello_function() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    godot_print!("Hello, World!");
    Ok("Successfully said hello".to_string())
}

