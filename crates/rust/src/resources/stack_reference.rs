use crate::{
    Context, CustomResourceOptions, InputOrOutput, Output, PulumiAny, RegisterResourceRequest,
    ResourceRequestObjectField,
};
use bon::Builder;

/// Input arguments used to create a [`StackReference`] resource.
#[derive(Builder)]
#[builder(finish_fn = build_struct)]
pub struct StackReferenceArgs {
    /// The name of the stack to reference in the form `organization/project/stack`.
    #[builder(into)]
    pub name: InputOrOutput<String>,
}

/// Output object returned from stack reference creation.
pub struct StackReferenceResult {
    /// Pulumi ID is the provider-assigned unique ID for this managed resource.
    /// It is set during deployments and may be missing (unknown) during planning phases.
    pub id: Output<String>,
    /// Pulumi URN is the stable logical identity of this resource in the Pulumi stack.
    pub urn: Output<String>,
    /// All outputs from the referenced stack as a map.
    pub outputs: Output<PulumiAny>,
}

impl StackReferenceResult {
    /// Returns the named output from the referenced stack as an [`Output<PulumiAny>`].
    ///
    /// If the given name is not present in the referenced stack, the output resolves to `null`.
    pub fn get_output(&self, key: &str) -> Output<PulumiAny> {
        let key = key.to_string();
        self.outputs.map(move |map| match map.0 {
            serde_json::Value::Object(ref obj) => obj
                .get(&key)
                .cloned()
                .map(PulumiAny)
                .unwrap_or(PulumiAny(serde_json::Value::Null)),
            _ => PulumiAny(serde_json::Value::Null),
        })
    }
}

/// Registers a new stack reference resource with the given unique name and arguments.
pub fn create(ctx: &Context, name: &str, args: StackReferenceArgs) -> StackReferenceResult {
    __create(ctx, name, args, None)
}

/// Same as [`create`], but with additional generic options that control registration behavior.
pub fn create_with_options(
    ctx: &Context,
    name: &str,
    args: StackReferenceArgs,
    options: CustomResourceOptions,
) -> StackReferenceResult {
    __create(ctx, name, args, Some(options))
}

fn __create(
    ctx: &Context,
    name: &str,
    args: StackReferenceArgs,
    options: Option<CustomResourceOptions>,
) -> StackReferenceResult {
    let name_binding = args.name.get_output(ctx);
    let request = RegisterResourceRequest {
        type_: "pulumi:pulumi:StackReference".into(),
        name: name.to_string(),
        version: String::new(),
        object: &[ResourceRequestObjectField {
            name: "name".into(),
            value: &name_binding.drop_type(),
        }],
        options,
    };
    let composite = ctx.register_resource(request);

    StackReferenceResult {
        id: composite.get_id(),
        urn: composite.get_urn(),
        outputs: composite.get_field("outputs"),
    }
}
