#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetOrganizationPolicyListPolicy {
    /// One or the other must be set.
    #[builder(into)]
    #[serde(rename = "allows")]
    pub r#allows: Vec<super::super::types::projects::GetOrganizationPolicyListPolicyAllow>,
    /// One or the other must be set.
    #[builder(into)]
    #[serde(rename = "denies")]
    pub r#denies: Vec<super::super::types::projects::GetOrganizationPolicyListPolicyDeny>,
    /// If set to true, the values from the effective Policy of the parent resource are inherited, meaning the values set in this Policy are added to the values inherited up the hierarchy.
    #[builder(into)]
    #[serde(rename = "inheritFromParent")]
    pub r#inherit_from_parent: bool,
    /// The Google Cloud Console will try to default to a configuration that matches the value specified in this field.
    #[builder(into)]
    #[serde(rename = "suggestedValue")]
    pub r#suggested_value: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetOrganizationPolicyListPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "allows",
                    &self.r#allows,
                ),
                to_pulumi_object_field(
                    "denies",
                    &self.r#denies,
                ),
                to_pulumi_object_field(
                    "inherit_from_parent",
                    &self.r#inherit_from_parent,
                ),
                to_pulumi_object_field(
                    "suggested_value",
                    &self.r#suggested_value,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetOrganizationPolicyListPolicy {
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
                    r#allows: {
                        let field_value = match fields_map.get("allows") {
                            Some(value) => value,
                            None => bail!("Missing field 'allows' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#denies: {
                        let field_value = match fields_map.get("denies") {
                            Some(value) => value,
                            None => bail!("Missing field 'denies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inherit_from_parent: {
                        let field_value = match fields_map.get("inherit_from_parent") {
                            Some(value) => value,
                            None => bail!("Missing field 'inherit_from_parent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#suggested_value: {
                        let field_value = match fields_map.get("suggested_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'suggested_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
