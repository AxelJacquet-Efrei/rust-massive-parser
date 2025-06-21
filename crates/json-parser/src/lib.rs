//! Parser JSON haute performance pour petits et très gros fichiers.
//!
//! - Utilise serde_json pour les petits fichiers (chargement complet)
//! - Parsing streaming pour les gros fichiers (JSONL ou incrémental)
//! - API similaire à txt-parser

use parser_core::{ParseError, DocumentParser};
use std::path::Path;
use serde_json::Value;
use memmap2::MmapOptions;
use rayon::prelude::*;
use std::fs::File;
use serde::de::DeserializeOwned;
use std::io::{BufRead, BufReader, Read};

pub struct JsonParser;

pub enum JsonObjectIter {
    Jsonl(Box<dyn Iterator<Item = Result<Value, ParseError>> + Send>),
    Array(Box<dyn Iterator<Item = Result<Value, ParseError>> + Send>),
}

impl Iterator for JsonObjectIter {
    type Item = Result<Value, ParseError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            JsonObjectIter::Jsonl(it) => it.next(),
            JsonObjectIter::Array(it) => it.next(),
        }
    }
}

impl JsonParser {
    /// Parse un fichier JSON (petit ou gros). Pour l'instant, charge tout en mémoire.
    pub fn parse(path: &Path) -> Result<Vec<Value>, ParseError> {
        let data = std::fs::read_to_string(path)?;
        // Tentative de parsing comme JSONL (une valeur JSON par ligne)
        if data.lines().count() > 1 {
            let mut values = Vec::new();
            for line in data.lines() {
                if !line.trim().is_empty() {
                    let v: Value = serde_json::from_str(line)?;
                    values.push(v);
                }
            }
            Ok(values)
        } else {
            // Sinon, parsing classique (JSON unique)
            let v: Value = serde_json::from_str(&data)?;
            Ok(vec![v])
        }
    }
    /// Parse un fichier JSONL (une valeur JSON par ligne) en parallèle, mmap + rayon.
    pub fn parse_jsonl_parallel(path: &Path) -> Result<Vec<Value>, ParseError> {
        let file = File::open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let data = &mmap[..];
        let text = std::str::from_utf8(data)?;
        // Découpe en lignes (sans allocation intermédiaire)
        let lines: Vec<&str> = text.lines().collect();
        let values: Result<Vec<_>, _> = lines
            .par_iter()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str::<Value>(line).map_err(ParseError::from))
            .collect();
        values
    }
    /// Parse un fichier JSON massif (unique objet/array) en streaming (faible RAM).
    pub fn parse_streaming<T: DeserializeOwned>(path: &Path) -> Result<T, ParseError> {
        let file = File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let mut deser = serde_json::Deserializer::from_reader(reader);
        let v = T::deserialize(&mut deser)?;
        Ok(v)
    }
    /// Choix auto du mode selon la taille et le format (JSONL ou massif).
    pub fn parse_auto(path: &Path) -> Result<Vec<Value>, ParseError> {
        let metadata = std::fs::metadata(path)?;
        if metadata.len() < 512 * 1024 * 1024 {
            // < 512 Mo : charge tout en mémoire
            Self::parse(path)
        } else {
            // > 512 Mo : tente JSONL mmap+rayon, sinon streaming
            match Self::parse_jsonl_parallel(path) {
                Ok(v) if !v.is_empty() => Ok(v),
                _ => Ok(vec![Self::parse_streaming::<Value>(path)?]),
            }
        }
    }
    /// Retourne un iterator sur les objets JSON du fichier (JSONL ou tableau).
    pub fn iter_objects(path: &Path) -> Result<JsonObjectIter, ParseError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut first_bytes = [0u8; 16];
        let n = reader.read(&mut first_bytes)?;
        let first = std::str::from_utf8(&first_bytes[..n])?.trim_start();
        if first.starts_with('[') {
            // Streaming sur tableau JSON
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let deser = serde_json::Deserializer::from_reader(reader);
            let iter = deser.into_iter::<Value>().map(|v| v.map_err(ParseError::from));
            Ok(JsonObjectIter::Array(Box::new(iter)))
        } else {
            // JSONL : une ligne = un objet JSON
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            let iter = reader
                .lines()
                .filter_map(|l| match l {
                    Ok(line) if !line.trim().is_empty() => Some(serde_json::from_str::<Value>(&line).map_err(ParseError::from)),
                    _ => None,
                });
            Ok(JsonObjectIter::Jsonl(Box::new(iter)))
        }
    }
}

impl DocumentParser for JsonParser {
    fn parse(path: &Path) -> Result<parser_core::Document, ParseError> {
        let mut values = Self::parse_auto(path)?;
        // Si le fichier est un tableau JSON unique, on découpe chaque élément comme une ligne
        if values.len() == 1 {
            if let Value::Array(arr) = &values[0] {
                values = arr.clone();
            }
        }
        // Sérialise chaque objet en texte (une "ligne" par objet)
        let mut buffer = Vec::new();
        let mut offsets = Vec::with_capacity(values.len());
        let mut pos = 0u32;
        for v in values {
            let s = serde_json::to_string(&v)?;
            let bytes = s.as_bytes();
            let len = bytes.len() as u32;
            buffer.extend_from_slice(bytes);
            buffer.push(b'\n');
            offsets.push((pos, len));
            pos += len + 1;
        }
        Ok(parser_core::Document {
            data: parser_core::DocumentData::Buffer(std::sync::Arc::new(buffer)),
            offsets,
        })
    }
}

impl JsonParser {
    /// API harmonisée : parse et retourne un Document (compatibilité txt-parser)
    pub fn parse_as_document(path: &Path) -> Result<parser_core::Document, ParseError> {
        <JsonParser as DocumentParser>::parse(path)
    }
}
