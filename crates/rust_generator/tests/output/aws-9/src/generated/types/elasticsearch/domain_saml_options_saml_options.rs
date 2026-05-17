#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainSamlOptionsSamlOptions {
    /// Whether SAML authentication is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Information from your identity provider.
    #[builder(into)]
    #[serde(rename = "idp")]
    pub r#idp: Option<Box<super::super::types::elasticsearch::DomainSamlOptionsSamlOptionsIdp>>,
    /// This backend role from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into)]
    #[serde(rename = "masterBackendRole")]
    pub r#master_backend_role: Option<String>,
    /// This username from the SAML IdP receives full permissions to the cluster, equivalent to a new master user.
    #[builder(into)]
    #[serde(rename = "masterUserName")]
    pub r#master_user_name: Option<String>,
    /// Element of the SAML assertion to use for backend roles. Default is roles.
    #[builder(into)]
    #[serde(rename = "rolesKey")]
    pub r#roles_key: Option<String>,
    /// Duration of a session in minutes after a user logs in. Default is 60. Maximum value is 1,440.
    #[builder(into)]
    #[serde(rename = "sessionTimeoutMinutes")]
    pub r#session_timeout_minutes: Option<i32>,
    /// Custom SAML attribute to use for user names. Default is an empty string - `""`. This will cause Elasticsearch to use the `NameID` element of the `Subject`, which is the default location for name identifiers in the SAML specification.
    #[builder(into)]
    #[serde(rename = "subjectKey")]
    pub r#subject_key: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainSamlOptionsSamlOptions {
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
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "idp",
                    &self.r#idp,
                ),
                to_pulumi_object_field(
                    "master_backend_role",
                    &self.r#master_backend_role,
                ),
                to_pulumi_object_field(
                    "master_user_name",
                    &self.r#master_user_name,
                ),
                to_pulumi_object_field(
                    "roles_key",
                    &self.r#roles_key,
                ),
                to_pulumi_object_field(
                    "session_timeout_minutes",
                    &self.r#session_timeout_minutes,
                ),
                to_pulumi_object_field(
                    "subject_key",
                    &self.r#subject_key,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainSamlOptionsSamlOptions {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp: {
                        let field_value = match fields_map.get("idp") {
                            Some(value) => value,
                            None => bail!("Missing field 'idp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_backend_role: {
                        let field_value = match fields_map.get("master_backend_role") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_backend_role' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#master_user_name: {
                        let field_value = match fields_map.get("master_user_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'master_user_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#roles_key: {
                        let field_value = match fields_map.get("roles_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'roles_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_timeout_minutes: {
                        let field_value = match fields_map.get("session_timeout_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_timeout_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subject_key: {
                        let field_value = match fields_map.get("subject_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'subject_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
