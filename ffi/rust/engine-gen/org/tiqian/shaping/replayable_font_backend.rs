use crate::org::tiqian::core::tiqian_illegal_argument_exception::TextRangeError;
use crate::org::tiqian::font::font_role::FontRole;
use crate::runtime::sorted_table::SortedMapTable;
use crate::runtime::sorted_table::SortedSetTable;
use crate::runtime::sorted_table::SortedTable;
use crate::runtime::u_string;
use std::sync::Arc;


#[derive(Clone, Copy)]
pub struct FontFaceIdImpl;

impl FontFaceIdImpl {
    pub fn font_face_id_impl_of(value: &str) -> Result<String, TextRangeError> {
        if false || u_string::unit_count(&(value.trim())) == 0 {
            return Err(TextRangeError::Message { text: "FontFaceId must not be blank".to_string() });
        }
        return Ok((value).to_string());
    }

    pub fn font_face_id_impl_from_string(value: &str) -> Result<String, TextRangeError> {
        return Ok(FontFaceIdImpl::font_face_id_impl_of(value)?);
    }

    pub fn font_face_id_impl_to_string(this1: &str) -> String {
        return this1.to_string();
    }
}

#[derive(Clone)]
pub struct ReplayableFontFaceDescriptor {
    pub id: String,
    pub family_aliases: SortedSetTable<String>,
    pub roles: Vec<FontRole>,
    pub weight: u32,
    pub italic: bool,
    pub collection_index: u32,
    pub source_label: String,
    pub variation_axes: SortedMapTable<String, f64>,
}

impl ReplayableFontFaceDescriptor {
    pub fn new(id: &str, family_aliases: SortedSetTable<String>, roles: Vec<FontRole>, source_label: &str, weight: Option<u32>, italic: Option<bool>, collection_index: Option<u32>, variation_axes: Option<SortedMapTable<String, f64>>) -> Result<Self, TextRangeError> {
        let variation_axes = variation_axes.unwrap_or_else(|| SortedTable::sorted_table_map_builder::<String, f64>(Arc::new(|a, b| SortedTable::sorted_table_compare_strings(a.as_str(), b.as_str()))).build());
        Ok(Self {
            id: id.to_string(),
            family_aliases,
            roles,
            source_label: source_label.to_string(),
            weight: weight.unwrap_or_default(),
            italic: italic.unwrap_or_default(),
            collection_index: collection_index.unwrap_or_default(),
            variation_axes: variation_axes,
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct ReplayableFontFaceRequest {
    pub role: FontRole,
    pub preferred_families: Vec<String>,
    pub font_size: f64,
    pub weight: u32,
    pub italic: bool,
    pub locale: String,
    pub selection_text: String,
}

impl ReplayableFontFaceRequest {
    pub fn new(role: FontRole, preferred_families: Vec<String>, font_size: f64, weight: u32, italic: bool, locale: &str, selection_text: &str) -> Result<Self, TextRangeError> {
        if !((font_size) > (0 as f64) && (font_size).is_finite()) {
            return Err(TextRangeError::Message { text: "fontSize must be positive and finite".to_string() });
        }
        Ok(Self {
            role,
            preferred_families,
            font_size,
            weight,
            italic,
            locale: locale.to_string(),
            selection_text: selection_text.to_string(),
        })
    }
}

#[derive(Clone, PartialEq)]
pub struct FontBackendCapabilityIssue {
    pub code: String,
    pub detail: String,
}

impl FontBackendCapabilityIssue {
    pub fn new(code: &str, detail: &str) -> Result<Self, TextRangeError> {
        Ok(Self {
            code: code.to_string(),
            detail: detail.to_string(),
        })
    }
}

#[derive(Clone)]
pub struct FontBackendCapabilityReport {
    pub backend: String,
    pub source_kind: String,
    pub faces: Vec<ReplayableFontFaceDescriptor>,
    pub issues: Vec<FontBackendCapabilityIssue>,
}

impl FontBackendCapabilityReport {
    pub fn new(backend: &str, source_kind: &str, faces: Vec<ReplayableFontFaceDescriptor>, issues: Option<Vec<FontBackendCapabilityIssue>>) -> Result<Self, TextRangeError> {
        let issues = issues.unwrap_or_else(|| vec![]);
        Ok(Self {
            backend: backend.to_string(),
            source_kind: source_kind.to_string(),
            faces,
            issues: issues,
        })
    }

    pub fn get_can_replay_from_controlled_bytes(&self) -> bool {
        if u32::try_from(((self.faces).clone().len()) & 0xFFFF_FFFF).unwrap_or(0) == 0 {
            return false;
        }
        {
            let _g1 = (self.issues).clone();
            for issue in &_g1 {
                if issue.code.to_string() == "MissingControlledFontFace" {
                    return false;
                }
            }
        }
        return true;
    }
}

pub trait ReplayableFontCatalog: Send + Sync {
    fn __haxe_type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_box(&self) -> Box<dyn ReplayableFontCatalog>;
    fn get_faces(&self) -> Vec<ReplayableFontFaceDescriptor>;
    fn get_capability_report(&self) -> Result<FontBackendCapabilityReport, TextRangeError>;
    fn resolve(&self, request: ReplayableFontFaceRequest) -> Option<ReplayableFontFaceDescriptor>;
}

impl Clone for Box<dyn ReplayableFontCatalog> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl std::fmt::Debug for dyn ReplayableFontCatalog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__haxe_type_name())
    }
}
