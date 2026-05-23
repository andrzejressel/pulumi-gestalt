#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CxPageFormParameter {
    /// Hierarchical advanced settings for this parameter. The settings exposed at the lower level overrides the settings exposed at the higher level.
    /// Hierarchy: Agent->Flow->Page->Fulfillment/Parameter.
    /// Structure is documented below.
    #[builder(into)]
    pub r#advanced_settings: Option<Box<super::super::types::diagflow::CxPageFormParameterAdvancedSettings>>,
    /// The default value of an optional parameter. If the parameter is required, the default value will be ignored.
    #[builder(into)]
    pub r#default_value: Option<String>,
    /// The human-readable name of the parameter, unique within the form.
    #[builder(into)]
    pub r#display_name: Option<String>,
    /// The entity type of the parameter.
    /// Format: projects/-/locations/-/agents/-/entityTypes/<System Entity Type ID> for system entity types (for example, projects/-/locations/-/agents/-/entityTypes/sys.date), or projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/entityTypes/<Entity Type ID> for developer entity types.
    #[builder(into)]
    pub r#entity_type: Option<String>,
    /// Defines fill behavior for the parameter.
    /// Structure is documented below.
    #[builder(into)]
    pub r#fill_behavior: Option<Box<super::super::types::diagflow::CxPageFormParameterFillBehavior>>,
    /// Indicates whether the parameter represents a list of values.
    #[builder(into)]
    pub r#is_list: Option<bool>,
    /// Indicates whether the parameter content should be redacted in log.
    /// If redaction is enabled, the parameter content will be replaced by parameter name during logging. Note: the parameter content is subject to redaction if either parameter level redaction or entity type level redaction is enabled.
    #[builder(into)]
    pub r#redact: Option<bool>,
    /// Indicates whether the parameter is required. Optional parameters will not trigger prompts; however, they are filled if the user specifies them.
    /// Required parameters must be filled before form filling concludes.
    #[builder(into)]
    pub r#required: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CxPageFormParameter {
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
                    "advancedSettings",
                    &self.r#advanced_settings,
                ),
                to_pulumi_object_field(
                    "defaultValue",
                    &self.r#default_value,
                ),
                to_pulumi_object_field(
                    "displayName",
                    &self.r#display_name,
                ),
                to_pulumi_object_field(
                    "entityType",
                    &self.r#entity_type,
                ),
                to_pulumi_object_field(
                    "fillBehavior",
                    &self.r#fill_behavior,
                ),
                to_pulumi_object_field(
                    "isList",
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
        .boxed()
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
                        let field_value = match fields_map.get("advancedSettings") {
                            Some(value) => value,
                            None => bail!("Missing field 'advancedSettings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_value: {
                        let field_value = match fields_map.get("defaultValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#display_name: {
                        let field_value = match fields_map.get("displayName") {
                            Some(value) => value,
                            None => bail!("Missing field 'displayName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#entity_type: {
                        let field_value = match fields_map.get("entityType") {
                            Some(value) => value,
                            None => bail!("Missing field 'entityType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fill_behavior: {
                        let field_value = match fields_map.get("fillBehavior") {
                            Some(value) => value,
                            None => bail!("Missing field 'fillBehavior' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#is_list: {
                        let field_value = match fields_map.get("isList") {
                            Some(value) => value,
                            None => bail!("Missing field 'isList' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
