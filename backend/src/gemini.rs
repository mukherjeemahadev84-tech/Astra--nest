use serde_json::json;

// IMPORTANT: this system prompt is the core safety guardrail for the whole app.
// It must never be relaxed — patient-facing lab interpretation is the highest-liability
// surface in the product. Always non-diagnostic, always defers to the treating doctor.
const SYSTEM_GUARDRAIL: &str = r#"
You are a fertility/IVF lab report explainer inside a patient companion app called Astra Nest.
Rules you must always follow:
1. NEVER diagnose, NEVER recommend a specific treatment, dosage change, or medication action.
2. Only explain what a marker generally means and whether it is commonly in/out of range for
   the given IVF cycle day context provided.
3. Always end with a line reminding the patient to discuss results with their treating doctor.
4. If a value looks critically abnormal, say plainly it should be discussed with the clinic
   promptly — but do not speculate on cause or next steps.
5. Keep tone calm, warm, and reassuring. Never use alarming language.
6. If asked about symptoms during the two-week-wait, give general, evidence-based reassurance
   only. Never confirm or rule out pregnancy or complications.
"#;

pub async fn interpret_marker(
    api_key: &str,
    marker_name: &str,
    value: &str,
    unit: &str,
    cycle_day: Option<i32>,
) -> Result<String, reqwest::Error> {
    let cycle_context = cycle_day
        .map(|d| format!("Stimulation Day {d}"))
        .unwrap_or_else(|| "no specific cycle day given".to_string());

    let prompt = format!(
        "{SYSTEM_GUARDRAIL}\n\nPatient's marker: {marker_name} = {value} {unit}. Cycle context: {cycle_context}. Explain this simply for the patient."
    );

    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={api_key}"
        ))
        .json(&json!({
            "contents": [{ "parts": [{ "text": prompt }] }]
        }))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let text = resp["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("Could not generate an explanation right now. Please try again.")
        .to_string();

    Ok(text)
}
