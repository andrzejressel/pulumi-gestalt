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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "apiKey",
                    &self.r#api_key,
                ),
                to_pulumi_object_field(
                    "applicationKey",
                    &self.r#application_key,
                ),
                to_pulumi_object_field(
                    "enterpriseAppId",
                    &self.r#enterprise_app_id,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "linkingAuthCode",
                    &self.r#linking_auth_code,
                ),
                to_pulumi_object_field(
                    "linkingClientId",
                    &self.r#linking_client_id,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "redirectUri",
                    &self.r#redirect_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("apiKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'apiKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#application_key: {
                        let field_value = match fields_map.get("applicationKey") {
                            Some(value) => value,
                            None => bail!("Missing field 'applicationKey' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enterprise_app_id: {
                        let field_value = match fields_map.get("enterpriseAppId") {
                            Some(value) => value,
                            None => bail!("Missing field 'enterpriseAppId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("linkingAuthCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'linkingAuthCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#linking_client_id: {
                        let field_value = match fields_map.get("linkingClientId") {
                            Some(value) => value,
                            None => bail!("Missing field 'linkingClientId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("redirectUri") {
                            Some(value) => value,
                            None => bail!("Missing field 'redirectUri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
