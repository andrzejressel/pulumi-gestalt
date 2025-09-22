#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableMagneticStoreWriteProperties {
    /// A flag to enable magnetic store writes.
    #[builder(into)]
    #[serde(rename = "enableMagneticStoreWrites")]
    pub r#enable_magnetic_store_writes: Option<bool>,
    /// The location to write error reports for records rejected asynchronously during magnetic store writes. See Magnetic Store Rejected Data Location below for more details.
    #[builder(into)]
    #[serde(rename = "magneticStoreRejectedDataLocation")]
    pub r#magnetic_store_rejected_data_location: Option<Box<super::super::types::timestreamwrite::TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation>>,
}
