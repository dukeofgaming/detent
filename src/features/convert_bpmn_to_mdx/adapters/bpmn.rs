//! BPMN 2.0 parsing and serialization
//!
//! This module provides types and utilities for working with BPMN 2.0 documents.

pub mod types;

pub use types::*;

use quick_xml::de::from_str;
use quick_xml::se::to_string;
use regex::Regex;
use std::sync::LazyLock;

/// Parse a BPMN XML string into Definitions
pub fn parse_bpmn(xml: &str) -> Result<Definitions, quick_xml::DeError> {
    let stripped = strip_unsupported(xml);
    from_str(&stripped)
}

/// Serialize Definitions to BPMN XML string
pub fn serialize_bpmn(definitions: &Definitions) -> Result<String, quick_xml::SeError> {
    let raw = to_string(definitions)?;
    Ok(apply_bpmn_namespaces(&raw))
}

fn apply_bpmn_namespaces(xml: &str) -> String {
    static NS_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"<definitions "#).unwrap()
    });
    static BPMN_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(</?)(definitions|process|startEvent|endEvent|task|manualTask|userTask|serviceTask|scriptTask|exclusiveGateway|parallelGateway|sequenceFlow|outgoing|incoming|sequenceFlow|conditionExpression|documentation|extensionElements|laneSet|lane)\b").unwrap()
    });
    static DI_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(</?)(BPMNDiagram|BPMNPlane|BPMNShape|BPMNEdge|BPMNLabel)\b").unwrap()
    });
    static DC_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(</?)(Bounds)\b").unwrap()
    });
    static DI_WP_RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(</?)(waypoint)\b").unwrap()
    });

    let bpmn_ns = concat!(
        r#"xmlns:bpmn="http://www.omg.org/spec/BPMN/20100524/MODEL" "#,
        r#"xmlns:bpmndi="http://www.omg.org/spec/BPMN/20100524/DI" "#,
        r#"xmlns:dc="http://www.omg.org/spec/DD/20100524/DC" "#,
        r#"xmlns:di="http://www.omg.org/spec/DD/20100524/DI""#
    );
    let replacement = format!("<bpmn:definitions {} ", bpmn_ns);

    let xml = NS_RE.replace(xml, replacement.as_str()).to_string();
    let xml = BPMN_RE.replace_all(&xml, |caps: &regex::Captures| {
        format!("{}bpmn:{}", &caps[1], &caps[2])
    }).to_string();
    let xml = DI_RE.replace_all(&xml, |caps: &regex::Captures| {
        format!("{}bpmndi:{}", &caps[1], &caps[2])
    }).to_string();
    let xml = DC_RE.replace_all(&xml, |caps: &regex::Captures| {
        format!("{}dc:{}", &caps[1], &caps[2])
    }).to_string();
    let xml = DI_WP_RE.replace_all(&xml, |caps: &regex::Captures| {
        format!("{}di:{}", &caps[1], &caps[2])
    }).to_string();

    format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", xml)
}

fn strip_unsupported(xml: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?s)<(?:bpmn:)?(laneSet|lane|collaboration|participant)\b[^>]*>.*?</(?:bpmn:)?(?:laneSet|lane|collaboration|participant)\s*>").unwrap()
    });
    RE.replace_all(xml, "").to_string()
}

