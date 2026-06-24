use regex::Regex;

pub fn compress_logs(text: &str, max_lines: usize) -> String {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() <= max_lines {
        return text.to_string();
    }

    // Regex to match verbose compilation and download patterns
    let verbose_re = Regex::new(
        r"(?i)\b(compiling|building|restoring|downloading|extracting|pulling|progress|download)\b"
    ).unwrap();

    let critical_re = Regex::new(
        r"(?i)\b(error|warn|fail|exception|critical|fatal)\b"
    ).unwrap();

    let mut compressed_lines = Vec::new();
    let mut skip_count = 0;

    for line in lines {
        // If line is critical, keep it
        if critical_re.is_match(line) {
            if skip_count > 0 {
                compressed_lines.push(format!("... [Skipped {} verbose lines] ...", skip_count));
                skip_count = 0;
            }
            compressed_lines.push(line.to_string());
        } 
        // If line is verbose, skip it
        else if verbose_re.is_match(line) {
            skip_count += 1;
        } 
        // Normal line, keep it
        else {
            if skip_count > 0 {
                compressed_lines.push(format!("... [Skipped {} verbose building lines] ...", skip_count));
                skip_count = 0;
            }
            compressed_lines.push(line.to_string());
        }
    }

    if skip_count > 0 {
        compressed_lines.push(format!("... [Skipped {} verbose building lines] ...", skip_count));
    }

    // Crop the middle if it still exceeds max_lines
    if compressed_lines.len() > max_lines {
        let half = max_lines / 2;
        let mut final_lines = Vec::new();
        
        for i in 0..half {
            final_lines.push(compressed_lines[i].clone());
        }
        
        let skipped_middle = compressed_lines.len() - max_lines;
        final_lines.push(format!("\n... [TRUNCATED {} MIDDLE LINES FOR CONTEXT SAVING] ...\n", skipped_middle));
        
        let start_idx = compressed_lines.len() - half;
        for i in start_idx..compressed_lines.len() {
            final_lines.push(compressed_lines[i].clone());
        }
        
        return final_lines.join("\n");
    }

    compressed_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verbose_skipping() {
        let log = "Compiling module A...\n\
                   Compiling module B...\n\
                   ERROR: Failed to compile module B!\n\
                   Compiling module C...\n\
                   Compiling module D...";
                   
        let compressed = compress_logs(log, 50);
        assert!(compressed.contains("... [Skipped 2 verbose lines] ..."));
        assert!(compressed.contains("ERROR: Failed to compile module B!"));
        assert!(compressed.contains("... [Skipped 2 verbose building lines] ..."));
    }

    #[test]
    fn test_middle_truncation() {
        let mut log = String::new();
        for i in 1..=200 {
            log.push_str(&format!("Line number {}\n", i));
        }
        
        let compressed = compress_logs(&log, 20);
        assert!(compressed.contains("... [TRUNCATED"));
        assert!(compressed.contains("Line number 1"));
        assert!(compressed.contains("Line number 200"));
    }
}
