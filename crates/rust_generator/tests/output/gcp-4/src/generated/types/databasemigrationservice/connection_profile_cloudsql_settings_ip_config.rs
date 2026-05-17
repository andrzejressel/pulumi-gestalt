#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionProfileCloudsqlSettingsIpConfig {
    /// The list of external networks that are allowed to connect to the instance using the IP.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authorizedNetworks")]
    pub r#authorized_networks: Option<Vec<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettingsIpConfigAuthorizedNetwork>>,
    /// Whether the instance should be assigned an IPv4 address or not.
    #[builder(into)]
    #[serde(rename = "enableIpv4")]
    pub r#enable_ipv_4: Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is accessible for private IP. For example, projects/myProject/global/networks/default.
    /// This setting can be updated, but it cannot be removed after it is set.
    #[builder(into)]
    #[serde(rename = "privateNetwork")]
    pub r#private_network: Option<String>,
    /// Whether SSL connections over IP should be enforced or not.
    #[builder(into)]
    #[serde(rename = "requireSsl")]
    pub r#require_ssl: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionProfileCloudsqlSettingsIpConfig {
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
                    "authorized_networks",
                    &self.r#authorized_networks,
                ),
                to_pulumi_object_field(
                    "enable_ipv_4",
                    &self.r#enable_ipv_4,
                ),
                to_pulumi_object_field(
                    "private_network",
                    &self.r#private_network,
                ),
                to_pulumi_object_field(
                    "require_ssl",
                    &self.r#require_ssl,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionProfileCloudsqlSettingsIpConfig {
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
                    r#authorized_networks: {
                        let field_value = match fields_map.get("authorized_networks") {
                            Some(value) => value,
                            None => bail!("Missing field 'authorized_networks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_ipv_4: {
                        let field_value = match fields_map.get("enable_ipv_4") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_ipv_4' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#private_network: {
                        let field_value = match fields_map.get("private_network") {
                            Some(value) => value,
                            None => bail!("Missing field 'private_network' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#require_ssl: {
                        let field_value = match fields_map.get("require_ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'require_ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
