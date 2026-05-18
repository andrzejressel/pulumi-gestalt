#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionAzure {
    /// (Output)
    /// The name of the Azure Active Directory Application.
    #[builder(into)]
    #[serde(rename = "application")]
    pub r#application: Option<String>,
    /// (Output)
    /// The client id of the Azure Active Directory Application.
    #[builder(into)]
    #[serde(rename = "clientId")]
    pub r#client_id: Option<String>,
    /// The id of customer's directory that host the data.
    #[builder(into)]
    #[serde(rename = "customerTenantId")]
    pub r#customer_tenant_id: String,
    /// The Azure Application (client) ID where the federated credentials will be hosted.
    #[builder(into)]
    #[serde(rename = "federatedApplicationClientId")]
    pub r#federated_application_client_id: Option<String>,
    /// (Output)
    /// A unique Google-owned and Google-generated identity for the Connection. This identity will be used to access the user's Azure Active Directory Application.
    #[builder(into)]
    #[serde(rename = "identity")]
    pub r#identity: Option<String>,
    /// (Output)
    /// The object id of the Azure Active Directory Application.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: Option<String>,
    /// (Output)
    /// The URL user will be redirected to after granting consent during connection setup.
    #[builder(into)]
    #[serde(rename = "redirectUri")]
    pub r#redirect_uri: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionAzure {
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
                    "application",
                    &self.r#application,
                ),
                to_pulumi_object_field(
                    "client_id",
                    &self.r#client_id,
                ),
                to_pulumi_object_field(
                    "customer_tenant_id",
                    &self.r#customer_tenant_id,
                ),
                to_pulumi_object_field(
                    "federated_application_client_id",
                    &self.r#federated_application_client_id,
                ),
                to_pulumi_object_field(
                    "identity",
                    &self.r#identity,
                ),
                to_pulumi_object_field(
                    "object_id",
                    &self.r#object_id,
                ),
                to_pulumi_object_field(
                    "redirect_uri",
                    &self.r#redirect_uri,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionAzure {
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
                    r#application: {
                        let field_value = match fields_map.get("application") {
                            Some(value) => value,
                            None => bail!("Missing field 'application' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#client_id: {
                        let field_value = match fields_map.get("client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#customer_tenant_id: {
                        let field_value = match fields_map.get("customer_tenant_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'customer_tenant_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#federated_application_client_id: {
                        let field_value = match fields_map.get("federated_application_client_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'federated_application_client_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identity: {
                        let field_value = match fields_map.get("identity") {
                            Some(value) => value,
                            None => bail!("Missing field 'identity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_id: {
                        let field_value = match fields_map.get("object_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
