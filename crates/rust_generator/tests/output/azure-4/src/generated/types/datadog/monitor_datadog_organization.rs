#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MonitorDatadogOrganization {
    /// Api key associated to the Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "apiKey")]
    pub r#api_key: String,
    /// Application key associated to the Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "applicationKey")]
    pub r#application_key: String,
    /// The ID of the enterprise_app. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "enterpriseAppId")]
    pub r#enterprise_app_id: Option<String>,
    /// The ID of the Datadog Monitor.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The auth code used to linking to an existing Datadog organization. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "linkingAuthCode")]
    pub r#linking_auth_code: Option<String>,
    /// The ID of the linking_client. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "linkingClientId")]
    pub r#linking_client_id: Option<String>,
    /// The name of the user that will be associated with the Datadog Monitor. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The redirect uri for linking. Changing this forces a new Datadog Monitor to be created.
    #[builder(into)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for MonitorDatadogOrganization {
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
                    "api_key",
                    &self.r#api_key,
                ),
                to_pulumi_object_field(
                    "application_key",
                    &self.r#application_key,
                ),
                to_pulumi_object_field(
                    "enterprise_app_id",
                    &self.r#enterprise_app_id,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "linking_auth_code",
                    &self.r#linking_auth_code,
                ),
                to_pulumi_object_field(
                    "linking_client_id",
                    &self.r#linking_client_id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "redirect_uri",
                    &self.r#redirect_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for MonitorDatadogOrganization {
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
                    r#api_key: {
                        let field_value = match fields_map.get("api_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'api_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_key: {
                        let field_value = match fields_map.get("application_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'application_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enterprise_app_id: {
                        let field_value = match fields_map.get("enterprise_app_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'enterprise_app_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linking_auth_code: {
                        let field_value = match fields_map.get("linking_auth_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'linking_auth_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linking_client_id: {
                        let field_value = match fields_map.get("linking_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'linking_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#redirect_uri: {
                        let field_value = match fields_map.get("redirect_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirect_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
