#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicyRuleAction {
    /// A `base_blob` block as documented below.
    #[builder(into)]
    #[serde(rename = "baseBlobs")]
    pub r#base_blobs: Vec<super::super::types::storage::GetPolicyRuleActionBaseBlob>,
    /// A `snapshot` block as documented below.
    #[builder(into)]
    #[serde(rename = "snapshots")]
    pub r#snapshots: Vec<super::super::types::storage::GetPolicyRuleActionSnapshot>,
    /// A `version` block as documented below.
    #[builder(into)]
    #[serde(rename = "versions")]
    pub r#versions: Vec<super::super::types::storage::GetPolicyRuleActionVersion>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPolicyRuleAction {
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
                    "base_blobs",
                    &self.r#base_blobs,
                ),
                to_pulumi_object_field(
                    "snapshots",
                    &self.r#snapshots,
                ),
                to_pulumi_object_field(
                    "versions",
                    &self.r#versions,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPolicyRuleAction {
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
                    r#base_blobs: {
                        let field_value = match fields_map.get("base_blobs") {
                            Some(value) => value,
                            None => bail!("Missing field 'base_blobs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#snapshots: {
                        let field_value = match fields_map.get("snapshots") {
                            Some(value) => value,
                            None => bail!("Missing field 'snapshots' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#versions: {
                        let field_value = match fields_map.get("versions") {
                            Some(value) => value,
                            None => bail!("Missing field 'versions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
