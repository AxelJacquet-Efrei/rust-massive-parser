//! Modèle de document et interface pour chaque parser.
//!
//! Garanties :
//! - Zéro-copy (mmap)
//! - Thread-safe (`Arc<Mmap>`)
//! - Accès rapide aux lignes
//! - API ergonomique pour serveurs ou batch

use memmap2::Mmap;
use std::{path::Path, sync::Arc};

/// Backend de données pour Document : mmap ou buffer mémoire.
pub enum DocumentData {
    Mmap(Arc<Mmap>),
    Buffer(Arc<Vec<u8>>),
}

/// Document texte stocké ZERO-COPY via mmap ou buffer + offsets.
pub struct Document {
    /// Garde le backend vivant.
    pub data: DocumentData,
    /// Pour chaque ligne, (offset_en_octets, longueur_en_octets).
    pub offsets: Vec<(u32, u32)>,
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Format error: {0}")]
    Format(String),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Index out of bounds: {0}")]
    Index(usize),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Tout parser de document doit implémenter ce trait.
pub trait DocumentParser {
    fn parse(path: &Path) -> Result<Document, ParseError>;
}

impl Document {
    /// Itérateur sur les lignes en &str, sans re-check UTF-8.
    /// # Safety
    /// Les offsets sont supposés corrects et pointent sur des tranches UTF-8 valides.
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.offsets.iter().map(move |&(start, len)| {
            let slice = match &self.data {
                DocumentData::Mmap(m) => &m[start as usize..(start + len) as usize],
                DocumentData::Buffer(b) => &b[start as usize..(start + len) as usize],
            };
            unsafe { std::str::from_utf8_unchecked(slice) }
        })
    }

    /// Retourne un Vec<&str> correspondant à un range de lignes [start, end).
    pub fn lines_range(&self, start: usize, end: usize) -> Result<Vec<&str>, ParseError> {
        if start > end || end > self.offsets.len() {
            return Err(ParseError::Index(end));
        }
        Ok((start..end)
            .map(|idx| self.get_line(idx).unwrap())
            .collect())
    }

    /// Nombre de lignes dans le document.
    pub fn line_count(&self) -> usize {
        self.offsets.len()
    }

    /// Accès à une ligne précise (avec vérification des bornes).
    pub fn get_line(&self, idx: usize) -> Result<&str, ParseError> {
        if let Some(&(start, len)) = self.offsets.get(idx) {
            let slice = match &self.data {
                DocumentData::Mmap(m) => &m[start as usize..(start + len) as usize],
                DocumentData::Buffer(b) => &b[start as usize..(start + len) as usize],
            };
            // Safety: offsets garantis valides par le parser
            Ok(unsafe { std::str::from_utf8_unchecked(slice) })
        } else {
            Err(ParseError::Index(idx))
        }
    }

    /// Accès à une ligne précise, vérification UTF-8 à la volée (safe).
    pub fn get_line_safe(&self, idx: usize) -> Result<&str, ParseError> {
        if let Some(&(start, len)) = self.offsets.get(idx) {
            let slice = match &self.data {
                DocumentData::Mmap(m) => &m[start as usize..(start + len) as usize],
                DocumentData::Buffer(b) => &b[start as usize..(start + len) as usize],
            };
            std::str::from_utf8(slice).map_err(ParseError::Utf8)
        } else {
            Err(ParseError::Index(idx))
        }
    }

    /// Itérateur streaming (sans indexation préalable, pour très gros fichiers)
    /// Peut être utilisé par un parser alternatif.
    pub fn streaming_lines(data: &[u8]) -> impl Iterator<Item = Result<&str, std::str::Utf8Error>> {
        data.split(|&b| b == b'\n')
            .map(|line| std::str::from_utf8(line))
    }
}
