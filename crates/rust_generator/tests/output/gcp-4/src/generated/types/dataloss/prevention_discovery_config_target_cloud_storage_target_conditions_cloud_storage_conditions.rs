#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetConditionsCloudStorageConditions {
    /// Only objects with the specified attributes will be scanned. Defaults to [ALL_SUPPORTED_BUCKETS] if unset.
    /// Each value may be one of: `ALL_SUPPORTED_BUCKETS`, `AUTOCLASS_DISABLED`, `AUTOCLASS_ENABLED`.
    #[builder(into)]
    #[serde(rename = "includedBucketAttributes")]
    pub r#included_bucket_attributes: Option<Vec<String>>,
    /// Only objects with the specified attributes will be scanned. If an object has one of the specified attributes but is inside an excluded bucket, it will not be scanned. Defaults to [ALL_SUPPORTED_OBJECTS]. A profile will be created even if no objects match the included_object_attributes.
    /// Each value may be one of: `ALL_SUPPORTED_OBJECTS`, `STANDARD`, `NEARLINE`, `COLDLINE`, `ARCHIVE`, `REGIONAL`, `MULTI_REGIONAL`, `DURABLE_REDUCED_AVAILABILITY`.
    #[builder(into)]
    #[serde(rename = "includedObjectAttributes")]
    pub r#included_object_attributes: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTargetCloudStorageTargetConditionsCloudStorageConditions {
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
                    "included_bucket_attributes",
                    &self.r#included_bucket_attributes,
                ),
                to_pulumi_object_field(
                    "included_object_attributes",
                    &self.r#included_object_attributes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigTargetCloudStorageTargetConditionsCloudStorageConditions {
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
                    r#included_bucket_attributes: {
                        let field_value = match fields_map.get("included_bucket_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_bucket_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#included_object_attributes: {
                        let field_value = match fields_map.get("included_object_attributes") {
                            Some(value) => value,
                            None => bail!("Missing field 'included_object_attributes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
