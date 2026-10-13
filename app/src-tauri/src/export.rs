//! Read-only export snapshots and atomic artifact output. No case-data writes.
use crate::case::{CaseSnapshot, CaseStore};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;
#[derive(Serialize)]
pub struct ExportContext {
    pub snapshot: CaseSnapshot,
    pub signature: String,
}
fn signature(snapshot: &CaseSnapshot) -> Result<String, String> {
    let bytes = serde_json::to_vec(snapshot).map_err(|e| e.to_string())?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
fn validate_payload(format: &str, bytes: &[u8]) -> Result<(), String> {
    if bytes.is_empty() || bytes.len() > 100 * 1024 * 1024 {
        return Err("Export must contain between 1 byte and 100 MiB".into());
    }
    let valid = match format {
        "png" => bytes.starts_with(b"\x89PNG\r\n\x1a\n"),
        "pdf" => bytes.starts_with(b"%PDF-"),
        "docx" => bytes.starts_with(b"PK\x03\x04"),
        "svg" => std::str::from_utf8(bytes).is_ok_and(|text| {
            text.starts_with("<?xml") && text.contains("<svg ") && text.ends_with("</svg>")
        }),
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err("Export format and payload do not match".into())
    }
}
impl CaseStore {
    pub fn export_context(&self) -> Result<ExportContext, String> {
        let snapshot = self.snapshot()?.ok_or("Open a case before exporting")?;
        let signature = signature(&snapshot)?;
        Ok(ExportContext {
            snapshot,
            signature,
        })
    }
    pub fn write_export(
        &self,
        path: &Path,
        format: &str,
        bytes: &[u8],
        expected_signature: &str,
    ) -> Result<(), String> {
        let context = self.export_context()?;
        if context.signature != expected_signature {
            return Err("The case changed while exporting. Retry with the current case.".into());
        }
        if !path.is_absolute()
            || path
                .extension()
                .and_then(|s| s.to_str())
                .map(str::to_ascii_lowercase)
                .as_deref()
                != Some(format)
        {
            return Err("Choose an absolute destination with the selected export extension".into());
        }
        validate_payload(format, bytes)?;
        let parent = path.parent().ok_or("Choose an export directory")?;
        let mut output = NamedTempFile::new_in(parent).map_err(|e| e.to_string())?;
        output.write_all(bytes).map_err(|e| e.to_string())?;
        output.as_file().sync_all().map_err(|e| e.to_string())?;
        output.persist(path).map_err(|e| e.to_string())?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn export_signature_is_stable_and_rejects_stale_case() {
        let mut s = CaseStore::default();
        let root = s.create("Report", "Workshop").unwrap().tree[0].id.clone();
        let first = s.export_context().unwrap();
        assert_eq!(first.signature, s.export_context().unwrap().signature);
        s.add_group(&root, "Changed").unwrap();
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("diagram.svg");
        assert!(s
            .write_export(
                &path,
                "svg",
                b"<?xml?><svg xmlns=\"http://www.w3.org/2000/svg\"></svg>",
                &first.signature
            )
            .unwrap_err()
            .contains("changed"));
        assert!(!path.exists());
    }
    #[test]
    fn exports_do_not_mutate_case_and_replace_atomically() {
        let mut s = CaseStore::default();
        s.create("Report", "Workshop").unwrap();
        let context = s.export_context().unwrap();
        let before = serde_json::to_value(s.snapshot().unwrap()).unwrap();
        let folder = tempfile::tempdir().unwrap();
        let path = folder.path().join("diagram.svg");
        let bytes = b"<?xml version=\"1.0\"?><svg xmlns=\"http://www.w3.org/2000/svg\"></svg>";
        s.write_export(&path, "svg", bytes, &context.signature)
            .unwrap();
        s.write_export(&path, "svg", bytes, &context.signature)
            .unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
        assert_eq!(before, serde_json::to_value(s.snapshot().unwrap()).unwrap());
        assert!(s
            .write_export(
                &folder.path().join("case.sqlite"),
                "svg",
                bytes,
                &context.signature
            )
            .is_err());
        assert!(s
            .write_export(&path, "svg", b"bad", &context.signature)
            .is_err());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
    #[test]
    fn closed_case_and_invalid_payloads_are_rejected() {
        let s = CaseStore::default();
        assert!(s.export_context().is_err());
        for f in ["png", "pdf", "svg", "docx", "sqlite"] {
            assert!(validate_payload(f, b"bad").is_err());
        }
        assert!(validate_payload("png", b"\x89PNG\r\n\x1a\ndata").is_ok());
    }
}
