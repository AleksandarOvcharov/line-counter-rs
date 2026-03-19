use std::path::Path;

#[derive(Debug)]
pub struct ProjectProfile {
    pub name: &'static str,
    pub extensions: Vec<String>,
    pub excludes: Vec<String>,
}

struct ProjectSignature {
    name: &'static str,
    marker_files: &'static [&'static str],
    marker_dirs: &'static [&'static str],
    extensions: &'static [&'static str],
    excludes: &'static [&'static str],
}

static SIGNATURES: &[ProjectSignature] = &[
    // Rust
    ProjectSignature {
        name: "Rust",
        marker_files: &["Cargo.toml"],
        marker_dirs: &[],
        extensions: &["rs", "toml"],
        excludes: &["target"],
    },
    // Node.js / JavaScript
    ProjectSignature {
        name: "Node.js",
        marker_files: &["package.json"],
        marker_dirs: &["node_modules"],
        extensions: &["js", "mjs", "cjs", "json"],
        excludes: &["node_modules", "dist", ".next", "build", "coverage", ".nuxt"],
    },
    // TypeScript
    ProjectSignature {
        name: "TypeScript",
        marker_files: &["tsconfig.json"],
        marker_dirs: &[],
        extensions: &["ts", "tsx", "mts", "cts", "js", "jsx", "json"],
        excludes: &["node_modules", "dist", ".next", "build", "coverage", ".nuxt"],
    },
    // React
    ProjectSignature {
        name: "React",
        marker_files: &["next.config.js", "next.config.mjs", "next.config.ts"],
        marker_dirs: &[".next"],
        extensions: &["tsx", "ts", "jsx", "js", "css", "scss", "json"],
        excludes: &["node_modules", ".next", "dist", "build", "coverage", "out"],
    },
    // Vue
    ProjectSignature {
        name: "Vue",
        marker_files: &["vue.config.js", "nuxt.config.ts", "nuxt.config.js"],
        marker_dirs: &[],
        extensions: &["vue", "ts", "js", "css", "scss", "json"],
        excludes: &["node_modules", "dist", ".nuxt", "build", "coverage"],
    },
    // Svelte
    ProjectSignature {
        name: "Svelte",
        marker_files: &["svelte.config.js"],
        marker_dirs: &[],
        extensions: &["svelte", "ts", "js", "css", "json"],
        excludes: &["node_modules", ".svelte-kit", "dist", "build"],
    },
    // Python
    ProjectSignature {
        name: "Python",
        marker_files: &["pyproject.toml", "setup.py", "setup.cfg", "Pipfile", "requirements.txt"],
        marker_dirs: &[],
        extensions: &["py", "pyi"],
        excludes: &["__pycache__", ".venv", "venv", ".env", "dist", "build", ".eggs", "*.egg-info"],
    },
    // Go
    ProjectSignature {
        name: "Go",
        marker_files: &["go.mod"],
        marker_dirs: &[],
        extensions: &["go"],
        excludes: &["vendor"],
    },
    // Java / Maven
    ProjectSignature {
        name: "Java (Maven)",
        marker_files: &["pom.xml"],
        marker_dirs: &[],
        extensions: &["java", "xml"],
        excludes: &["target", ".gradle", "build"],
    },
    // Java / Gradle
    ProjectSignature {
        name: "Java (Gradle)",
        marker_files: &["build.gradle", "build.gradle.kts"],
        marker_dirs: &[],
        extensions: &["java", "kt", "kts", "xml", "gradle"],
        excludes: &[".gradle", "build", "target"],
    },
    // C# / .NET
    ProjectSignature {
        name: ".NET",
        marker_files: &[],
        marker_dirs: &[],
        extensions: &["cs", "csproj", "sln", "xml"],
        excludes: &["bin", "obj", ".vs"],
    },
    // Ruby
    ProjectSignature {
        name: "Ruby",
        marker_files: &["Gemfile"],
        marker_dirs: &[],
        extensions: &["rb", "erb", "yml"],
        excludes: &["vendor", ".bundle", "tmp", "log"],
    },
    // PHP / Laravel
    ProjectSignature {
        name: "PHP",
        marker_files: &["composer.json"],
        marker_dirs: &[],
        extensions: &["php", "blade.php", "json"],
        excludes: &["vendor", "node_modules", "storage"],
    },
    // Swift
    ProjectSignature {
        name: "Swift",
        marker_files: &["Package.swift"],
        marker_dirs: &[],
        extensions: &["swift"],
        excludes: &[".build", "Pods", "DerivedData"],
    },
    // Dart / Flutter
    ProjectSignature {
        name: "Flutter",
        marker_files: &["pubspec.yaml"],
        marker_dirs: &[],
        extensions: &["dart", "yaml"],
        excludes: &[".dart_tool", "build", ".flutter-plugins"],
    },
    // Elixir
    ProjectSignature {
        name: "Elixir",
        marker_files: &["mix.exs"],
        marker_dirs: &[],
        extensions: &["ex", "exs", "eex", "heex"],
        excludes: &["_build", "deps", ".elixir_ls"],
    },
    // Zig
    ProjectSignature {
        name: "Zig",
        marker_files: &["build.zig"],
        marker_dirs: &[],
        extensions: &["zig"],
        excludes: &["zig-cache", "zig-out"],
    },
    // Haskell
    ProjectSignature {
        name: "Haskell",
        marker_files: &["stack.yaml", "cabal.project"],
        marker_dirs: &[],
        extensions: &["hs", "cabal"],
        excludes: &[".stack-work", "dist-newstyle"],
    },
    // Scala
    ProjectSignature {
        name: "Scala",
        marker_files: &["build.sbt"],
        marker_dirs: &[],
        extensions: &["scala", "sbt"],
        excludes: &["target", ".bsp", ".metals"],
    },
    // C / C++  (CMake)
    ProjectSignature {
        name: "C/C++ (CMake)",
        marker_files: &["CMakeLists.txt"],
        marker_dirs: &[],
        extensions: &["c", "h", "cpp", "cxx", "cc", "hpp", "hxx"],
        excludes: &["build", "cmake-build-debug", "cmake-build-release"],
    },
    // C / C++ (Makefile)
    ProjectSignature {
        name: "C/C++",
        marker_files: &["Makefile"],
        marker_dirs: &[],
        extensions: &["c", "h", "cpp", "cxx", "cc", "hpp", "hxx"],
        excludes: &["build"],
    },
];

/// .sln / .csproj detection needs a glob check
fn has_dotnet_markers(root: &Path) -> bool {
    if let Ok(entries) = std::fs::read_dir(root) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.ends_with(".sln") || name.ends_with(".csproj") {
                return true;
            }
        }
    }
    false
}

pub fn detect_project(root: &Path) -> Option<ProjectProfile> {
    let mut matched: Vec<&ProjectSignature> = Vec::new();

    for sig in SIGNATURES {
        let file_match = sig
            .marker_files
            .iter()
            .any(|f| root.join(f).exists());
        let dir_match = sig
            .marker_dirs
            .iter()
            .any(|d| root.join(d).is_dir());

        if file_match || dir_match {
            matched.push(sig);
        }
    }

    // .NET special case
    if has_dotnet_markers(root) {
        if let Some(dotnet) = SIGNATURES.iter().find(|s| s.name == ".NET") {
            if !matched.iter().any(|s| s.name == ".NET") {
                matched.push(dotnet);
            }
        }
    }

    if matched.is_empty() {
        return None;
    }

    // Merge all matched profiles — a monorepo may have multiple project types
    let mut extensions: Vec<String> = Vec::new();
    let mut excludes: Vec<String> = Vec::new();
    let mut names: Vec<&str> = Vec::new();

    for sig in &matched {
        names.push(sig.name);
        for ext in sig.extensions {
            let e = ext.to_string();
            if !extensions.contains(&e) {
                extensions.push(e);
            }
        }
        for exc in sig.excludes {
            let e = exc.to_string();
            if !excludes.contains(&e) {
                excludes.push(e);
            }
        }
    }

    // Pick the most specific name, or join them
    let name = if names.len() == 1 {
        names[0]
    } else {
        // Prefer more specific over generic (e.g. TypeScript over Node.js)
        // Return the first matched since SIGNATURES is ordered by specificity for overlapping cases
        names[0]
    };

    Some(ProjectProfile {
        name,
        extensions,
        excludes,
    })
}
