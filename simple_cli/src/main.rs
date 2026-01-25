//! xr-safety CLI: Accessible tool for checking safety rules
//! No complex setup, just compile and run

use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        println!("Usage: xr-safety summary <aln-file>");
        println!("Example: xr-safety summary quantum-roaming-debug.v1.aln");
        return;
    }
    
    let command = &args[1];
    let filename = &args[2];
    
    if command == "summary" {
        print_safety_summary(filename);
    }
}

fn print_safety_summary(filename: &str) {
    println!("=== XR SAFETY SUMMARY ===");
    println!("File: {}", filename);
    
    // Read and parse ALN shard (simplified)
    if let Ok(content) = fs::read_to_string(filename) {
        // Extract key values (simplified parsing)
        let sleeptoken = extract_value(&content, "sleeptoken").unwrap_or(0.0);
        let psychriskscore = extract_value(&content, "psychriskscore").unwrap_or(0.0);
        let enstasisscore = extract_value(&content, "enstasisscore").unwrap_or(1.0);
        let sleepstage = extract_string(&content, "sleepstage").unwrap_or("wake".to_string());
        let dreammode = extract_string(&content, "dreammode").unwrap_or("passive".to_string());
        
        // Calculate
        let e = sleeptoken * (1.0 - psychriskscore) * enstasisscore;
        let allowed = e >= 0.5 && psychriskscore <= 0.35 && 
                     (sleepstage == "N2" || sleepstage == "N3") &&
                     dreammode == "quantum_consciousness";
        
        // Simple output
        println!("Sleep stage: {}", sleepstage);
        println!("Dream mode: {}", dreammode);
        println!("Risk score: {:.2}", psychriskscore);
        println!("Eligibility E: {:.2}", e);
        println!("Quantum roaming allowed: {}", if allowed { "YES" } else { "NO" });
        
        if !allowed {
            println!("\n⚠️  BLOCKED REASONS:");
            if e < 0.5 { println!("- Eligibility {:.2} < 0.5", e); }
            if psychriskscore > 0.35 { println!("- Risk {:.2} > 0.35", psychriskscore); }
            if sleepstage != "N2" && sleepstage != "N3" { 
                println!("- Sleep stage {} not N2/N3", sleepstage); 
            }
            if dreammode != "quantum_consciousness" { 
                println!("- Dream mode {} not quantum_consciousness", dreammode); 
            }
        }
    } else {
        println!("Could not read file. Here's a template ALN to create:");
        print_aln_template();
    }
}
