use memchr::memchr_iter;
use memmap2::MmapOptions;
use parser_core::{Document, DocumentData, DocumentParser, ParseError};
use rayon::prelude::*;
use std::{fs::File, path::Path, sync::Arc};

pub struct TxtParser;

impl DocumentParser for TxtParser {
    fn parse(path: &Path) -> Result<Document, ParseError> {
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
                    // On retire le \n du calcul de longueur
                    let line_end = if nl > 0 && slice[nl - 1] == b'\r' {
                        nl - 1
                    } else {
                        nl
                    };
                    local.push(((s + prev) as u32, (line_end - prev) as u32));
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

        // 6) Vérification explicite UTF-8 sur toutes les lignes (robustesse)
        for &(start, len) in &offsets {
            let slice = &data[start as usize..(start + len) as usize];
            std::str::from_utf8(slice)?;
        }

        Ok(Document {
            data: DocumentData::Mmap(Arc::new(mmap)),
            offsets,
        })
    }
}

// (Suppression du mod tests ici, les tests d'intégration sont déplacés dans tests/integration.rs)
