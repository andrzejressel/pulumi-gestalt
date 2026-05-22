#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExpressRoutePortLink2 {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExpressRoutePortLink2 {
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
                    "adminEnabled",
                    &self.r#admin_enabled,
                ),
                to_pulumi_object_field(
                    "connectorType",
                    &self.r#connector_type,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "interfaceName",
                    &self.r#interface_name,
                ),
                to_pulumi_object_field(
                    "macsecCakKeyvaultSecretId",
                    &self.r#macsec_cak_keyvault_secret_id,
                ),
                to_pulumi_object_field(
                    "macsecCipher",
                    &self.r#macsec_cipher,
                ),
                to_pulumi_object_field(
                    "macsecCknKeyvaultSecretId",
                    &self.r#macsec_ckn_keyvault_secret_id,
                ),
                to_pulumi_object_field(
                    "macsecSciEnabled",
                    &self.r#macsec_sci_enabled,
                ),
                to_pulumi_object_field(
                    "patchPanelId",
                    &self.r#patch_panel_id,
                ),
                to_pulumi_object_field(
                    "rackId",
                    &self.r#rack_id,
                ),
                to_pulumi_object_field(
                    "routerName",
                    &self.r#router_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExpressRoutePortLink2 {
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
                        let field_value = match fields_map.get("adminEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'adminEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connector_type: {
                        let field_value = match fields_map.get("connectorType") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectorType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("interfaceName") {
                            Some(value) => value,
                            None => bail!("Missing field 'interfaceName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_cak_keyvault_secret_id: {
                        let field_value = match fields_map.get("macsecCakKeyvaultSecretId") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsecCakKeyvaultSecretId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_cipher: {
                        let field_value = match fields_map.get("macsecCipher") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsecCipher' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_ckn_keyvault_secret_id: {
                        let field_value = match fields_map.get("macsecCknKeyvaultSecretId") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsecCknKeyvaultSecretId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#macsec_sci_enabled: {
                        let field_value = match fields_map.get("macsecSciEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'macsecSciEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#patch_panel_id: {
                        let field_value = match fields_map.get("patchPanelId") {
                            Some(value) => value,
                            None => bail!("Missing field 'patchPanelId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#rack_id: {
                        let field_value = match fields_map.get("rackId") {
                            Some(value) => value,
                            None => bail!("Missing field 'rackId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#router_name: {
                        let field_value = match fields_map.get("routerName") {
                            Some(value) => value,
                            None => bail!("Missing field 'routerName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
