//! Safety eligibility calculations for Reality.os
//! Implements E = S(1-R)Es with neurorights guards

/// Calculate accessibility-adjusted output
/// A = 1 - cognitiveloadband (from your research action)
/// When A < 0.7, use simplified explanations
pub fn adjust_for_cognitive_load(cognitive_load_band: f32) -> OutputComplexity {
    let accessibility_score = 1.0 - cognitive_load_band.clamp(0.0, 1.0);
    
    match accessibility_score {
        a if a >= 0.7 => OutputComplexity::Detailed,
        a if a >= 0.4 => OutputComplexity::Standard,
        _ => OutputComplexity::Simplified,
    }
}

/// Core eligibility calculation matching your ALN exactly
pub fn calculate_eligibility(s: f32, r: f32, es: f32) -> f32 {
    // E = S(1-R)Es exactly as in your shard
    let clamped_s = s.clamp(0.0, 1.0);
    let clamped_r = r.clamp(0.0, 1.0);
    let clamped_es = es.clamp(0.0, 1.0);
    
    clamped_s * (1.0 - clamped_r) * clamped_es
}

/// Check ALL guards for quantum roaming
pub fn is_quantum_roaming_allowed(
    sleep_stage: &str,
    s: f32,
    r: f32,
    es: f32,
    dream_mode: &str,
) -> (bool, Vec<String>) {
    let mut reasons = Vec::new();
    
    // Guard 1: Sleep stage in {N2, N3}
    let valid_stage = matches!(sleep_stage, "N2" | "N3");
    if !valid_stage {
        reasons.push(format!("Sleep stage {} not in N2,N3", sleep_stage));
    }
    
    // Guard 2: Calculate E
    let e = calculate_eligibility(s, r, es);
    let meets_eligibility = e >= 0.5;
    if !meets_eligibility {
        reasons.push(format!("Eligibility E={:.2} < 0.5", e));
    }
    
    // Guard 3: Psychrisk ceiling
    let below_risk_limit = r <= 0.35;
    if !below_risk_limit {
        reasons.push(format!("Risk R={:.2} > 0.35", r));
    }
    
    // Guard 4: Dream mode
    let correct_mode = dream_mode == "quantum_consciousness";
    if !correct_mode {
        reasons.push(format!("Dream mode {} not quantum_consciousness", dream_mode));
    }
    
    let allowed = valid_stage && meets_eligibility && below_risk_limit && correct_mode;
    (allowed, reasons)
}
