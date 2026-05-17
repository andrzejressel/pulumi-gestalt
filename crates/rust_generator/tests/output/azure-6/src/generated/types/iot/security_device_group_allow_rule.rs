#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecurityDeviceGroupAllowRule {
    /// Specifies which IP is not allowed to be connected to in current device group for inbound connection.
    #[builder(into)]
    #[serde(rename = "connectionFromIpsNotAlloweds")]
    pub r#connection_from_ips_not_alloweds: Option<Vec<String>>,
    /// Specifies which IP is not allowed to be connected to in current device group for outbound connection.
    #[builder(into)]
    #[serde(rename = "connectionToIpsNotAlloweds")]
    pub r#connection_to_ips_not_alloweds: Option<Vec<String>>,
    /// Specifies which local user is not allowed to login in current device group.
    #[builder(into)]
    #[serde(rename = "localUsersNotAlloweds")]
    pub r#local_users_not_alloweds: Option<Vec<String>>,
    /// Specifies which process is not allowed to be executed in current device group.
    #[builder(into)]
    #[serde(rename = "processesNotAlloweds")]
    pub r#processes_not_alloweds: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SecurityDeviceGroupAllowRule {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "connection_from_ips_not_alloweds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_from_ips_not_alloweds,
                )
                .await,
            );
            map.insert(
                "connection_to_ips_not_alloweds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connection_to_ips_not_alloweds,
                )
                .await,
            );
            map.insert(
                "local_users_not_alloweds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_users_not_alloweds,
                )
                .await,
            );
            map.insert(
                "processes_not_alloweds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#processes_not_alloweds,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SecurityDeviceGroupAllowRule {
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
                    r#connection_from_ips_not_alloweds: {
                        let field_value = match fields_map.get("connection_from_ips_not_alloweds") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_from_ips_not_alloweds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connection_to_ips_not_alloweds: {
                        let field_value = match fields_map.get("connection_to_ips_not_alloweds") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_to_ips_not_alloweds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_users_not_alloweds: {
                        let field_value = match fields_map.get("local_users_not_alloweds") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_users_not_alloweds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#processes_not_alloweds: {
                        let field_value = match fields_map.get("processes_not_alloweds") {
                            Some(value) => value,
                            None => bail!("Missing field 'processes_not_alloweds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
