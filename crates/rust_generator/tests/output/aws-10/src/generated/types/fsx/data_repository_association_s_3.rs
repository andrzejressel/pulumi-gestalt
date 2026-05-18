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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "auto_export_policy",
                    &self.r#auto_export_policy,
                ),
                to_pulumi_object_field(
                    "auto_import_policy",
                    &self.r#auto_import_policy,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataRepositoryAssociationS3 {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#auto_export_policy: {
                        let field_value = match fields_map.get("auto_export_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_export_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auto_import_policy: {
                        let field_value = match fields_map.get("auto_import_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'auto_import_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
