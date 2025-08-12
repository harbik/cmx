
use crate::{is_zero, round_to_precision, signatures::Signature, tag::RenderingIntent};

#[derive(serde::Serialize)]
pub struct IccHeaderToml {
    profile_size: u32,
    cmm: String,
    version: String,
    device_class: String,
    color_space: String,
    pcs: String,
    creation_datetime: String,
    primary_platform: String,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    embedded: bool,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    use_embedded_only: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    manufacturer: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    model: String,
    #[serde(skip_serializing_if = "is_zero")]
    attributes: u64,
    rendering_intent: String,
    pcs_illuminant: (f64, f64, f64),
    #[serde(skip_serializing_if = "String::is_empty")]
    creator: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    profile_id: String,
}

impl From<&super::RawProfile> for IccHeaderToml {
    fn from(raw_profile: &super::RawProfile) -> Self {
        let header = raw_profile.header();
        let (major, minor) = raw_profile.version().unwrap();
        let version= format!("{major}.{minor}");
        let (embedded, use_embedded_only) = raw_profile.flags();
        let profile_id_raw = raw_profile.profile_id();
        let profile_id = if profile_id_raw.iter().all(is_zero) {
            String::new()
        } else {
            hex::encode(profile_id_raw)
        };

        let model = if header.model == 0 {
            String::new()
        } else {
            Signature(header.model.get()).to_string()
        };

        let manufacturer = if header.manufacturer == 0 {
            String::new()
        } else {
            Signature(header.manufacturer.get()).to_string()
        };

        IccHeaderToml {
            profile_size: raw_profile.profile_size() as u32,
            cmm: raw_profile.cmm().to_string(),
            version,
            device_class: raw_profile.device_class().to_string(),
            color_space: raw_profile.data_color_space().to_string(),
            pcs: raw_profile.pcs().unwrap().to_string(),
            creation_datetime: raw_profile.creation_date().to_string(),
            primary_platform: raw_profile.primary_platform().to_string(),
            embedded,
            use_embedded_only,
            manufacturer,
            model,
            attributes: header.attributes.get(),
            rendering_intent: RenderingIntent::from(header.rendering_intent.get()).to_string(),
            pcs_illuminant: (
                round_to_precision(f64::from(header.pcs_illuminant[0]), 4),
                round_to_precision(f64::from(header.pcs_illuminant[1]), 4),
                round_to_precision(f64::from(header.pcs_illuminant[2]), 4),
            ),
            creator: Signature(header.creator.get()).to_string(),
            profile_id,
        }
    }
}
