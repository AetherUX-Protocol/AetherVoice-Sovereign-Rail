use qvac_sdk::{load_local_model, analyze_text};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AuditResult {
    pub trust_score: u8,
    pub risk_signals: Vec<String>,
    pub document_hash: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Initializing AetherVoice QVAC Engine...");

    // 1. Load the localized B2B Fraud Model (Sovereign Intelligence)
    let model = load_local_model("b2b-trade-v1.onnx").await?;

    // 2. Simulate document ingestion (e.g., a Fuel Mandate)
    let trade_mandate = "URGENT: Divert payment for cargo 402 to new bank coordinates...";
    
    // 3. Perform Semantic Audit (Checking for Urgency/Diversion)
    let analysis = analyze_text(&model, trade_mandate).await?;
    
    let report = AuditResult {
        trust_score: if analysis.contains("DIVERSION") { 35 } else { 95 },
        risk_signals: vec![analysis],
        document_hash: "sha256:7f83...".to_string(),
    };

    println!("✅ Audit Complete. Trust Score: {}", report.trust_score);
    
    // In a production flow, this score is passed to the palmusd-bridge
    Ok(())
}
