#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RemoteImageBuildAuthConfig {
    /// the auth token
    #[builder(into)]
    #[serde(rename = "auth")]
    pub r#auth: Option<String>,
    /// the user emal
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// hostname of the registry
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: String,
    /// the identity token
    #[builder(into)]
    #[serde(rename = "identityToken")]
    pub r#identity_token: Option<String>,
    /// the registry password
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Option<String>,
    /// the registry token
    #[builder(into)]
    #[serde(rename = "registryToken")]
    pub r#registry_token: Option<String>,
    /// the server address
    #[builder(into)]
    #[serde(rename = "serverAddress")]
    pub r#server_address: Option<String>,
    /// the registry user name
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RemoteImageBuildAuthConfig {
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
                    "auth",
                    &self.r#auth,
                ),
                to_pulumi_object_field(
                    "email",
                    &self.r#email,
                ),
                to_pulumi_object_field(
                    "hostName",
                    &self.r#host_name,
                ),
                to_pulumi_object_field(
                    "identityToken",
                    &self.r#identity_token,
                ),
                to_pulumi_object_field(
                    "password",
                    &self.r#password,
                ),
                to_pulumi_object_field(
                    "registryToken",
                    &self.r#registry_token,
                ),
                to_pulumi_object_field(
                    "serverAddress",
                    &self.r#server_address,
                ),
                to_pulumi_object_field(
                    "userName",
                    &self.r#user_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RemoteImageBuildAuthConfig {
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
                    r#auth: {
                        let field_value = match fields_map.get("auth") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email: {
                        let field_value = match fields_map.get("email") {
                            Some(value) => value,
                            None => bail!("Missing field 'email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#host_name: {
                        let field_value = match fields_map.get("hostName") {
                            Some(value) => value,
                            None => bail!("Missing field 'hostName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity_token: {
                        let field_value = match fields_map.get("identityToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'identityToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#password: {
                        let field_value = match fields_map.get("password") {
                            Some(value) => value,
                            None => bail!("Missing field 'password' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registry_token: {
                        let field_value = match fields_map.get("registryToken") {
                            Some(value) => value,
                            None => bail!("Missing field 'registryToken' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#server_address: {
                        let field_value = match fields_map.get("serverAddress") {
                            Some(value) => value,
                            None => bail!("Missing field 'serverAddress' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_name: {
                        let field_value = match fields_map.get("userName") {
                            Some(value) => value,
                            None => bail!("Missing field 'userName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
