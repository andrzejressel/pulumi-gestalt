#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerAzureadAdministrator {
    /// Specifies whether only AD Users and administrators (e.g. `azuread_administrator[0].login_username`) can be used to login, or also local database users (e.g. `administrator_login`). When `true`, the `administrator_login` and `administrator_login_password` properties can be omitted.
    #[builder(into)]
    #[serde(rename = "azureadAuthenticationOnly")]
    pub r#azuread_authentication_only: Option<bool>,
    /// The login username of the Azure AD Administrator of this SQL Server.
    #[builder(into)]
    #[serde(rename = "loginUsername")]
    pub r#login_username: String,
    /// The object id of the Azure AD Administrator of this SQL Server.
    #[builder(into)]
    #[serde(rename = "objectId")]
    pub r#object_id: String,
    /// The tenant id of the Azure AD Administrator of this SQL Server.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServerAzureadAdministrator {
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
                    "azuread_authentication_only",
                    &self.r#azuread_authentication_only,
                ),
                to_pulumi_object_field(
                    "login_username",
                    &self.r#login_username,
                ),
                to_pulumi_object_field(
                    "object_id",
                    &self.r#object_id,
                ),
                to_pulumi_object_field(
                    "tenant_id",
                    &self.r#tenant_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServerAzureadAdministrator {
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
                    r#azuread_authentication_only: {
                        let field_value = match fields_map.get("azuread_authentication_only") {
                            Some(value) => value,
                            None => bail!("Missing field 'azuread_authentication_only' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#login_username: {
                        let field_value = match fields_map.get("login_username") {
                            Some(value) => value,
                            None => bail!("Missing field 'login_username' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#tenant_id: {
                        let field_value = match fields_map.get("tenant_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'tenant_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
