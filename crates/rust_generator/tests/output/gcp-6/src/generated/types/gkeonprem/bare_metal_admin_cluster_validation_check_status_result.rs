#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalAdminClusterValidationCheckStatusResult {
    /// (Output)
    /// The category of the validation.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Option<String>,
    /// A human readable description of this Bare Metal Admin Cluster.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// (Output)
    /// Detailed failure information, which might be unformatted.
    #[builder(into)]
    #[serde(rename = "details")]
    pub r#details: Option<String>,
    /// (Output)
    /// Options used for the validation check.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: Option<String>,
    /// (Output)
    /// A human-readable message of the check failure.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalAdminClusterValidationCheckStatusResult {
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
                    "category",
                    &self.r#category,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "details",
                    &self.r#details,
                ),
                to_pulumi_object_field(
                    "options",
                    &self.r#options,
                ),
                to_pulumi_object_field(
                    "reason",
                    &self.r#reason,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalAdminClusterValidationCheckStatusResult {
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
                    r#category: {
                        let field_value = match fields_map.get("category") {
                            Some(value) => value,
                            None => bail!("Missing field 'category' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#details: {
                        let field_value = match fields_map.get("details") {
                            Some(value) => value,
                            None => bail!("Missing field 'details' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#options: {
                        let field_value = match fields_map.get("options") {
                            Some(value) => value,
                            None => bail!("Missing field 'options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reason: {
                        let field_value = match fields_map.get("reason") {
                            Some(value) => value,
                            None => bail!("Missing field 'reason' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
