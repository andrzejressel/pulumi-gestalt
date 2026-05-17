#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertRuleScheduledAlertDetailsOverride {
    /// The format containing columns name(s) to override the description of this Sentinel Alert Rule.
    #[builder(into)]
    #[serde(rename = "descriptionFormat")]
    pub r#description_format: Option<String>,
    /// The format containing columns name(s) to override the name of this Sentinel Alert Rule.
    #[builder(into)]
    #[serde(rename = "displayNameFormat")]
    pub r#display_name_format: Option<String>,
    /// A list of `dynamic_property` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "dynamicProperties")]
    pub r#dynamic_properties: Option<Vec<super::super::types::sentinel::AlertRuleScheduledAlertDetailsOverrideDynamicProperty>>,
    /// The column name to take the alert severity from.
    #[builder(into)]
    #[serde(rename = "severityColumnName")]
    pub r#severity_column_name: Option<String>,
    /// The column name to take the alert tactics from.
    #[builder(into)]
    #[serde(rename = "tacticsColumnName")]
    pub r#tactics_column_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AlertRuleScheduledAlertDetailsOverride {
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
                    "description_format",
                    &self.r#description_format,
                ),
                to_pulumi_object_field(
                    "display_name_format",
                    &self.r#display_name_format,
                ),
                to_pulumi_object_field(
                    "dynamic_properties",
                    &self.r#dynamic_properties,
                ),
                to_pulumi_object_field(
                    "severity_column_name",
                    &self.r#severity_column_name,
                ),
                to_pulumi_object_field(
                    "tactics_column_name",
                    &self.r#tactics_column_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AlertRuleScheduledAlertDetailsOverride {
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
                    r#description_format: {
                        let field_value = match fields_map.get("description_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'description_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name_format: {
                        let field_value = match fields_map.get("display_name_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dynamic_properties: {
                        let field_value = match fields_map.get("dynamic_properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'dynamic_properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#severity_column_name: {
                        let field_value = match fields_map.get("severity_column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'severity_column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tactics_column_name: {
                        let field_value = match fields_map.get("tactics_column_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'tactics_column_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
