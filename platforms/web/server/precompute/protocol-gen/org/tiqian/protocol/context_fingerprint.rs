use crate::org::tiqian::protocol::canonical::Canonical;
use crate::org::tiqian::protocol::face_info::FaceInfo;
use crate::org::tiqian::protocol::revision::Revision;
use crate::org::tiqian::protocol::snapshot_typography::SnapshotTypography;
use crate::runtime::bytes_buffer::BytesBuffer;
use crate::runtime::fp_helper::FPHelper;
use crate::runtime::u_string::UStr;
use crate::runtime::u_string::UString;


#[derive(Clone, Copy)]
pub struct ContextFingerprint;

impl ContextFingerprint {
    pub fn context_fingerprint_context_fingerprint(typography: SnapshotTypography, faces: &Vec<FaceInfo>, harfbuzz_version: &UStr) -> Vec<u8> {
        let mut writer = FingerprintWriter::new();
        writer.u32(1, Revision::REVISION_SNAPSHOT_SCHEMA);
        writer.str(2, Revision::REVISION_LAYOUT_REVISION.to_ustring().as_ustr());
        writer.str(3, Revision::REVISION_RENDER_REVISION.to_ustring().as_ustr());
        writer.str(4, Revision::REVISION_FONT_BACKEND_REVISION.to_ustring().as_ustr());
        writer.str(5, Revision::REVISION_FONT_REPLAY_REVISION.to_ustring().as_ustr());
        writer.str(6, Revision::REVISION_FONT_REPLAY_TRANSPORT.to_ustring().as_ustr());
        writer.str(7, Revision::REVISION_FONT_SOURCE_POLICY.to_ustring().as_ustr());
        writer.str(8, harfbuzz_version);
        writer.u32(9, 1);
        writer.tag(16);
        writer.bytes_str_list(&typography.font_families);
        writer.f64(17, typography.font_size_px);
        writer.f64(18, typography.line_height_px);
        writer.str(19, (typography.locale).to_ustring().as_ustr());
        writer.i64(20, typography.font_weight);
        writer.bool(21, typography.italic);
        writer.f64(22, typography.first_line_indent_ic);
        writer.bool(23, typography.line_length_grid_enabled);
        writer.f64(24, typography.letter_spacing_px);
        writer.str(25, (typography.font_feature_settings).to_ustring().as_ustr());
        writer.str(26, (typography.font_variation_settings).to_ustring().as_ustr());
        writer.str(27, (typography.font_variant_numeric).to_ustring().as_ustr());
        writer.u32(28, u32::try_from((faces.len()) & 0xFFFF_FFFF).unwrap_or(0));
        for face in faces {
            ContextFingerprint::context_fingerprint_encode_face(&mut writer, (face).clone());
        }
        return writer.finish();
    }

    pub(crate) fn context_fingerprint_encode_face(writer: &mut FingerprintWriter, face: FaceInfo) {
        writer.str(32, (face.family).to_ustring().as_ustr());
        writer.str(33, (face.style).to_ustring().as_ustr());
        writer.f64(34, face.weight[0usize]);
        writer.f64(35, face.weight[1usize]);
        writer.str(36, (face.unicode_range).to_ustring().as_ustr());
        writer.str(37, (face.public_url).to_ustring().as_ustr());
        writer.str(38, (face.source_sha256).to_ustring().as_ustr());
        writer.str(39, (face.sfnt_sha256).to_ustring().as_ustr());
        writer.f64(40, face.face_index);
        writer.u32(41, face.source_order);
        writer.tag(42);
        writer.bytes_str_list(&face.axis_tags);
        writer.tag(43);
        writer.bytes_str_list(&face.local_names);
    }
}

pub struct FingerprintWriter {
    pub(crate) buf: BytesBuffer,
}

impl FingerprintWriter {
    pub fn new() -> Self {
        Self {
            buf: BytesBuffer::new(),
        }
    }

    pub fn tag(&mut self, tag: u32) {
        self.buf.add_byte(u8::try_from((u32::from(tag.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
    }

    pub fn str(&mut self, tag: u32, value: &UStr) {
        self.tag(tag);
        let encoded = value.as_bytes().to_vec();
        self.emit_bits(u32::try_from((encoded.len()) & 0xFFFF_FFFF).unwrap_or(0));
        self.buf.add(&encoded);
    }

    pub fn u32(&mut self, tag: u32, value: u32) {
        self.tag(tag);
        self.emit_bits(value);
    }

    pub fn i64(&mut self, tag: u32, value: u32) {
        self.tag(tag);
        self.emit_bits(value);
        self.emit_bits(0);
    }

    pub fn f64(&mut self, tag: u32, value: f64) {
        self.tag(tag);
        let normalized = if !(value).is_finite() || value == 0.0f64 { 0.0f64 } else { value };
        let bits = FPHelper::double_to_i64(normalized);
        self.emit_bits(bits.low);
        self.emit_bits(bits.high);
    }

    pub fn bool(&mut self, tag: u32, value: bool) {
        self.tag(tag);
        self.buf.add_byte(u8::try_from((if value { 1 } else { 0 }) & 0xFF).unwrap_or(0));
    }

    pub fn bytes_str_list(&mut self, values: &Vec<UString>) {
        self.emit_bits(u32::try_from((values.len()) & 0xFFFF_FFFF).unwrap_or(0));
        for value in values {
            let encoded = value.as_bytes().to_vec();
            self.emit_bits(u32::try_from((encoded.len()) & 0xFFFF_FFFF).unwrap_or(0));
            self.buf.add(&encoded);
        }
    }

    pub fn finish(&mut self) -> Vec<u8> {
        return Canonical::canonical_digest(&self.buf.get_bytes());
    }

    fn emit_bits(&mut self, value: u32) {
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[3])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[2])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[1])) & 0xFF).unwrap_or(0));
        self.buf.add_byte(u8::try_from((u32::from(value.to_be_bytes()[0])) & 0xFF).unwrap_or(0));
    }
}
