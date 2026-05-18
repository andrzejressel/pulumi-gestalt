#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouterConfigurationRouter {
    /// Router platform
    #[builder(into)]
    #[serde(rename = "platform")]
    pub r#platform: String,
    /// ID of the Router Type. For example: `CiscoSystemsInc-2900SeriesRouters-IOS124`
    /// 
    /// There is currently no AWS API to retrieve the full list of `router_type_identifier` values. Here is a list of known `RouterType` objects that can be used:
    /// 
    /// ```json
    /// {
    /// "routerTypes": [
    /// {"platform":"2900 Series Routers","routerTypeIdentifier":"CiscoSystemsInc-2900SeriesRouters-IOS124","software":"IOS 12.4+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-router-cisco-generic.xslt","xsltTemplateNameForMacSec":""},
    /// {"platform":"3700 Series Routers","routerTypeIdentifier":"CiscoSystemsInc-3700SeriesRouters-IOS124","software":"IOS 12.4+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-router-cisco-generic.xslt","xsltTemplateNameForMacSec":""},
    /// {"platform":"7200 Series Routers","routerTypeIdentifier":"CiscoSystemsInc-7200SeriesRouters-IOS124","software":"IOS 12.4+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-router-cisco-generic.xslt","xsltTemplateNameForMacSec":""},
    /// {"platform":"Nexus 7000 Series Switches","routerTypeIdentifier":"CiscoSystemsInc-Nexus7000SeriesSwitches-NXOS51","software":"NX-OS 5.1+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-switch-cisco-nexus-generic.xslt","xsltTemplateNameForMacSec":""},
    /// {"platform":"Nexus 9K+ Series Switches","routerTypeIdentifier":"CiscoSystemsInc-Nexus9KSeriesSwitches-NXOS93","software":"NX-OS 9.3+","vendor":"Cisco Systems, Inc.","xsltTemplateName":"customer-switch-cisco-nexus-generic.xslt","xsltTemplateNameForMacSec":"customer-switch-cisco-nexus-generic-macsec.xslt"},
    /// {"platform":"M/MX Series Routers","routerTypeIdentifier":"JuniperNetworksInc-MMXSeriesRouters-JunOS95","software":"JunOS 9.5+","vendor":"Juniper Networks, Inc.","xsltTemplateName":"customer-router-juniper-generic.xslt","xsltTemplateNameForMacSec":"customer-router-juniper-generic-macsec.xslt"},
    /// {"platform":"SRX Series Routers","routerTypeIdentifier":"JuniperNetworksInc-SRXSeriesRouters-JunOS95","software":"JunOS 9.5+","vendor":"Juniper Networks, Inc.","xsltTemplateName":"customer-router-juniper-generic.xslt","xsltTemplateNameForMacSec":""},
    /// {"platform":"T Series Routers","routerTypeIdentifier":"JuniperNetworksInc-TSeriesRouters-JunOS95","software":"JunOS 9.5+","vendor":"Juniper Networks, Inc.","xsltTemplateName":"customer-router-juniper-generic.xslt","xsltTemplateNameForMacSec":""},
    /// {"platform":"PA-3000+ and 5000+ series","routerTypeIdentifier":"PaloAltoNetworks-PA3000and5000series-PANOS803","software":"PAN-OS 8.0.3+","vendor":"Palo Alto Networks","xsltTemplateName":"customer-router-palo-alto-generic.xslt","xsltTemplateNameForMacSec":""}]
    /// }
    /// ```
    #[builder(into)]
    #[serde(rename = "routerTypeIdentifier")]
    pub r#router_type_identifier: String,
    /// Router operating system
    #[builder(into)]
    #[serde(rename = "software")]
    pub r#software: String,
    /// Router vendor
    #[builder(into)]
    #[serde(rename = "vendor")]
    pub r#vendor: String,
    /// Router XSLT Template Name
    #[builder(into)]
    #[serde(rename = "xsltTemplateName")]
    pub r#xslt_template_name: String,
    #[builder(into)]
    #[serde(rename = "xsltTemplateNameForMacSec")]
    pub r#xslt_template_name_for_mac_sec: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouterConfigurationRouter {
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
                    "platform",
                    &self.r#platform,
                ),
                to_pulumi_object_field(
                    "router_type_identifier",
                    &self.r#router_type_identifier,
                ),
                to_pulumi_object_field(
                    "software",
                    &self.r#software,
                ),
                to_pulumi_object_field(
                    "vendor",
                    &self.r#vendor,
                ),
                to_pulumi_object_field(
                    "xslt_template_name",
                    &self.r#xslt_template_name,
                ),
                to_pulumi_object_field(
                    "xslt_template_name_for_mac_sec",
                    &self.r#xslt_template_name_for_mac_sec,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouterConfigurationRouter {
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
                    r#platform: {
                        let field_value = match fields_map.get("platform") {
                            Some(value) => value,
                            None => bail!("Missing field 'platform' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#router_type_identifier: {
                        let field_value = match fields_map.get("router_type_identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'router_type_identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#software: {
                        let field_value = match fields_map.get("software") {
                            Some(value) => value,
                            None => bail!("Missing field 'software' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vendor: {
                        let field_value = match fields_map.get("vendor") {
                            Some(value) => value,
                            None => bail!("Missing field 'vendor' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#xslt_template_name: {
                        let field_value = match fields_map.get("xslt_template_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'xslt_template_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#xslt_template_name_for_mac_sec: {
                        let field_value = match fields_map.get("xslt_template_name_for_mac_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'xslt_template_name_for_mac_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
