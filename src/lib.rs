use anyhow::{bail, Result};
use spin_sdk::http::{IntoResponse, Params, Request, ResponseBuilder, Router};
use spin_sdk::http_component;
use std::fs::File;
use std::io::Read;

/// A simple Spin HTTP component.
#[http_component]
fn handle_encryption_detector(req: Request) -> Result<impl IntoResponse> {
    let mut router = Router::default();
    router.get("/assets/*", is_encrypted);
    Ok(router.handle(req))
}

fn is_encrypted(req: Request, _p: Params) -> Result<impl IntoResponse> {
    let is_password_protected = match req.path() {
        p if is_pdf(p) => is_pdf_password_protected(p),
        p if is_office_doc(p) => is_ooxml_password_protected(p),
        _ => {
            return Ok(ResponseBuilder::new(400)
                .body("Only .pdf, .docx, .pptx and .xlsx supported")
                .build())
        }
    };

    Ok(ResponseBuilder::new(200)
        .header("content-type", "application/json")
        .body(format!(
            "{{ \"is_password_protected\": {} }}",
            is_password_protected?
        ))
        .build())
}

fn is_pdf_password_protected(path: &str) -> Result<bool> {
    let doc = lopdf::Document::load(path)?;
    Ok(doc.is_encrypted())
}

const ZIP_MAGIC: [u8; 4] = [0x50, 0x4B, 0x03, 0x04];
const OLE_MAGIC: [u8; 8] = [0xD0, 0xCF, 0x11, 0xE0, 0xA1, 0xB1, 0x1A, 0xE1];

fn is_ooxml_password_protected(path: &str) -> Result<bool> {
    let mut file = File::open(path)?;

    let mut header = [0u8; 8];
    file.read_exact(&mut header)?;
    if header == OLE_MAGIC {
        return Ok(true);
    }
    if header[..4] == ZIP_MAGIC {
        return Ok(false);
    }
    bail!("Unknown file format");
}

fn is_pdf(path: &str) -> bool {
    path.ends_with(".pdf")
}

fn is_office_doc(path: &str) -> bool {
    path.ends_with(".docx") || path.ends_with(".xlsx") || path.ends_with(".pptx")
}
