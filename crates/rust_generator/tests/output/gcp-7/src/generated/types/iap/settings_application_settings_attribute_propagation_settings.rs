#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsApplicationSettingsAttributePropagationSettings {
    /// Whether the provided attribute propagation settings should be evaluated on user requests.
    /// If set to true, attributes returned from the expression will be propagated in the set output credentials.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Option<bool>,
    /// Raw string CEL expression. Must return a list of attributes. A maximum of 45 attributes can
    /// be selected. Expressions can select different attribute types from attributes:
    /// attributes.saml_attributes, attributes.iap_attributes.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Option<String>,
    /// Which output credentials attributes selected by the CEL expression should be propagated in.
    /// All attributes will be fully duplicated in each selected output credential.
    /// Possible values are:
    /// * `HEADER`: Propagate attributes in the headers with "x-goog-iap-attr-" prefix.
    /// * `JWT`: Propagate attributes in the JWT of the form:
    /// "additional_claims": { "my_attribute": ["value1", "value2"] }
    /// * `RCTOKEN`: Propagate attributes in the RCToken of the form: "
    /// additional_claims": { "my_attribute": ["value1", "value2"] }
    /// Each value may be one of: `HEADER`, `JWT`, `RCTOKEN`.
    #[builder(into)]
    #[serde(rename = "outputCredentials")]
    pub r#output_credentials: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsApplicationSettingsAttributePropagationSettings {
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
                    "enable",
                    &self.r#enable,
                ),
                to_pulumi_object_field(
                    "expression",
                    &self.r#expression,
                ),
                to_pulumi_object_field(
                    "output_credentials",
                    &self.r#output_credentials,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SettingsApplicationSettingsAttributePropagationSettings {
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
                    r#enable: {
                        let field_value = match fields_map.get("enable") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#expression: {
                        let field_value = match fields_map.get("expression") {
                            Some(value) => value,
                            None => bail!("Missing field 'expression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_credentials: {
                        let field_value = match fields_map.get("output_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
