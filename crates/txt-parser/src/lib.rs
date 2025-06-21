//! Parser texte haute performance pour gros fichiers, usage batch ou serveur web.
//!
//! - Zéro-copy via mmap ou buffer mémoire (upload, flux)
//! - Accès rapide aux lignes (offsets pré-calculés)
//! - API thread-safe (Arc)
//! - Validation UTF-8 optionnelle (sécurité vs performance)
//! - Pagination efficace (extraction de range de lignes)
//! - Mode "safe" pour l'accès aux lignes (vérification UTF-8 à la volée)
//!
//! Limites :
//! - L'indexation des offsets consomme ~8 octets par ligne (RAM)
//! - Pour des fichiers >100M lignes, prévoir un mode streaming ou index partiel
//! - Les accès rapides supposent des offsets corrects (attention si le fichier est corrompu)
//!
//! Voir aussi : parser_core::Document, parser_core::DocumentParser

use memchr::memchr_iter;
use memmap2::MmapOptions;
use rayon::prelude::*;
use std::{fs::File, path::Path, sync::Arc};
use parser_core::{Document, DocumentParser, ParseError, DocumentData};

pub struct TxtParser;

impl TxtParser {
    /// Parse un fichier texte avec option de validation UTF-8 complète.
    pub fn parse_with_validation(path: &Path, validate_utf8: bool) -> Result<Document, ParseError> {
        // 1) Memory-map en lecture seule
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let data = &mmap[..];
        let len = data.len();

        // 2) Conseille le kernel pour un readahead séquentiel
        #[cfg(unix)]
        unsafe {
            libc::posix_madvise(
                data.as_ptr() as *mut _,
                len as libc::size_t,
                libc::POSIX_MADV_SEQUENTIAL,
            );
        }

        // 3) Découpe en chunks de 64 MiB
        const CHUNK_SIZE: usize = 64 * 1024 * 1024;
        let boundaries: Vec<(usize, usize)> = (0..len)
            .step_by(CHUNK_SIZE)
            .map(|start| (start, (start + CHUNK_SIZE).min(len)))
            .collect();

        // 4) Scan parallèle, collecte offsets (memchr_iter)
        let sets: Vec<Vec<(u32, u32)>> = boundaries
            .into_par_iter()
            .map(|(s, e)| {
                let slice = &data[s..e];
                let mut local = Vec::with_capacity(slice.len() / 40 + 1); // estimation plus large
                let mut prev = 0;
                for nl in memchr_iter(b'\n', slice) {
                    local.push(((s + prev) as u32, (nl - prev) as u32));
                    prev = nl + 1;
                }
                if prev < slice.len() {
                    local.push(((s + prev) as u32, (slice.len() - prev) as u32));
                }
                local
            })
            .collect();

        // 5) Concaténation efficace
        let total: usize = sets.iter().map(Vec::len).sum();
        let mut offsets = Vec::with_capacity(total);
        for v in sets {
            offsets.extend(v);
        }

        // 6) Validation UTF-8
        if validate_utf8 {
            for &(start, len) in &offsets {
                let slice = &data[start as usize .. (start + len) as usize];
                std::str::from_utf8(slice)?;
            }
        } else {
            // Vérification explicite UTF-8 sur la première ligne (robustesse)
            if let Some(&(start, len)) = offsets.get(0) {
                let slice = &data[start as usize .. (start + len) as usize];
                std::str::from_utf8(slice)?;
            }
        }

        Ok(Document {
            data: DocumentData::Mmap(Arc::new(mmap)),
            offsets,
        })
    }

    /// Parse un buffer en mémoire (slice de bytes), avec option de validation UTF-8.
    pub fn parse_buffer(data: &[u8], validate_utf8: bool) -> Result<Document, ParseError> {
        let len = data.len();
        // 1) Découpe en chunks de 64 MiB
        const CHUNK_SIZE: usize = 64 * 1024 * 1024;
        let boundaries: Vec<(usize, usize)> = (0..len)
            .step_by(CHUNK_SIZE)
            .map(|start| (start, (start + CHUNK_SIZE).min(len)))
            .collect();
        // 2) Scan parallèle, collecte offsets (memchr_iter)
        let sets: Vec<Vec<(u32, u32)>> = boundaries
            .into_par_iter()
            .map(|(s, e)| {
                let slice = &data[s..e];
                let mut local = Vec::with_capacity(slice.len() / 40 + 1);
                let mut prev = 0;
                for nl in memchr_iter(b'\n', slice) {
                    local.push(((s + prev) as u32, (nl - prev) as u32));
                    prev = nl + 1;
                }
                if prev < slice.len() {
                    local.push(((s + prev) as u32, (slice.len() - prev) as u32));
                }
                local
            })
            .collect();
        // 3) Concaténation efficace
        let total: usize = sets.iter().map(Vec::len).sum();
        let mut offsets = Vec::with_capacity(total);
        for v in sets {
            offsets.extend(v);
        }
        // 4) Validation UTF-8
        if validate_utf8 {
            for &(start, len) in &offsets {
                let slice = &data[start as usize .. (start + len) as usize];
                std::str::from_utf8(slice)?;
            }
        } else {
            if let Some(&(start, len)) = offsets.get(0) {
                let slice = &data[start as usize .. (start + len) as usize];
                std::str::from_utf8(slice)?;
            }
        }
        // 5) Copie le buffer dans un Arc<Vec<u8>> pour respecter l'API Document
        let arc_data = Arc::new(data.to_vec());
        Ok(Document {
            data: DocumentData::Buffer(arc_data),
            offsets,
        })
    }

    /// Parse un fichier texte en indexant seulement 1 ligne sur `stride` (index partiel).
    pub fn parse_with_partial_index(path: &Path, stride: usize, validate_utf8: bool) -> Result<Document, ParseError> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let data = &mmap[..];
        let len = data.len();
        #[cfg(unix)]
        unsafe {
            libc::posix_madvise(
                data.as_ptr() as *mut _,
                len as libc::size_t,
                libc::POSIX_MADV_SEQUENTIAL,
            );
        }
        const CHUNK_SIZE: usize = 64 * 1024 * 1024;
        let boundaries: Vec<(usize, usize)> = (0..len)
            .step_by(CHUNK_SIZE)
            .map(|start| (start, (start + CHUNK_SIZE).min(len)))
            .collect();
        let sets: Vec<Vec<(u32, u32)>> = boundaries
            .into_par_iter()
            .map(|(s, e)| {
                let slice = &data[s..e];
                let mut local = Vec::with_capacity(slice.len() / 40 + 1);
                let mut prev = 0;
                let mut line_idx = 0;
                for nl in memchr_iter(b'\n', slice) {
                    if line_idx % stride == 0 {
                        local.push(((s + prev) as u32, (nl - prev) as u32));
                    }
                    prev = nl + 1;
                    line_idx += 1;
                }
                if prev < slice.len() && (line_idx % stride == 0) {
                    local.push(((s + prev) as u32, (slice.len() - prev) as u32));
                }
                local
            })
            .collect();
        let total: usize = sets.iter().map(Vec::len).sum();
        let mut offsets = Vec::with_capacity(total);
        for v in sets {
            offsets.extend(v);
        }
        if validate_utf8 {
            for &(start, len) in &offsets {
                let slice = &data[start as usize .. (start + len) as usize];
                std::str::from_utf8(slice)?;
            }
        } else {
            if let Some(&(start, len)) = offsets.get(0) {
                let slice = &data[start as usize .. (start + len) as usize];
                std::str::from_utf8(slice)?;
            }
        }
        Ok(Document {
            data: DocumentData::Mmap(Arc::new(mmap)),
            offsets,
        })
    }
}

impl DocumentParser for TxtParser {
    fn parse(path: &Path) -> Result<Document, ParseError> {
        Self::parse_with_validation(path, false)
    }
}

// (Suppression du mod tests ici, les tests d'intégration sont déplacés dans tests/integration.rs)
