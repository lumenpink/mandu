//! Ponte com o motor de renderização Joaju (SVG / PDF).
//! Nota: A crate joaju está em desenvolvimento ativo. Este módulo expõe um stub inicial.

#[derive(Debug, Clone)]
pub struct JoajuStub;

impl JoajuStub {
    #[must_use]
    pub fn render_block_svg(engine: &str, content: &str) -> String {
        let len = content.len();
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"100\">\
                <rect width=\"100%\" height=\"100%\" fill=\"#222222\"/>\
                <text x=\"20\" y=\"50\" fill=\"#ffffff\" font-family=\"monospace\">\
                    Joaju Stub [{engine}]: {len} bytes\
                </text>\
            </svg>"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_joaju_stub_render() {
        let svg = JoajuStub::render_block_svg("heredograma", "dados");
        assert!(svg.contains("Joaju Stub [heredograma]: 5 bytes"));
    }
}
