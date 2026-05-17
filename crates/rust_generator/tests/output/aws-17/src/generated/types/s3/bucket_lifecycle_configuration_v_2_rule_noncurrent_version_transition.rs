#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition {
    /// Number of noncurrent versions Amazon S3 will retain. Must be a non-zero positive integer.
    #[builder(into)]
    #[serde(rename = "newerNoncurrentVersions")]
    pub r#newer_noncurrent_versions: Option<String>,
    /// Number of days an object is noncurrent before Amazon S3 can perform the associated action.
    #[builder(into)]
    #[serde(rename = "noncurrentDays")]
    pub r#noncurrent_days: Option<i32>,
    /// Class of storage used to store the object. Valid Values: `GLACIER`, `STANDARD_IA`, `ONEZONE_IA`, `INTELLIGENT_TIERING`, `DEEP_ARCHIVE`, `GLACIER_IR`.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "newer_noncurrent_versions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#newer_noncurrent_versions,
                )
                .await,
            );
            map.insert(
                "noncurrent_days".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#noncurrent_days,
                )
                .await,
            );
            map.insert(
                "storage_class".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_class,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition {
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
                    r#newer_noncurrent_versions: {
                        let field_value = match fields_map.get("newer_noncurrent_versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'newer_noncurrent_versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#noncurrent_days: {
                        let field_value = match fields_map.get("noncurrent_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'noncurrent_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_class: {
                        let field_value = match fields_map.get("storage_class") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_class' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
