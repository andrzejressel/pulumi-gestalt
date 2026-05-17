#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionStoredInfoTypeLargeCustomDictionary {
    /// Field in a BigQuery table where each cell represents a dictionary phrase.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "bigQueryField")]
    pub r#big_query_field: Option<Box<super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionaryBigQueryField>>,
    /// Set of files containing newline-delimited lists of dictionary phrases.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorageFileSet")]
    pub r#cloud_storage_file_set: Option<Box<super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionaryCloudStorageFileSet>>,
    /// Location to store dictionary artifacts in Google Cloud Storage. These files will only be accessible by project owners and the DLP API.
    /// If any of these artifacts are modified, the dictionary is considered invalid and can no longer be used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "outputPath")]
    pub r#output_path: Box<super::super::types::dataloss::PreventionStoredInfoTypeLargeCustomDictionaryOutputPath>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionStoredInfoTypeLargeCustomDictionary {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "big_query_field",
                    &self.r#big_query_field,
                ),
                to_pulumi_object_field(
                    "cloud_storage_file_set",
                    &self.r#cloud_storage_file_set,
                ),
                to_pulumi_object_field(
                    "output_path",
                    &self.r#output_path,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionStoredInfoTypeLargeCustomDictionary {
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
                    r#big_query_field: {
                        let field_value = match fields_map.get("big_query_field") {
                            Some(value) => value,
                            None => bail!("Missing field 'big_query_field' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cloud_storage_file_set: {
                        let field_value = match fields_map.get("cloud_storage_file_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_storage_file_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_path: {
                        let field_value = match fields_map.get("output_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
