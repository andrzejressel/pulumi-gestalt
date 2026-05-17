#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApplicationScimConfig {
    /// Attributes for configuring HTTP Basic, OAuth Bearer token, or OAuth 2 authentication schemes for SCIM provisioning to an application.
    #[builder(into)]
    #[serde(rename = "authentication")]
    pub r#authentication: Option<Box<super::types::AccessApplicationScimConfigAuthentication>>,
    /// If false, propagates DELETE requests to the target application for SCIM resources. If true, sets 'active' to false on the SCIM resource. Note: Some targets do not support DELETE operations.
    #[builder(into)]
    #[serde(rename = "deactivateOnDelete")]
    pub r#deactivate_on_delete: Option<bool>,
    /// Whether SCIM provisioning is turned on for this application.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The UID of the IdP to use as the source for SCIM resources to provision to this application.
    #[builder(into)]
    #[serde(rename = "idpUid")]
    pub r#idp_uid: String,
    /// A list of mappings to apply to SCIM resources before provisioning them in this application. These can transform or filter the resources to be provisioned.
    #[builder(into)]
    #[serde(rename = "mappings")]
    pub r#mappings: Option<Vec<super::types::AccessApplicationScimConfigMapping>>,
    /// The base URI for the application's SCIM-compatible API.
    #[builder(into)]
    #[serde(rename = "remoteUri")]
    pub r#remote_uri: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessApplicationScimConfig {
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
                    "authentication",
                    &self.r#authentication,
                ),
                to_pulumi_object_field(
                    "deactivate_on_delete",
                    &self.r#deactivate_on_delete,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "idp_uid",
                    &self.r#idp_uid,
                ),
                to_pulumi_object_field(
                    "mappings",
                    &self.r#mappings,
                ),
                to_pulumi_object_field(
                    "remote_uri",
                    &self.r#remote_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessApplicationScimConfig {
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
                    r#authentication: {
                        let field_value = match fields_map.get("authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#deactivate_on_delete: {
                        let field_value = match fields_map.get("deactivate_on_delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'deactivate_on_delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idp_uid: {
                        let field_value = match fields_map.get("idp_uid") {
                            Some(value) => value,
                            None => bail!("Missing field 'idp_uid' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mappings: {
                        let field_value = match fields_map.get("mappings") {
                            Some(value) => value,
                            None => bail!("Missing field 'mappings' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#remote_uri: {
                        let field_value = match fields_map.get("remote_uri") {
                            Some(value) => value,
                            None => bail!("Missing field 'remote_uri' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
