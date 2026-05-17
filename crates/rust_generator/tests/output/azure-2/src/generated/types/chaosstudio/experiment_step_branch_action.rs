#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExperimentStepBranchAction {
    /// The type of action that should be added to the experiment. Possible values are `continuous`, `delay` and `discrete`.
    #[builder(into)]
    #[serde(rename = "actionType")]
    pub r#action_type: String,
    /// An ISO8601 formatted string specifying the duration for a `delay` or `continuous` action.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Option<String>,
    /// A key-value map of additional parameters to configure the action. The values that are accepted by this depend on the `urn` i.e. the capability/fault that is applied. Possible parameter values can be found in this [documentation](https://learn.microsoft.com/azure/chaos-studio/chaos-studio-fault-library)
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Option<std::collections::HashMap<String, String>>,
    /// The name of the Selector to which this action should apply to. This must be specified if the `action_type` is `continuous` or `discrete`.
    #[builder(into)]
    #[serde(rename = "selectorName")]
    pub r#selector_name: Option<String>,
    /// The Unique Resource Name of the action, this value is provided by the `azure.chaosstudio.Capability` resource e.g. `azurerm_chaos_studio_capability.example.urn`. This must be specified if the `action_type` is `continuous` or `discrete`.
    #[builder(into)]
    #[serde(rename = "urn")]
    pub r#urn: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExperimentStepBranchAction {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "action_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#action_type,
                )
                .await,
            );
            map.insert(
                "duration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#duration,
                )
                .await,
            );
            map.insert(
                "parameters".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#parameters,
                )
                .await,
            );
            map.insert(
                "selector_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selector_name,
                )
                .await,
            );
            map.insert(
                "urn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#urn,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExperimentStepBranchAction {
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
                    r#action_type: {
                        let field_value = match fields_map.get("action_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'action_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#duration: {
                        let field_value = match fields_map.get("duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#parameters: {
                        let field_value = match fields_map.get("parameters") {
                            Some(value) => value,
                            None => bail!("Missing field 'parameters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selector_name: {
                        let field_value = match fields_map.get("selector_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'selector_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#urn: {
                        let field_value = match fields_map.get("urn") {
                            Some(value) => value,
                            None => bail!("Missing field 'urn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
