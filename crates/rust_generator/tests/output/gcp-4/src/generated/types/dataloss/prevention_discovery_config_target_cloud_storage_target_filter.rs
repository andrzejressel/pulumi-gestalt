#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigTargetCloudStorageTargetFilter {
    /// The bucket to scan. Targets including this can only include one target (the target with this bucket). This enables profiling the contents of a single bucket, while the other options allow for easy profiling of many buckets within a project or an organization.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cloudStorageResourceReference")]
    pub r#cloud_storage_resource_reference: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterCloudStorageResourceReference>>,
    /// A specific set of buckets for this filter to apply to.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "collection")]
    pub r#collection: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterCollection>>,
    /// Match discovery resources not covered by any other filter.
    #[builder(into)]
    #[serde(rename = "others")]
    pub r#others: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigTargetCloudStorageTargetFilterOthers>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigTargetCloudStorageTargetFilter {
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
                    "cloud_storage_resource_reference",
                    &self.r#cloud_storage_resource_reference,
                ),
                to_pulumi_object_field(
                    "collection",
                    &self.r#collection,
                ),
                to_pulumi_object_field(
                    "others",
                    &self.r#others,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigTargetCloudStorageTargetFilter {
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
                    r#cloud_storage_resource_reference: {
                        let field_value = match fields_map.get("cloud_storage_resource_reference") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_storage_resource_reference' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collection: {
                        let field_value = match fields_map.get("collection") {
                            Some(value) => value,
                            None => bail!("Missing field 'collection' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#others: {
                        let field_value = match fields_map.get("others") {
                            Some(value) => value,
                            None => bail!("Missing field 'others' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
