use reqwest;
use std::fs;
use std::io::Write;

pub fn recompile(force: bool) -> Result<String, Box<dyn std::error::Error>> {
    let language_stubs = [
        "zed-industries/zed/main/crates/languages/src/typescript",
        "zed-industries/zed/main/extensions/html/languages/html",
    ];
    let schemas = [
        "brackets.scm",
        "config.toml",
        "embedding.scm",
        "highlights.scm",
        "indents.scm",
        "injections.scm",
        "outline.scm",
        "overrides.scm",
        "runnables.scm",
        "textobjects.scm",
    ];

    for language_stub in &language_stubs {
        let parts: Vec<&str> = language_stub.split('/').collect();
        let language_name = parts.last().unwrap();

        if !force && std::path::Path::new(&format!("languages/{}", language_name)).exists() {
            continue;
        }

        let parts: Vec<&str> = language_stub.split('/').collect();
        let language_name = parts.last().unwrap();

        for schema in &schemas {
            let url = format!(
                "https://raw.githubusercontent.com/{}/{}",
                language_stub, schema
            );
            let mut response = reqwest::blocking::get(&url)?.text()?;

            if response.contains("404") {
                continue;
            }

            match *schema {
                "config.toml" => {
                    // let collection: Vec<&str> = response.lines().collect();

                    // let lines = collection
                    //     .into_iter()
                    //     .map(|line| {
                    //         if line.starts_with("name = ") {
                    //             return format!(
                    //                 "{} (Zedder Comments)\"\n",
                    //                 &line[..line.len() - 1]
                    //             );
                    //         }
                    //         return line.to_string();
                    //     })
                    //     .collect::<Vec<String>>();

                    // response = lines.join("\n");
                }
                "highlights.scm" => {
                    let highlights = "((comment) @keyword\n\
                   (#match? @keyword \"MARK\"))\n\n\
                  ((comment) @comment\n\
                   (#not-match? @comment \"MARK\"))\n";
                    response = format!("{}\n{}", response, highlights);
                }
                _ => {}
            }

            let dir_path = format!("languages/{}", language_name);
            fs::create_dir_all(&dir_path)?;

            let file_path = format!("{}/{}", dir_path, schema);
            let mut file = fs::File::create(&file_path)?;
            file.write_all(response.as_bytes())?;
        }
    }

    Ok("Schemas downloaded and stored successfully.".to_string())
}
