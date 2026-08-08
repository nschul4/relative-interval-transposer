use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. Get current short Git commit hash
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
        .unwrap_or_else(|_| "unknown".to_string());

    // 2. Cross-platform UTC date/time calculation via SystemTime
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Convert epoch seconds into UTC string components (YYYY-MM-DD HH:MM:SS UTC)
    let days = now / 86400;
    let seconds_into_day = now % 86400;
    let hours = seconds_into_day / 3600;
    let minutes = (seconds_into_day % 3600) / 60;
    let seconds = seconds_into_day % 60;

    let (year, month, day) = days_to_date(days);
    let timestamp = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
        year, month, day, hours, minutes, seconds
    );

    // Pass environment variables to rustc for env!()
    println!("cargo:rustc-env=BUILD_GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);

    // Since the crates are in `subprojects/<name>`, `../../.git` works,
    // but checking `.git` relative to CARGO_MANIFEST_DIR is cleaner:
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
    let git_head = std::path::Path::new(&manifest_dir).join("../../.git/HEAD");
    let git_index = std::path::Path::new(&manifest_dir).join("../../.git/index");

    if git_head.exists() {
        println!("cargo:rerun-if-changed={}", git_head.display());
        println!("cargo:rerun-if-changed={}", git_index.display());
    }
}

fn days_to_date(days_since_epoch: u64) -> (u64, u64, u64) {
    let z = days_since_epoch + 719468;
    let era = z / 146097;
    let doe = z % 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}
