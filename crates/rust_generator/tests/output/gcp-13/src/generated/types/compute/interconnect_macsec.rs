#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InterconnectMacsec {
    /// If set to true, the Interconnect connection is configured with a should-secure
    /// MACsec security policy, that allows the Google router to fallback to cleartext
    /// traffic if the MKA session cannot be established. By default, the Interconnect
    /// connection is configured with a must-secure security policy that drops all traffic
    /// if the MKA session cannot be established with your router.
    #[builder(into)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Option<bool>,
    /// A keychain placeholder describing a set of named key objects along with their
    /// start times. A MACsec CKN/CAK is generated for each key in the key chain.
    /// Google router automatically picks the key with the most recent startTime when establishing
    /// or re-establishing a MACsec secure link.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "preSharedKeys")]
    pub r#pre_shared_keys: Vec<super::super::types::compute::InterconnectMacsecPreSharedKey>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InterconnectMacsec {
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
                    "fail_open",
                    &self.r#fail_open,
                ),
                to_pulumi_object_field(
                    "pre_shared_keys",
                    &self.r#pre_shared_keys,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InterconnectMacsec {
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
                    r#fail_open: {
                        let field_value = match fields_map.get("fail_open") {
                            Some(value) => value,
                            None => bail!("Missing field 'fail_open' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pre_shared_keys: {
                        let field_value = match fields_map.get("pre_shared_keys") {
                            Some(value) => value,
                            None => bail!("Missing field 'pre_shared_keys' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
