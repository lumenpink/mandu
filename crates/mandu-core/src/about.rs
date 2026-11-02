//! Diagnóstico de versões canônicas dos motores e bibliotecas embutidas do Mandu.

/// Descritor de versão e propósito de um componente interno.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentVersion {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}

/// Retorna a lista dos componentes e bibliotecas embutidas com suas versões SemVer canônicas.
#[must_use]
pub fn get_components_tree() -> Vec<ComponentVersion> {
    vec![
        ComponentVersion {
            name: "Mandu Core",
            version: env!("CARGO_PKG_VERSION"),
            description: "Núcleo de domínio, cofre e indexador SQLite",
        },
        ComponentVersion {
            name: "Motor Joaju",
            version: "0.2.0-dev.137+20261102.e5c6878",
            description: "Compilador tipográfico e orquestrador acadêmico",
        },
        ComponentVersion {
            name: "Typst PDF Engine",
            version: "0.13.1 (embutido via joaju-pdf-static)",
            description: "Motor nativo de renderização e diagramação de documentos",
        },
        ComponentVersion {
            name: "Química (Joaju Chem)",
            version: "0.1.1",
            description: "Módulo de síntese, estruturas moleculares e equações químicas",
        },
        ComponentVersion {
            name: "Pedigree Clínico (Joaju Pedigree)",
            version: "0.2.2",
            description: "Renderizador padronizado de heredogramas e pedigrees médicos",
        },
    ]
}

/// Formata a árvore de versões em texto legível para CLI e TUI.
#[must_use]
pub fn format_about_text() -> String {
    let mut out = String::new();
    out.push_str(&format!("Mandu v{}\n", env!("CARGO_PKG_VERSION")));
    out.push_str("Árvore de versões dos motores e componentes embutidos:\n\n");
    for comp in get_components_tree() {
        out.push_str(&format!(
            "  • {:<32} : v{} ({})\n",
            comp.name, comp.version, comp.description
        ));
    }
    out
}
