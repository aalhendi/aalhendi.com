use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const PROFILES: [&str; 2] = ["resume", "highlights"];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("resume export failed: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let requested = env::args().nth(1).unwrap_or_else(|| "resume".to_owned());
    let profiles: Vec<&str> = if requested == "all" {
        PROFILES.to_vec()
    } else if PROFILES.contains(&requested.as_str()) {
        vec![
            PROFILES
                .iter()
                .copied()
                .find(|profile| *profile == requested)
                .expect("validated profile"),
        ]
    } else {
        return Err(format!("unknown profile '{requested}'; use resume, highlights or all").into());
    };

    let repository = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("could not locate the repository root")?
        .canonicalize()?;
    let resume = repository.join("about.html").canonicalize()?;
    let assets = repository.join("assets");
    fs::create_dir_all(&assets)?;

    let browser = find_browser()?;
    println!("Using {}", browser.display());

    for profile in profiles {
        export_profile(&browser, &resume, &assets, profile)?;
    }

    Ok(())
}

fn export_profile(
    browser: &Path,
    resume: &Path,
    assets: &Path,
    profile: &str,
) -> Result<(), Box<dyn Error>> {
    let output_name = match profile {
        "resume" => "aalhendi_cv.pdf",
        "highlights" => "aalhendi_highlights.pdf",
        _ => unreachable!("profile is validated before export"),
    };
    let output = assets.join(output_name);
    let temporary_output = assets.join(format!(".{output_name}.tmp.pdf"));
    if temporary_output.exists() {
        fs::remove_file(&temporary_output)?;
    }

    let temp_root = env::temp_dir().canonicalize()?;
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let user_data = temp_root.join(format!(
        "aalhendi-resume-export-{}-{profile}-{nonce}",
        std::process::id()
    ));
    if user_data.parent() != Some(temp_root.as_path()) {
        return Err("unsafe browser temporary directory".into());
    }
    fs::create_dir(&user_data)?;

    let resume_url = format!("{}?view={profile}", file_url(resume));
    let preflight = Command::new(browser)
        .arg("--headless=new")
        .arg("--disable-extensions")
        .arg("--allow-file-access-from-files")
        .arg("--timeout=5000")
        .arg(format!("--user-data-dir={}", user_data.display()))
        .arg("--dump-dom")
        .arg(&resume_url)
        .output()?;
    let rendered_dom = String::from_utf8_lossy(&preflight.stdout);
    if !preflight.status.success()
        || !rendered_dom.contains("id=\"resume\"")
        || !rendered_dom.contains("Abdulrazzaq Alhendi")
        || !rendered_dom.contains(&format!("data-resume-view=\"{profile}\""))
    {
        let _ = fs::remove_dir_all(&user_data);
        let stderr = String::from_utf8_lossy(&preflight.stderr);
        return Err(format!("browser could not load the {profile} resume: {stderr}").into());
    }

    let result = Command::new(browser)
        .arg("--headless=new")
        .arg("--disable-extensions")
        .arg("--allow-file-access-from-files")
        .arg("--hide-scrollbars")
        .arg("--no-pdf-header-footer")
        .arg("--timeout=5000")
        .arg(format!("--user-data-dir={}", user_data.display()))
        .arg(format!("--print-to-pdf={}", temporary_output.display()))
        .arg(resume_url)
        .output();

    let _ = fs::remove_dir_all(&user_data);
    let browser_output = result?;
    if !browser_output.status.success() {
        let stderr = String::from_utf8_lossy(&browser_output.stderr);
        return Err(format!("browser exited with {}: {stderr}", browser_output.status).into());
    }

    for _ in 0..40 {
        if temporary_output.is_file() {
            break;
        }
        thread::sleep(Duration::from_millis(50));
    }

    let metadata = fs::metadata(&temporary_output).map_err(|_| {
        format!(
            "the browser reported success but did not create {}",
            temporary_output.display()
        )
    })?;
    if metadata.len() < 10_000 {
        return Err(format!(
            "generated PDF is unexpectedly small ({} bytes)",
            metadata.len()
        )
        .into());
    }

    fs::copy(&temporary_output, &output)?;
    fs::remove_file(&temporary_output)?;
    println!("Generated {} ({profile})", output.display());
    Ok(())
}

fn find_browser() -> Result<PathBuf, Box<dyn Error>> {
    let mut candidates = Vec::new();

    if let Some(configured) = env::var_os("RESUME_BROWSER") {
        candidates.push(PathBuf::from(configured));
    }

    if cfg!(target_os = "windows") {
        for variable in ["PROGRAMFILES", "PROGRAMFILES(X86)", "LOCALAPPDATA"] {
            if let Some(root) = env::var_os(variable) {
                let root = PathBuf::from(root);
                candidates.push(root.join("Microsoft/Edge/Application/msedge.exe"));
                candidates.push(root.join("Google/Chrome/Application/chrome.exe"));
            }
        }
        candidates.extend([PathBuf::from("msedge"), PathBuf::from("chrome")]);
    } else if cfg!(target_os = "macos") {
        candidates.extend([
            PathBuf::from("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"),
            PathBuf::from("/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"),
        ]);
    } else {
        candidates.extend([
            PathBuf::from("google-chrome"),
            PathBuf::from("microsoft-edge"),
            PathBuf::from("chromium"),
            PathBuf::from("chromium-browser"),
        ]);
    }

    for candidate in candidates {
        if candidate.is_file() || Command::new(&candidate).arg("--version").output().is_ok() {
            return Ok(candidate);
        }
    }

    Err("could not find Edge, Chrome or Chromium; set RESUME_BROWSER to its executable".into())
}

fn file_url(path: &Path) -> String {
    let raw = path.to_string_lossy().replace('\\', "/");
    let raw = raw.strip_prefix("//?/").unwrap_or(&raw).to_owned();
    let prefixed = if raw.starts_with('/') {
        raw
    } else {
        format!("/{raw}")
    };
    format!("file://{}", percent_encode_path(&prefixed))
}

fn percent_encode_path(path: &str) -> String {
    let mut encoded = String::with_capacity(path.len());
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b':' | b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}
