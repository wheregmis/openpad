//! File operations and search example.
//!
//! This example demonstrates:
//! - Searching for text in files
//! - Finding files by pattern
//! - Reading file contents
//! - Getting file status
//! - Searching for symbols
//!
//! To run this example:
//! ```bash
//! cargo run --example file_operations
//! ```

use openpad_protocol::{
    FileReadRequest, FilesSearchRequest, OpenCodeClient, SymbolsSearchRequest, TextSearchRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OpenCode Protocol Client - File Operations Example");
    println!("===================================================\n");

    let client = OpenCodeClient::new("http://localhost:4096");

    // Check server health first
    match client.health().await {
        Ok(health) => {
            println!("✓ Connected to OpenCode server v{}\n", health.version);
        }
        Err(e) => {
            eprintln!("✗ Failed to connect to server: {}", e);
            eprintln!("  Make sure OpenCode server is running on http://localhost:4096");
            return Err(e.into());
        }
    }

    // 1. Search for text in files
    println!("1. Searching for 'OpenCodeClient' in Rust files...");
    match client
        .search_text(TextSearchRequest {
            pattern: "OpenCodeClient".to_string(),
        })
        .await
    {
        Ok(results) => {
            println!("   Found {} matches", results.len());
            for (i, result) in results.iter().take(5).enumerate() {
                println!(
                    "   {}. {} (line {})",
                    i + 1,
                    result.path,
                    result.line_number
                );
                println!("      {}", result.lines.trim());
            }
            if results.len() > 5 {
                println!("   ... and {} more matches", results.len() - 5);
            }
        }
        Err(e) => eprintln!("   ✗ Search failed: {}", e),
    }

    // 2. Find Rust files
    println!("\n2. Finding Rust source files...");
    match client
        .search_files(FilesSearchRequest {
            query: "*.rs".to_string(),
            type_filter: Some("file".to_string()),
            directory: None,
            limit: Some(10),
        })
        .await
    {
        Ok(files) => {
            println!("   Found {} Rust files (showing first 10)", files.len());
            for (i, file) in files.iter().enumerate() {
                println!("   {}. {}", i + 1, file);
            }
        }
        Err(e) => eprintln!("   ✗ Search failed: {}", e),
    }

    // 3. Find directories
    println!("\n3. Finding 'src' directories...");
    match client
        .search_files(FilesSearchRequest {
            query: "src".to_string(),
            type_filter: Some("directory".to_string()),
            directory: None,
            limit: Some(5),
        })
        .await
    {
        Ok(dirs) => {
            println!("   Found {} directories", dirs.len());
            for (i, dir) in dirs.iter().enumerate() {
                println!("   {}. {}", i + 1, dir);
            }
        }
        Err(e) => eprintln!("   ✗ Search failed: {}", e),
    }

    // 4. Read a file
    println!("\n4. Reading README.md...");
    match client
        .read_file(FileReadRequest {
            path: "README.md".to_string(),
        })
        .await
    {
        Ok(content) => {
            println!("   ✓ File read successfully");
            println!("   Type: {}", content.type_name);
            println!("   Content length: {} bytes", content.content.len());
            println!("   First 200 characters:");
            let preview = content.content.chars().take(200).collect::<String>();
            println!("   {}", preview);
            if content.content.len() > 200 {
                println!("   ...");
            }
        }
        Err(e) => eprintln!("   ✗ Failed to read file: {}", e),
    }

    // 5. Get file status (git status)
    println!("\n5. Getting file status...");
    match client.get_file_status(None).await {
        Ok(files) => {
            println!("   Found {} tracked files", files.len());
            for (i, file) in files.iter().take(10).enumerate() {
                println!(
                    "   {}. {} ({}) [+{}/-{}]",
                    i + 1,
                    file.path,
                    file.status,
                    file.added,
                    file.removed
                );
            }
            if files.len() > 10 {
                println!("   ... and {} more files", files.len() - 10);
            }
        }
        Err(e) => eprintln!("   ✗ Failed to get status: {}", e),
    }

    // 6. Search for symbols
    println!("\n6. Searching for 'Client' symbols...");
    match client
        .search_symbols(SymbolsSearchRequest {
            query: "Client".to_string(),
        })
        .await
    {
        Ok(symbols) => {
            println!("   Found {} symbols", symbols.len());
            for (i, symbol) in symbols.iter().take(10).enumerate() {
                println!("   {}. {} (kind {})", i + 1, symbol.name, symbol.kind);
                let loc = &symbol.location;
                println!(
                    "      at {} ({}:{} to {}:{})",
                    loc.uri,
                    loc.range.start.line,
                    loc.range.start.character,
                    loc.range.end.line,
                    loc.range.end.character
                );
            }
            if symbols.len() > 10 {
                println!("   ... and {} more symbols", symbols.len() - 10);
            }
        }
        Err(e) => eprintln!("   ✗ Search failed: {}", e),
    }

    println!("\n✓ File operations example completed!");
    Ok(())
}
