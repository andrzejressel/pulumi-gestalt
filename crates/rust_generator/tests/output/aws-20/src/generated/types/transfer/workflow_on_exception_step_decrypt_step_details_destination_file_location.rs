#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocation {
    /// Specifies the details for the EFS file being copied.
    #[builder(into)]
    #[serde(rename = "efsFileLocation")]
    pub r#efs_file_location: Option<Box<super::super::types::transfer::WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation>>,
    /// Specifies the details for the S3 file being copied.
    #[builder(into)]
    #[serde(rename = "s3FileLocation")]
    pub r#s_3_file_location: Option<Box<super::super::types::transfer::WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocation {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("efs_file_location".to_string(), self.r#efs_file_location.to_pulumi_value().await);
            map.insert("s_3_file_location".to_string(), self.r#s_3_file_location.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocation {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#efs_file_location: {
                        let field_value = match fields_map.get("efs_file_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'efs_file_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::transfer::WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_file_location: {
                        let field_value = match fields_map.get("s_3_file_location") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_file_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::transfer::WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
