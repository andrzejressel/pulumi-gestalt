#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExpressRoutePortLink1 {
    /// Whether enable administration state on the Express Route Port Link? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "adminEnabled")]
    pub r#admin_enabled: Option<bool>,
    /// The connector type of the Express Route Port Link.
    #[builder(into)]
    #[serde(rename = "connectorType")]
    pub r#connector_type: Option<String>,
    /// The ID of this Express Route Port Link.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The interface name of the Azure router associated with the Express Route Port Link.
    #[builder(into)]
    #[serde(rename = "interfaceName")]
    pub r#interface_name: Option<String>,
    /// The ID of the Key Vault Secret that contains the Mac security CAK key for this Express Route Port Link.
    #[builder(into)]
    #[serde(rename = "macsecCakKeyvaultSecretId")]
    pub r#macsec_cak_keyvault_secret_id: Option<String>,
    /// The MACSec cipher used for this Express Route Port Link. Possible values are `GcmAes128` and `GcmAes256`. Defaults to `GcmAes128`.
    #[builder(into)]
    #[serde(rename = "macsecCipher")]
    pub r#macsec_cipher: Option<String>,
    /// The ID of the Key Vault Secret that contains the MACSec CKN key for this Express Route Port Link.
    #[builder(into)]
    #[serde(rename = "macsecCknKeyvaultSecretId")]
    pub r#macsec_ckn_keyvault_secret_id: Option<String>,
    /// Should Secure Channel Identifier on the Express Route Port Link be enabled? Defaults to `false`.
    /// 
    /// > **NOTE** `macsec_ckn_keyvault_secret_id` and `macsec_cak_keyvault_secret_id` should be used together with `identity`, so that the Express Route Port instance have the right permission to access the Key Vault.
    #[builder(into)]
    #[serde(rename = "macsecSciEnabled")]
    pub r#macsec_sci_enabled: Option<bool>,
    /// The ID that maps from the Express Route Port Link to the patch panel port.
    #[builder(into)]
    #[serde(rename = "patchPanelId")]
    pub r#patch_panel_id: Option<String>,
    /// The ID that maps from the patch panel port to the rack.
    #[builder(into)]
    #[serde(rename = "rackId")]
    pub r#rack_id: Option<String>,
    /// The name of the Azure router associated with the Express Route Port Link.
    #[builder(into)]
    #[serde(rename = "routerName")]
    pub r#router_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExpressRoutePortLink1 {
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
                "admin_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#admin_enabled,
                )
                .await,
            );
            map.insert(
                "connector_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#connector_type,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "interface_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#interface_name,
                )
                .await,
            );
            map.insert(
                "macsec_cak_keyvault_secret_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#macsec_cak_keyvault_secret_id,
                )
                .await,
            );
            map.insert(
                "macsec_cipher".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#macsec_cipher,
                )
                .await,
            );
            map.insert(
                "macsec_ckn_keyvault_secret_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#macsec_ckn_keyvault_secret_id,
                )
                .await,
            );
            map.insert(
                "macsec_sci_enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#macsec_sci_enabled,
                )
                .await,
            );
            map.insert(
                "patch_panel_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#patch_panel_id,
                )
                .await,
            );
            map.insert(
                "rack_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#rack_id,
                )
                .await,
            );
            map.insert(
                "router_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#router_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExpressRoutePortLink1 {
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
                    r#admin_enabled: {
                        let field_value = match fields_map.get("admin_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'admin_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connector_type: {
                        let field_value = match fields_map.get("connector_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'connector_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#interface_name: {
                        let field_value = match fields_map.get("interface_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'interface_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_cak_keyvault_secret_id: {
                        let field_value = match fields_map.get("macsec_cak_keyvault_secret_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsec_cak_keyvault_secret_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_cipher: {
                        let field_value = match fields_map.get("macsec_cipher") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsec_cipher' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_ckn_keyvault_secret_id: {
                        let field_value = match fields_map.get("macsec_ckn_keyvault_secret_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsec_ckn_keyvault_secret_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_sci_enabled: {
                        let field_value = match fields_map.get("macsec_sci_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsec_sci_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patch_panel_id: {
                        let field_value = match fields_map.get("patch_panel_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'patch_panel_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rack_id: {
                        let field_value = match fields_map.get("rack_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'rack_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#router_name: {
                        let field_value = match fields_map.get("router_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'router_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
