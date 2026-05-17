#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateMapGclbTarget {
    /// An IP configuration where this Certificate Map is serving
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ipConfigs")]
    pub r#ip_configs: Option<Vec<super::super::types::certificatemanager::CertificateMapGclbTargetIpConfig>>,
    /// Proxy name must be in the format projects/*/locations/*/targetHttpsProxies/*.
    /// This field is part of a union field `target_proxy`: Only one of `targetHttpsProxy` or
    /// `targetSslProxy` may be set.
    #[builder(into)]
    #[serde(rename = "targetHttpsProxy")]
    pub r#target_https_proxy: Option<String>,
    /// Proxy name must be in the format projects/*/locations/*/targetSslProxies/*.
    /// This field is part of a union field `target_proxy`: Only one of `targetHttpsProxy` or
    /// `targetSslProxy` may be set.
    #[builder(into)]
    #[serde(rename = "targetSslProxy")]
    pub r#target_ssl_proxy: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateMapGclbTarget {
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
                "ip_configs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_configs,
                )
                .await,
            );
            map.insert(
                "target_https_proxy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_https_proxy,
                )
                .await,
            );
            map.insert(
                "target_ssl_proxy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_ssl_proxy,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateMapGclbTarget {
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
                    r#ip_configs: {
                        let field_value = match fields_map.get("ip_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_https_proxy: {
                        let field_value = match fields_map.get("target_https_proxy") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_https_proxy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_ssl_proxy: {
                        let field_value = match fields_map.get("target_ssl_proxy") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_ssl_proxy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
