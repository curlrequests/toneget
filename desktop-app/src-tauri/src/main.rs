#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::io::Write;

const AUTH0_DOMAIN: &str = "tonal.auth0.com";
const CLIENT_ID: &str = "ERCyexW-xoVG_Yy3RDe-eV4xsOnRHP6L";
const API_BASE: &str = "https://api.tonal.com";

// Fields to remove from each set (dashboard doesn't use these)
const SET_FIELDS_TO_REMOVE: &[&str] = &[
    "beginTimeMCB", "endTimeMCB",
    "romWeightMode", "romWeight", "romWeightFrac",
    "isoModeSpeed", "dualMotorReps", "suggestedResistanceLevel",
    "offMachineModifiedWeight", "userWeightPounds", "meanMaxPos",
    "velAtMaxConPower", "weightAtMaxConPower",
    "inchesUpdated", "powerUpdated",
    "triggeredFeedback", "reps",
    "workoutActivityID", "workoutId", "userId", "setId",
];

// Fields to remove from workouts
const WORKOUT_FIELDS_TO_REMOVE: &[&str] = &["deletedAt"];

// Fields to remove from user/profile
const USER_FIELDS_TO_REMOVE: &[&str] = &[
    "recentMobileDevice", "auth0Id", "isGuestAccount", "isDemoAccount",
    "watchedSafetyVideo", "social", "profileAssetID", "mobileWorkoutsEnabled",
    "accountType", "sharingCustomWorkoutsDisabled", "workoutDurationMin",
    "workoutDurationMax", "updatedPreferencesAt", "primaryDeviceType",
    "emailVerified", "workoutsPerWeek",
];

// Known Tonal workout types (not custom)
const KNOWN_WORKOUT_TYPES: &[&str] = &["PROGRAM", "ON_DEMAND", "QUICK_FIT", "LIVE", "MOVEMENT", "ASSESSMENT"];

// ============================================================================
// Data structures for API responses
// ============================================================================

#[derive(Serialize)]
pub struct AuthResult {
    success: bool,
    id_token: Option<String>,
    user_id: Option<String>,
    user_name: Option<String>,
    error: Option<String>,
}

#[derive(Serialize)]
pub struct DownloadProgress {
    current: i32,
    total: i32,
    stage: String,
}

#[derive(Serialize)]
pub struct DownloadResult {
    success: bool,
    json_data: Option<String>,
    gzip_data: Option<Vec<u8>>,
    stats: Option<ExportStats>,
    error: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct ExportStats {
    workouts: usize,
    custom_workouts: usize,
    total_volume: i64,
    json_size: usize,
    gzip_size: usize,
    compression_ratio: f32,
}

#[derive(Deserialize)]
struct Auth0Response {
    id_token: Option<String>,
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Deserialize)]
struct UserInfo {
    id: Option<String>,
    #[serde(rename = "firstName")]
    first_name: Option<String>,
    #[serde(rename = "lastName")]
    last_name: Option<String>,
}

// ============================================================================
// Data trimming functions
// ============================================================================

fn trim_object(obj: &Value, fields_to_remove: &[&str]) -> Value {
    match obj {
        Value::Object(map) => {
            let filtered: Map<String, Value> = map
                .iter()
                .filter(|(k, _)| !fields_to_remove.contains(&k.as_str()))
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            Value::Object(filtered)
        }
        _ => obj.clone(),
    }
}

fn trim_set(set: &Value) -> Value {
    trim_object(set, SET_FIELDS_TO_REMOVE)
}

fn trim_workout(workout: &Value) -> Value {
    let mut trimmed = trim_object(workout, WORKOUT_FIELDS_TO_REMOVE);
    
    if let Some(obj) = trimmed.as_object_mut() {
        if let Some(Value::Array(sets)) = obj.get("workoutSetActivity") {
            let trimmed_sets: Vec<Value> = sets.iter().map(trim_set).collect();
            obj.insert("workoutSetActivity".to_string(), Value::Array(trimmed_sets));
        }
    }
    
    trimmed
}

fn trim_user_data(user: &Value) -> Value {
    trim_object(user, USER_FIELDS_TO_REMOVE)
}

fn compress_json(json_str: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(json_str.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

/// Parse current strength scores into both raw and parsed format
fn parse_current_strength_scores(raw_data: &Value) -> Value {
    if let Value::Array(regions) = raw_data {
        if regions.is_empty() {
            return json!({});
        }
        
        let mut region_scores: Map<String, Value> = Map::new();
        let mut muscle_scores: Map<String, Value> = Map::new();
        
        for region in regions {
            let region_name = region.get("strengthBodyRegion")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            let region_score = region.get("score")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0);
            
            region_scores.insert(region_name.to_string(), json!(region_score));
            
            // Extract individual muscle scores
            if let Some(Value::Array(families)) = region.get("familyActivity") {
                for muscle in families {
                    let muscle_name = muscle.get("strengthFamily")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    let muscle_score = muscle.get("score")
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let updated_at = muscle.get("updatedAt").cloned();
                    
                    muscle_scores.insert(muscle_name.to_string(), json!({
                        "score": muscle_score.round() as i64,
                        "region": region_name,
                        "updatedAt": updated_at
                    }));
                }
            }
        }
        
        json!({
            "raw": raw_data,
            "parsed": {
                "regions": region_scores,
                "muscles": muscle_scores
            }
        })
    } else {
        json!({})
    }
}

// ============================================================================
// Tauri commands
// ============================================================================

#[tauri::command]
async fn authenticate(email: String, password: String) -> AuthResult {
    let client = reqwest::Client::new();
    
    let mut body = HashMap::new();
    body.insert("grant_type", "password");
    body.insert("client_id", CLIENT_ID);
    body.insert("username", &email);
    body.insert("password", &password);
    body.insert("scope", "openid profile email offline_access");
    
    let auth_response = match client
        .post(format!("https://{}/oauth/token", AUTH0_DOMAIN))
        .json(&body)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return AuthResult {
                success: false,
                id_token: None,
                user_id: None,
                user_name: None,
                error: Some(format!("Connection failed: {}", e)),
            }
        }
    };
    
    let status = auth_response.status();
    let auth_data: Auth0Response = match auth_response.json().await {
        Ok(data) => data,
        Err(e) => {
            return AuthResult {
                success: false,
                id_token: None,
                user_id: None,
                user_name: None,
                error: Some(format!("Failed to parse response: {}", e)),
            }
        }
    };
    
    if !status.is_success() {
        let error_msg = auth_data.error_description
            .or(auth_data.error)
            .unwrap_or_else(|| "Invalid email or password".to_string());
        return AuthResult {
            success: false,
            id_token: None,
            user_id: None,
            user_name: None,
            error: Some(error_msg),
        };
    }
    
    let id_token = match auth_data.id_token {
        Some(token) => token,
        None => {
            return AuthResult {
                success: false,
                id_token: None,
                user_id: None,
                user_name: None,
                error: Some("No token received".to_string()),
            }
        }
    };
    
    let user_response = match client
        .get(format!("{}/v6/users/userinfo", API_BASE))
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return AuthResult {
                success: false,
                id_token: None,
                user_id: None,
                user_name: None,
                error: Some(format!("Failed to get user info: {}", e)),
            }
        }
    };
    
    let user_info: UserInfo = match user_response.json().await {
        Ok(info) => info,
        Err(e) => {
            return AuthResult {
                success: false,
                id_token: None,
                user_id: None,
                user_name: None,
                error: Some(format!("Failed to parse user info: {}", e)),
            }
        }
    };
    
    let user_name = match (&user_info.first_name, &user_info.last_name) {
        (Some(first), Some(last)) => Some(format!("{} {}", first, last)),
        (Some(first), None) => Some(first.clone()),
        _ => None,
    };
    
    AuthResult {
        success: true,
        id_token: Some(id_token),
        user_id: user_info.id,
        user_name,
        error: None,
    }
}

/// Fetch a single workout template by ID
async fn fetch_workout_template(client: &reqwest::Client, id_token: &str, workout_id: &str) -> Option<Value> {
    let response = client
        .get(format!("{}/v6/workouts/{}", API_BASE, workout_id))
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
        .ok()?;
    
    if response.status().is_success() {
        response.json().await.ok()
    } else {
        None
    }
}

#[tauri::command]
async fn download_workouts(id_token: String, user_id: String) -> DownloadResult {
    let client = reqwest::Client::new();
    
    // Get user info
    let user_info: Value = match client
        .get(format!("{}/v6/users/userinfo", API_BASE))
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
    {
        Ok(resp) => resp.json().await.unwrap_or(json!({})),
        Err(_) => json!({}),
    };
    
    // Get user profile
    let profile: Value = match client
        .get(format!("{}/v6/users/{}/profile", API_BASE, user_id))
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
    {
        Ok(resp) => resp.json().await.unwrap_or(json!({})),
        Err(_) => json!({}),
    };
    
    // Download all workouts with pagination
    let mut all_workouts: Vec<Value> = Vec::new();
    let mut offset = 0;
    let limit = 100;
    
    loop {
        let response = match client
            .get(format!("{}/v6/users/{}/workout-activities", API_BASE, user_id))
            .header("Authorization", format!("Bearer {}", id_token))
            .header("pg-offset", offset.to_string())
            .header("pg-limit", limit.to_string())
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return DownloadResult {
                    success: false,
                    json_data: None,
                    gzip_data: None,
                    stats: None,
                    error: Some(format!("Failed to fetch workouts: {}", e)),
                }
            }
        };
        
        let total: i32 = response
            .headers()
            .get("pg-total")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);
        
        let batch: Vec<Value> = match response.json().await {
            Ok(data) => data,
            Err(e) => {
                return DownloadResult {
                    success: false,
                    json_data: None,
                    gzip_data: None,
                    stats: None,
                    error: Some(format!("Failed to parse workouts: {}", e)),
                }
            }
        };
        
        if batch.is_empty() {
            break;
        }
        
        all_workouts.extend(batch);
        offset += limit;
        
        if offset >= total {
            break;
        }
    }
    
    // Sort by date (newest first)
    all_workouts.sort_by(|a, b| {
        let date_a = a.get("beginTime").and_then(|v| v.as_str()).unwrap_or("");
        let date_b = b.get("beginTime").and_then(|v| v.as_str()).unwrap_or("");
        date_b.cmp(date_a)
    });
    
    // Calculate total volume
    let total_volume: i64 = all_workouts
        .iter()
        .filter_map(|w| w.get("totalVolume").and_then(|v| v.as_i64()))
        .sum();
    
    // Find custom workout IDs that need fetching
    let mut custom_ids: HashSet<String> = HashSet::new();
    for workout in &all_workouts {
        let workout_type = workout.get("workoutType")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let template_id = workout.get("workoutId")
            .and_then(|v| v.as_str());
        
        if let Some(id) = template_id {
            // It's custom if type is "Custom" or not in known types
            if workout_type == "Custom" || !KNOWN_WORKOUT_TYPES.contains(&workout_type) {
                custom_ids.insert(id.to_string());
            }
        }
    }
    
    // Fetch custom workout templates
    let mut custom_workouts: Map<String, Value> = Map::new();
    for workout_id in &custom_ids {
        if let Some(details) = fetch_workout_template(&client, &id_token, workout_id).await {
            let entry = json!({
                "id": details.get("id"),
                "title": details.get("title"),
                "userId": details.get("userId")
            });
            custom_workouts.insert(workout_id.clone(), entry);
        }
        // Small delay to avoid rate limiting
        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }
    
    // Fetch strength score history
    let strength_history: Vec<Value> = match client
        .get(format!("{}/v6/users/{}/strength-scores/history", API_BASE, user_id))
        .header("Authorization", format!("Bearer {}", id_token))
        .query(&[("limit", "5000"), ("endDate", &chrono::Utc::now().format("%Y-%m-%d").to_string())])
        .send()
        .await
    {
        Ok(resp) => resp.json().await.unwrap_or_default(),
        Err(_) => vec![],
    };
    
    // Fetch current strength scores (raw)
    let current_strength_raw: Value = match client
        .get(format!("{}/v6/users/{}/strength-scores/current", API_BASE, user_id))
        .header("Authorization", format!("Bearer {}", id_token))
        .send()
        .await
    {
        Ok(resp) => resp.json().await.unwrap_or(json!({})),
        Err(_) => json!({}),
    };
    
    // Parse current strength scores into both raw and parsed format
    let current_strength_scores = parse_current_strength_scores(&current_strength_raw);
    
    // Apply trimming
    let trimmed_workouts: Vec<Value> = all_workouts.iter().map(trim_workout).collect();
    let trimmed_user = trim_user_data(&user_info);
    let trimmed_profile = trim_user_data(&profile);
    
    // Build export data
    let export_data = json!({
        "version": "3.0",
        "exportedAt": chrono::Utc::now().to_rfc3339(),
        "exportedWith": "ToneGet v2.0.0",
        "user": trimmed_user,
        "profile": trimmed_profile,
        "workouts": trimmed_workouts,
        "customWorkouts": custom_workouts,
        "strengthScoreHistory": strength_history,
        "currentStrengthScores": current_strength_scores,
    });
    
    // Serialize to compact JSON
    let json_str = serde_json::to_string(&export_data).unwrap_or_default();
    let json_size = json_str.len();
    
    // Compress with gzip
    let gzip_bytes = compress_json(&json_str);
    let gzip_size = gzip_bytes.len();
    
    let compression_ratio = if json_size > 0 {
        (1.0 - (gzip_size as f32 / json_size as f32)) * 100.0
    } else {
        0.0
    };
    
    let stats = ExportStats {
        workouts: trimmed_workouts.len(),
        custom_workouts: custom_workouts.len(),
        total_volume,
        json_size,
        gzip_size,
        compression_ratio,
    };
    
    DownloadResult {
        success: true,
        json_data: Some(json_str),
        gzip_data: Some(gzip_bytes),
        stats: Some(stats),
        error: None,
    }
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![authenticate, download_workouts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
