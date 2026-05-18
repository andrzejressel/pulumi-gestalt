#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerRuleAction {
    /// An action to authenticate using Amazon Cognito.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "authenticateCognito")]
    pub r#authenticate_cognito: Option<Box<super::super::types::lb::GetListenerRuleActionAuthenticateCognito>>,
    /// An action to authenticate using OIDC.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "authenticateOidc")]
    pub r#authenticate_oidc: Option<Box<super::super::types::lb::GetListenerRuleActionAuthenticateOidc>>,
    /// An action to return a fixed response.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Option<Box<super::super::types::lb::GetListenerRuleActionFixedResponse>>,
    /// An action to forward the request.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "forward")]
    pub r#forward: Option<Box<super::super::types::lb::GetListenerRuleActionForward>>,
    /// The evaluation order of the action.
    #[builder(into)]
    #[serde(rename = "order")]
    pub r#order: f64,
    /// An action to redirect the request.
    /// Detailed below.
    #[builder(into)]
    #[serde(rename = "redirect")]
    pub r#redirect: Option<Box<super::super::types::lb::GetListenerRuleActionRedirect>>,
    /// The type of the action, indicates which sub-block will be populated.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetListenerRuleAction {
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
                    "authenticate_cognito",
                    &self.r#authenticate_cognito,
                ),
                to_pulumi_object_field(
                    "authenticate_oidc",
                    &self.r#authenticate_oidc,
                ),
                to_pulumi_object_field(
                    "fixed_response",
                    &self.r#fixed_response,
                ),
                to_pulumi_object_field(
                    "forward",
                    &self.r#forward,
                ),
                to_pulumi_object_field(
                    "order",
                    &self.r#order,
                ),
                to_pulumi_object_field(
                    "redirect",
                    &self.r#redirect,
                ),
                to_pulumi_object_field(
                    "type_",
                    &self.r#type_,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetListenerRuleAction {
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
                    r#authenticate_cognito: {
                        let field_value = match fields_map.get("authenticate_cognito") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticate_cognito' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#authenticate_oidc: {
                        let field_value = match fields_map.get("authenticate_oidc") {
                            Some(value) => value,
                            None => bail!("Missing field 'authenticate_oidc' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fixed_response: {
                        let field_value = match fields_map.get("fixed_response") {
                            Some(value) => value,
                            None => bail!("Missing field 'fixed_response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward: {
                        let field_value = match fields_map.get("forward") {
                            Some(value) => value,
                            None => bail!("Missing field 'forward' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#order: {
                        let field_value = match fields_map.get("order") {
                            Some(value) => value,
                            None => bail!("Missing field 'order' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect: {
                        let field_value = match fields_map.get("redirect") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
