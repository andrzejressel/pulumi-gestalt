#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationResponse {
    /// Availability information of the product system.
    #[builder(into)]
    #[serde(rename = "availabilityInformation")]
    pub r#availability_information: Box<super::types::AvailabilityInformationResponse>,
    /// Cost information for the product system.
    #[builder(into)]
    #[serde(rename = "costInformation")]
    pub r#cost_information: Box<super::types::CostInformationResponse>,
    /// Description related to the product system.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<super::types::DescriptionResponse>,
    /// Dimensions of the configuration
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<super::types::DimensionsResponse>,
    /// Display Name for the product system.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: String,
    /// list of filters supported for a product
    #[builder(into)]
    #[serde(rename = "filterableProperties")]
    pub r#filterable_properties: Vec<super::types::FilterablePropertyResponse>,
    /// Hierarchy information of a product.
    #[builder(into)]
    #[serde(rename = "hierarchyInformation")]
    pub r#hierarchy_information: Box<super::types::HierarchyInformationResponse>,
    /// Image information for the product system.
    #[builder(into)]
    #[serde(rename = "imageInformation")]
    pub r#image_information: Vec<super::types::ImageInformationResponse>,
    /// Specifications of the configuration
    #[builder(into)]
    #[serde(rename = "specifications")]
    pub r#specifications: Vec<super::types::SpecificationResponse>,
}
