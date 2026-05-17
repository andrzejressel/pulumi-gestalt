#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkAttachmentConnectionEndpoint {
    /// (Output)
    /// The IPv4 address assigned to the producer instance network interface. This value will be a range in case of Serverless.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// (Output)
    /// The project id or number of the interface to which the IP was assigned.
    #[builder(into)]
    #[serde(rename = "projectIdOrNum")]
    pub r#project_id_or_num: Option<String>,
    /// (Output)
    /// Alias IP ranges from the same subnetwork.
    #[builder(into)]
    #[serde(rename = "secondaryIpCidrRanges")]
    pub r#secondary_ip_cidr_ranges: Option<String>,
    /// (Output)
    /// The status of a connected endpoint to this network attachment.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// (Output)
    /// The subnetwork used to assign the IP to the producer instance network interface.
    #[builder(into)]
    #[serde(rename = "subnetwork")]
    pub r#subnetwork: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for NetworkAttachmentConnectionEndpoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "ip_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_address,
                )
                .await,
            );
            map.insert(
                "project_id_or_num".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#project_id_or_num,
                )
                .await,
            );
            map.insert(
                "secondary_ip_cidr_ranges".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_ip_cidr_ranges,
                )
                .await,
            );
            map.insert(
                "status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#status,
                )
                .await,
            );
            map.insert(
                "subnetwork".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnetwork,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for NetworkAttachmentConnectionEndpoint {
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
                    r#ip_address: {
                        let field_value = match fields_map.get("ip_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#project_id_or_num: {
                        let field_value = match fields_map.get("project_id_or_num") {
                            Some(value) => value,
                            None => bail!("Missing field 'project_id_or_num' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_ip_cidr_ranges: {
                        let field_value = match fields_map.get("secondary_ip_cidr_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_ip_cidr_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#status: {
                        let field_value = match fields_map.get("status") {
                            Some(value) => value,
                            None => bail!("Missing field 'status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork: {
                        let field_value = match fields_map.get("subnetwork") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
