#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EntryGcsFilesetSpec {
    /// Patterns to identify a set of files in Google Cloud Storage.
    /// See [Cloud Storage documentation](https://cloud.google.com/storage/docs/gsutil/addlhelp/WildcardNames)
    /// for more information. Note that bucket wildcards are currently not supported. Examples of valid filePatterns:
    /// * gs://bucket_name/dir/*: matches all files within bucket_name/dir directory.
    /// * gs://bucket_name/dir/**: matches all files in bucket_name/dir spanning all subdirectories.
    /// * gs://bucket_name/file*: matches files prefixed by file in bucket_name
    /// * gs://bucket_name/??.txt: matches files with two characters followed by .txt in bucket_name
    /// * gs://bucket_name/[aeiou].txt: matches files that contain a single vowel character followed by .txt in bucket_name
    /// * gs://bucket_name/[a-m].txt: matches files that contain a, b, ... or m followed by .txt in bucket_name
    /// * gs://bucket_name/a/*/b: matches all files in bucket_name that match a/*/b pattern, such as a/c/b, a/d/b
    /// * gs://another_bucket/a.txt: matches gs://another_bucket/a.txt
    #[builder(into)]
    #[serde(rename = "filePatterns")]
    pub r#file_patterns: Vec<String>,
    /// (Output)
    /// Sample files contained in this fileset, not all files contained in this fileset are represented here.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_sample_gcs_file_specs"></a>The `sample_gcs_file_specs` block contains:
    #[builder(into)]
    #[serde(rename = "sampleGcsFileSpecs")]
    pub r#sample_gcs_file_specs: Option<Vec<super::super::types::datacatalog::EntryGcsFilesetSpecSampleGcsFileSpec>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EntryGcsFilesetSpec {
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
                    "file_patterns",
                    &self.r#file_patterns,
                ),
                to_pulumi_object_field(
                    "sample_gcs_file_specs",
                    &self.r#sample_gcs_file_specs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EntryGcsFilesetSpec {
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
                    r#file_patterns: {
                        let field_value = match fields_map.get("file_patterns") {
                            Some(value) => value,
                            None => bail!("Missing field 'file_patterns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sample_gcs_file_specs: {
                        let field_value = match fields_map.get("sample_gcs_file_specs") {
                            Some(value) => value,
                            None => bail!("Missing field 'sample_gcs_file_specs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
