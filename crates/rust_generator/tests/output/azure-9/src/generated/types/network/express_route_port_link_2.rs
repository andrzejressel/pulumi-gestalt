#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
