use serde::Deserialize;

#[derive(Deserialize)]
struct DockerHubTag {
    name: String,
}

#[derive(Deserialize)]
struct DockerHubTagsResponse {
    results: Vec<DockerHubTag>,
}

fn normalize_version(tag: &str) -> Option<String> {
    let dot_count = tag.chars().filter(|&c| c == '.').count();
    if dot_count == 0 || dot_count > 2 {
        return None;
    }

    if !tag
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.')
    {
        return None;
    }

    let valid_segments = tag.split('.').all(|segment| !segment.is_empty() && segment.chars().all(|c| c.is_ascii_digit()));
    if !valid_segments {
        return None;
    }

    Some(tag.to_string())
}

fn compare_versions_desc(a: &str, b: &str) -> std::cmp::Ordering {
    let pa: Vec<u32> = a.split('.').map(|x| x.parse::<u32>().unwrap_or(0)).collect();
    let pb: Vec<u32> = b.split('.').map(|x| x.parse::<u32>().unwrap_or(0)).collect();
    let size = pa.len().max(pb.len());

    for i in 0..size {
        let av = *pa.get(i).unwrap_or(&0);
        let bv = *pb.get(i).unwrap_or(&0);
        match bv.cmp(&av) {
            std::cmp::Ordering::Equal => continue,
            ord => return ord,
        }
    }

    std::cmp::Ordering::Equal
}

async fn fetch_docker_versions(image: &str) -> Result<Vec<String>, String> {
    let url = format!("https://hub.docker.com/v2/repositories/library/{}/tags?page_size=100", image);

    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Error de red: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("Error HTTP: {}", response.status()));
    }

    let payload: DockerHubTagsResponse = response
        .json()
        .await
        .map_err(|e| format!("Error parseando respuesta: {e}"))?;

    let mut versions: Vec<String> = payload
        .results
        .into_iter()
        .filter_map(|item| normalize_version(&item.name))
        .collect();

    versions.sort_by(|a, b| compare_versions_desc(a, b));
    versions.dedup();

    Ok(versions)
}

#[tauri::command]
async fn fetch_php_versions() -> Result<Vec<String>, String> {
    fetch_docker_versions("php").await
}

#[tauri::command]
async fn fetch_apache_versions() -> Result<Vec<String>, String> {
    fetch_docker_versions("httpd").await
}

#[tauri::command]
async fn fetch_mysql_versions() -> Result<Vec<String>, String> {
    fetch_docker_versions("mysql").await
}

#[tauri::command]
async fn fetch_redis_versions() -> Result<Vec<String>, String> {
    fetch_docker_versions("redis").await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
        fetch_php_versions,
        fetch_apache_versions,
        fetch_mysql_versions,
        fetch_redis_versions
    ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
