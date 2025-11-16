#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelDestination {
    /// User-specified id. Ths is used in an output group or an output.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Destination settings for a MediaPackage output; one destination for both encoders. See Media Package Settings for more details.
    #[builder(into)]
    #[serde(rename = "mediaPackageSettings")]
    pub r#media_package_settings: Option<Vec<super::super::types::medialive::ChannelDestinationMediaPackageSetting>>,
    /// Destination settings for a Multiplex output; one destination for both encoders. See Multiplex Settings for more details.
    #[builder(into)]
    #[serde(rename = "multiplexSettings")]
    pub r#multiplex_settings: Option<Box<super::super::types::medialive::ChannelDestinationMultiplexSettings>>,
    /// Destination settings for a standard output; one destination for each redundant encoder. See Settings for more details.
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Option<Vec<super::super::types::medialive::ChannelDestinationSetting>>,
}
