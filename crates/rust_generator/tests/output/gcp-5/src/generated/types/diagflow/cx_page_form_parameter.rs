#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageFormParameter {
    /// Hierarchical advanced settings for this parameter. The settings exposed at the lower level overrides the settings exposed at the higher level.
    /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "advancedSettings")]
    pub r#advanced_settings: Option<Box<super::super::types::diagflow::CxPageFormParameterAdvancedSettings>>,
    /// The default value of an optional parameter. If the parameter is required, the default value will be ignored.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// The human-readable name of the parameter, unique within the form.
    #[builder(into)]
    #[serde(rename = "displayName")]
    pub r#display_name: Option<String>,
    /// The entity type of the parameter.
    /// Format: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types.
    #[builder(into)]
    #[serde(rename = "entityType")]
    pub r#entity_type: Option<String>,
    /// Defines fill behavior for the parameter.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "fillBehavior")]
    pub r#fill_behavior: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehavior>>,
    /// Indicates whether the parameter represents a list of values.
    #[builder(into)]
    #[serde(rename = "isList")]
    pub r#is_list: Option<bool>,
    /// Indicates whether the parameter content should be redacted in log.
    /// If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled.
    #[builder(into)]
    #[serde(rename = "redact")]
    pub r#redact: Option<bool>,
    /// Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them.
    /// Required parameters must be filled before form filling concludes.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxPageFormParameter {
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
                    "advanced_settings",
                    &self.r#advanced_settings,
                ),
                to_pulumi_object_field(
                    "default_value",
                    &self.r#default_value,
                ),
                to_pulumi_object_field(
                    "display_name",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "entity_type",
                    &self.r#entity_type,
                ),
                to_pulumi_object_field(
                    "fill_behavior",
                    &self.r#fill_behavior,
                ),
                to_pulumi_object_field(
                    "is_list",
                    &self.r#is_list,
                ),
                to_pulumi_object_field(
                    "redact",
                    &self.r#redact,
                ),
                to_pulumi_object_field(
                    "required",
                    &self.r#required,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CxPageFormParameter {
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
                    r#advanced_settings: {
                        let field_value = match fields_map.get("advanced_settings") {
                            Some(value) => value,
                            None => bail!("Missing field 'advanced_settings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_value: {
                        let field_value = match fields_map.get("default_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("display_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'display_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entity_type: {
                        let field_value = match fields_map.get("entity_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'entity_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fill_behavior: {
                        let field_value = match fields_map.get("fill_behavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'fill_behavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_list: {
                        let field_value = match fields_map.get("is_list") {
                            Some(value) => value,
                            None => bail!("Missing field 'is_list' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redact: {
                        let field_value = match fields_map.get("redact") {
                            Some(value) => value,
                            None => bail!("Missing field 'redact' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required: {
                        let field_value = match fields_map.get("required") {
                            Some(value) => value,
                            None => bail!("Missing field 'required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
