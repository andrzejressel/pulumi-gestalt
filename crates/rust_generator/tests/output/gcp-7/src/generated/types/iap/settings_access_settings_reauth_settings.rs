#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SettingsAccessSettingsReauthSettings {
    /// Reauth session lifetime, how long before a user has to reauthenticate again.
    /// A duration in seconds with up to nine fractional digits, ending with 's'.
    /// Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: String,
    /// Reauth method requested. The possible values are:
    /// * `LOGIN`: Prompts the user to log in again.
    /// * `SECURE_KEY`: User must use their secure key 2nd factor device.
    /// * `ENROLLED_SECOND_FACTORS`: User can use any enabled 2nd factor.
    /// Possible values are: `LOGIN`, `SECURE_KEY`, `ENROLLED_SECOND_FACTORS`.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: String,
    /// How IAP determines the effective policy in cases of hierarchical policies.
    /// Policies are merged from higher in the hierarchy to lower in the hierarchy.
    /// The possible values are:
    /// * `MINIMUM`: This policy acts as a minimum to other policies, lower in the hierarchy.
    /// Effective policy may only be the same or stricter.
    /// * `DEFAULT`: This policy acts as a default if no other reauth policy is set.
    /// Possible values are: `MINIMUM`, `DEFAULT`.
    #[builder(into)]
    #[serde(rename = "policyType")]
    pub r#policy_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SettingsAccessSettingsReauthSettings {
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
                    "max_age",
                    &self.r#max_age,
                ),
                to_pulumi_object_field(
                    "method",
                    &self.r#method,
                ),
                to_pulumi_object_field(
                    "policy_type",
                    &self.r#policy_type,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SettingsAccessSettingsReauthSettings {
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
                    r#max_age: {
                        let field_value = match fields_map.get("max_age") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_age' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#method: {
                        let field_value = match fields_map.get("method") {
                            Some(value) => value,
                            None => bail!("Missing field 'method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_type: {
                        let field_value = match fields_map.get("policy_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
