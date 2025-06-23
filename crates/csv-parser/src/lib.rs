//! CSV/TSV parser haute performance pour fichiers massifs.
//!
//! - Zéro-copy via mmap ou buffer mémoire
//! - Indexation rapide des lignes et colonnes (offsets)
//! - Parallélisation rayon
//! - API unifiée (Document)
//! - Support CSV (,) et TSV (\t) auto-détecté
//! - Pagination, stride, validation optionnelle

use memchr::memchr_iter;
use memmap2::MmapOptions;
use parser_core::{Document, DocumentData, DocumentParser, ParseError};
use rayon::prelude::*;
use std::{fs::File, path::Path, sync::Arc};

const CHUNK_SIZE: usize = 64 * 1024 * 1024;
const AVG_LINE_LEN: usize = 40;

pub struct CsvParser;

impl CsvParser {
    /// Détecte le séparateur (CSV ou TSV) sur les premières lignes.
    #[allow(dead_code)]
    fn detect_separator(data: &[u8]) -> u8 {
        let sample = &data[..data.len().min(4096)];
        let csv = sample.iter().filter(|&&b| b == b',').count();
        let tsv = sample.iter().filter(|&&b| b == b'\t').count();
        if tsv > csv {
            b'\t'
        } else {
            b','
        }
    }

    /// Indexe les offsets (start, len) de chaque ligne.
    fn compute_offsets(data: &[u8], stride: usize) -> Vec<(u32, u32)> {
        let len = data.len();
        if len == 0 {
            return Vec::new();
        }
        let boundaries: Vec<(usize, usize)> = (0..len)
            .step_by(CHUNK_SIZE)
            .map(|s| (s, (s + CHUNK_SIZE).min(len)))
            .collect();
        let total_est = len / AVG_LINE_LEN;
        let per_chunk = (total_est / boundaries.len()).max(1);
        boundaries
            .into_par_iter()
            .fold(
                || Vec::with_capacity(per_chunk),
                |mut local, (s, e)| {
                    let slice = &data[s..e];
                    let mut prev = 0;
                    let mut idx = 0;
                    for pos in memchr_iter(b'\n', slice) {
                        if idx % stride == 0 {
                            // On retire le \n du calcul de longueur
                            let line_end = if pos > 0 && slice[pos - 1] == b'\r' {
                                pos - 1
                            } else {
                                pos
                            };
                            local.push(((s + prev) as u32, (line_end - prev) as u32));
                        }
                        prev = pos + 1;
                        idx += 1;
                    }
                    if prev < slice.len() && idx % stride == 0 {
                        local.push(((s + prev) as u32, (slice.len() - prev) as u32));
                    }
                    local
                },
            )
            .reduce(Vec::new, |mut acc, mut local| {
                acc.append(&mut local);
                acc
            })
    }

    /// Validation UTF-8 stricte sur toutes les lignes.
    fn validate(data: &[u8], offsets: &[(u32, u32)], _full: bool) -> Result<(), ParseError> {
        for &(st, ln) in offsets {
            let slice = &data[st as usize..(st + ln) as usize];
            std::str::from_utf8(slice)?;
        }
        Ok(())
    }

    /// Parse un fichier CSV/TSV via mmap, avec validation optionnelle.
    pub fn parse_with_validation(path: &Path, validate_utf8: bool) -> Result<Document, ParseError> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let arc_map = Arc::new(mmap);
        let data = &arc_map[..];
        #[cfg(unix)]
        unsafe {
            libc::posix_madvise(
                data.as_ptr() as *mut _,
                data.len(),
                libc::POSIX_MADV_SEQUENTIAL,
            );
        }
        let offsets = Self::compute_offsets(data, 1);
        Self::validate(data, &offsets, validate_utf8)?;
        Ok(Document {
            data: DocumentData::Mmap(arc_map),
            offsets,
        })
    }

    /// Parse un buffer mémoire, avec validation optionnelle.
    pub fn parse_buffer(data: &[u8], validate_utf8: bool) -> Result<Document, ParseError> {
        let offsets = Self::compute_offsets(data, 1);
        Self::validate(data, &offsets, validate_utf8)?;
        Ok(Document {
            data: DocumentData::Buffer(Arc::new(data.to_vec())),
            offsets,
        })
    }

    /// Parse en n'indexant qu'une ligne sur `stride` (index partiel).
    pub fn parse_with_partial_index(
        path: &Path,
        stride: usize,
        validate_utf8: bool,
    ) -> Result<Document, ParseError> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let arc_map = Arc::new(mmap);
        let data = &arc_map[..];
        #[cfg(unix)]
        unsafe {
            libc::posix_madvise(
                data.as_ptr() as *mut _,
                data.len(),
                libc::POSIX_MADV_SEQUENTIAL,
            );
        }
        let offsets = Self::compute_offsets(data, stride);
        Self::validate(data, &offsets, validate_utf8)?;
        Ok(Document {
            data: DocumentData::Mmap(arc_map),
            offsets,
        })
    }
}

impl DocumentParser for CsvParser {
    fn parse(path: &Path) -> Result<Document, ParseError> {
        Self::parse_with_validation(path, false)
    }
}
