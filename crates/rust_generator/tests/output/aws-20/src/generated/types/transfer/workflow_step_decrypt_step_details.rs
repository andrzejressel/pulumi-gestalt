#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowStepDecryptStepDetails {
    /// Specifies the location for the file being copied. Use ${Transfer:username} in this field to parametrize the destination prefix by username.
    #[builder(into)]
    #[serde(rename = "destinationFileLocation")]
    pub r#destination_file_location: Box<Option<super::super::types::transfer::WorkflowStepDecryptStepDetailsDestinationFileLocation>>,
    /// The name of the step, used as an identifier.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// A flag that indicates whether or not to overwrite an existing file of the same name. The default is `FALSE`. Valid values are `TRUE` and `FALSE`.
    #[builder(into)]
    #[serde(rename = "overwriteExisting")]
    pub r#overwrite_existing: Option<String>,
    /// Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
    #[builder(into)]
    #[serde(rename = "sourceFileLocation")]
    pub r#source_file_location: Option<String>,
    /// The type of encryption used. Currently, this value must be `"PGP"`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
