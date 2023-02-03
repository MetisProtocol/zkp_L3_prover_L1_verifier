use regex::Regex;

use crate::constants::{TEXT_SECTION_MIN_ADDRESS, WORD};
use crate::line::Line;
use crate::section::Section;

pub struct Label {
    name: String,
    pub address: i32,
}

impl Label {
    pub fn new(name: &str, address: i32) -> Self {
        Self {
            name: name.to_string(),
            address,
        }
    }
}

pub fn find_label<'a>(name: &'a str, labels: &'a [Label]) -> Option<&'a Label> {
    labels.iter().find(|label| label.name == name)
}

pub fn is_label(code: &str) -> bool {
    let label_regex = Regex::new(r"^.*:").unwrap();
    label_regex.is_match(code)
}

pub fn resolve_labels(code: &str) -> Option<Label> {
    let label_regex = Regex::new(r"^.*:").unwrap();
    let _label1_regex = label_regex.clone(); 
    /*
    if let Some(cap) = &label1_regex.captures_iter(&code).next() {
        let name = cap[0].trim_end_matches(':');
        Some(Label::new(name, 0))
    } else {
        None
    }
    */ 
    
    let x = if let Some(cap) = _label1_regex.captures_iter(&code).next() {
          let name = cap[0].trim_end_matches(':');
 
          Some(Label::new(name, 0))
       //   Some(None)
      } else 
      { Some(Label::new("None",0))  };
      x 
      
}

pub fn get_addressed_labels(lines: &[Line], codes: &[String]) -> Vec<Label> {
    let mut current_address = TEXT_SECTION_MIN_ADDRESS;
    let labels = extract_labels_from_lines(lines);

    codes
        .iter()
        .filter_map(|code| {
            if let Some(label) = resolve_labels(&code) {
                if let Some(label) = find_label(&label.name, &labels) {
                    Some(Label::new(&label.name, current_address))
                } else {
                    panic!("Use of undeclared label.");
                }
            } else {
                current_address += WORD;
                None
            }
        })
        .collect()
}

fn extract_labels_from_lines(lines: &[Line]) -> Vec<Label> {
    lines
        .iter()
        .filter(|line| line.section == Section::TEXT)
        .map(|line| {
            if let Some(label) = resolve_labels(&line.text.as_ref().unwrap()) {
                Some(label)
            } else {
                None
            }
        })
        .filter_map(|label| label)
        .collect()
}
