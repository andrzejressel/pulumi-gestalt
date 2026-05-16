#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataRepositoryAssociationS3 {
    /// Specifies the type of updated objects that will be automatically exported from your file system to the linked S3 bucket. See the `events` configuration block.
    #[builder(into)]
    #[serde(rename = "autoExportPolicy")]
    pub r#auto_export_policy: Option<Box<super::super::types::fsx::DataRepositoryAssociationS3AutoExportPolicy>>,
    /// Specifies the type of updated objects that will be automatically imported from the linked S3 bucket to your file system. See the `events` configuration block.
    #[builder(into)]
    #[serde(rename = "autoImportPolicy")]
    pub r#auto_import_policy: Option<Box<super::super::types::fsx::DataRepositoryAssociationS3AutoImportPolicy>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataRepositoryAssociationS3 {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("auto_export_policy".to_string(), self.r#auto_export_policy.to_pulumi_value().await);
            map.insert("auto_import_policy".to_string(), self.r#auto_import_policy.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataRepositoryAssociationS3 {
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
                    r#auto_export_policy: {
                        let field_value = match fields_map.get("auto_export_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_export_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::fsx::DataRepositoryAssociationS3AutoExportPolicy>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#auto_import_policy: {
                        let field_value = match fields_map.get("auto_import_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_import_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::fsx::DataRepositoryAssociationS3AutoImportPolicy>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
