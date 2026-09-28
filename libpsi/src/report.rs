use std::borrow::Cow;

use crate::{module_cache::ModuleCache, source::SourceSpan};

pub struct Report {
    pub severity: ReportSeverity,
    pub message: Cow<'static, str>,
    pub span: Option<SourceSpan>,
    pub notes: Box<[Cow<'static, str>]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportSeverity {
    Error,
    Warning,
    Note,
}

pub trait IntoReport {
    fn into_report(self) -> Report;
}

impl ReportSeverity {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "Error",
            Self::Warning => "Warning",
            Self::Note => "Note",
        }
    }
}

impl Report {
    pub fn render(&self, modules: &ModuleCache) -> String {
        let mut output = String::new();

        output.push_str(self.severity.as_str());
        output.push_str(": ");
        output.push_str(&self.message);

        if let Some(span) = &self.span {
            Self::render_label(&mut output, modules, span);
        }

        for note in &self.notes {
            output.push_str("\n| ");
            output.push_str("\n| note: ");
            output.push_str(note);
            output.push('\n');
        }

        return output;
    }

    fn render_label(output: &mut String, modules: &ModuleCache, span: &SourceSpan) {
        let Some(module) = modules.get(span.module_index()) else {
            return;
        };

        let start = span.start_pos();

        output.push_str("\n| --> ");
        output.push_str(&module.filename);
        output.push(':');
        output.push_str(&start.line().to_string());
        output.push(':');
        output.push_str(&start.column().to_string());

        output.push_str("\n|\n| ");
        output.push_str(
            module
                .source
                .lines()
                .nth(start.line().get() as usize - 1)
                .unwrap(),
        );

        output.push_str("\n| ");
        output.push_str(&" ".repeat(start.column().get() as usize - 1));
        output.push_str("^ here");
    }
}
