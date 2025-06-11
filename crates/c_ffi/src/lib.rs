use anyhow::Context;
use pulumi_gestalt_rust_integration as integration;
use std::cell::RefCell;
use std::ffi::{CStr, CString, c_char, c_void};
use std::ptr::null_mut;
use std::rc::{Rc, Weak};

pub struct CustomOutputId {
    native: integration::Output,
    ctx: Weak<RefCell<InnerPulumiContext>>,
}

pub struct CustomCompositeOutputId {
    native: integration::CompositeOutput,
    ctx: Weak<RefCell<InnerPulumiContext>>,
}

pub struct InnerPulumiContext {
    ctx: integration::Context,
    outputs: Vec<*mut CustomOutputId>,
    context: *const c_void,
}

pub struct PulumiContext {
    inner: Rc<RefCell<InnerPulumiContext>>,
}

#[repr(C)]
pub enum ConfigValue {
    PlainValue(*mut c_char),
    Secret(*mut CustomOutputId),
}

#[repr(C)]
pub struct ObjectField {
    name: *const c_char,
    value: *const CustomOutputId,
}

#[repr(C)]
pub struct ResultField {
    name: *const c_char,
}

#[repr(C)]
pub struct RegisterResourceRequest {
    type_: *const c_char,
    name: *const c_char,
    version: *const c_char,
    inputs: *const ObjectField,
    inputs_len: usize,
}

#[repr(C)]
pub struct InvokeResourceRequest {
    token: *const c_char,
    version: *const c_char,
    inputs: *const ObjectField,
    inputs_len: usize,
}

/// String that may contain nulls and is not null-terminated.
#[repr(C)]
pub struct CFFIString {
    data: *mut u8,
    len: usize,
}

impl CFFIString {
    pub fn from_string(data: Vec<u8>) -> Self {
        let len = data.len();
        let ptr = data.as_ptr() as *mut u8;
        std::mem::forget(data); // Prevent Rust from freeing the memory
        CFFIString { data: ptr, len }
    }
}

impl Drop for CFFIString {
    fn drop(&mut self) {
        unsafe {
            let _ = Vec::from_raw_parts(self.data, self.len, self.len);
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_free_string(value: *mut CFFIString) {
    let _ = unsafe { Box::from_raw(value) };
}

/// Arguments: Engine context, Function context, Serialized JSON value
/// Returned string must represent a JSON value;
/// Library will free the returned string
type MappingFunction = extern "C" fn(*const c_void, *const c_void, *const c_char) -> *mut c_char;

#[unsafe(no_mangle)]
extern "C" fn pulumi_create_context(context: *const c_void) -> *mut PulumiContext {
    let engine = integration::Context::create_context();
    let t = InnerPulumiContext {
        ctx: engine,
        outputs: Vec::new(),
        context,
    };
    let t = PulumiContext {
        inner: Rc::new(RefCell::new(t)),
    };
    Box::into_raw(Box::new(t))
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_finish(ctx: *mut PulumiContext) {
    let pulumi_context = unsafe { &mut *ctx };
    pulumi_context.inner.borrow_mut().ctx.finish();
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_destroy_context(ctx: *mut PulumiContext) {
    unsafe {
        let b = Box::from_raw(ctx);
        for output in b.inner.borrow_mut().outputs.iter() {
            let _ = Box::from_raw(*output);
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_create_output(
    ctx: *mut PulumiContext,
    value: *const c_char,
    secret: bool,
) -> *mut CustomOutputId {
    let value = unsafe { CStr::from_ptr(value) }
        .to_str()
        .unwrap()
        .to_string();
    let pulumi_context = unsafe { &mut *ctx };
    let mut inner_engine = pulumi_context.inner.borrow_mut();
    let output_id = inner_engine.ctx.create_output(value, secret);
    let output = CustomOutputId {
        native: output_id,
        ctx: Rc::downgrade(&pulumi_context.inner),
    };
    let raw = Box::into_raw(Box::new(output));
    inner_engine.outputs.push(raw);
    raw
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_register_resource(
    ctx: *mut PulumiContext,
    request: *const RegisterResourceRequest,
) -> *mut CustomCompositeOutputId {
    let pulumi_context = unsafe { &mut *ctx };
    let request = unsafe { &*request };

    let type_ = unsafe { CStr::from_ptr(request.type_) }
        .to_str()
        .unwrap()
        .to_owned();

    let name = unsafe { CStr::from_ptr(request.name) }
        .to_str()
        .unwrap()
        .to_owned();

    let version = unsafe { CStr::from_ptr(request.version) }
        .to_str()
        .unwrap()
        .to_owned();

    let objects = extract_field(request.inputs, request.inputs_len);

    let inner = &pulumi_context.inner;
    let inner_engine = pulumi_context.inner.borrow_mut();
    let request = integration::RegisterResourceRequest {
        type_,
        name,
        inputs: &objects,
        version,
    };
    let output_id = inner_engine.ctx.register_resource(request);

    let output = CustomCompositeOutputId {
        native: output_id,
        ctx: Rc::downgrade(inner),
    };

    // inner_engine.outputs.push(raw); //FIXME
    Box::into_raw(Box::new(output))
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_invoke_resource(
    ctx: *mut PulumiContext,
    request: *const InvokeResourceRequest,
) -> *mut CustomCompositeOutputId {
    let pulumi_context = unsafe { &mut *ctx };
    let request = unsafe { &*request };

    let token = unsafe { CStr::from_ptr(request.token) }
        .to_str()
        .unwrap()
        .to_owned();

    let version = unsafe { CStr::from_ptr(request.version) }
        .to_str()
        .unwrap()
        .to_owned();

    let objects = extract_field(request.inputs, request.inputs_len);

    let inner = &pulumi_context.inner;
    let inner_engine = pulumi_context.inner.borrow_mut();
    let request = integration::InvokeResourceRequest {
        token,
        inputs: &objects,
        version,
    };
    let output_id = inner_engine.ctx.invoke_resource(request);

    let output = CustomCompositeOutputId {
        native: output_id,
        ctx: Rc::downgrade(inner),
    };

    // inner_engine.outputs.push(raw); //FIXME
    Box::into_raw(Box::new(output))
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_output_map(
    ctx: *mut PulumiContext,
    output: *const CustomOutputId,
    function_context: *const c_void,
    function: MappingFunction,
) -> *mut CustomOutputId {
    let output = unsafe { &*output };
    let engine = unsafe { &mut *ctx };
    let mut inner_engine = engine.inner.borrow_mut();
    let context = inner_engine.context;

    let f = move |value: String| {
        let c_string = CString::new(value).unwrap();
        let str = function(context, function_context, c_string.as_ptr());
        let result = unsafe { CStr::from_ptr(str) }.to_str().unwrap();
        let v = result.to_owned();
        unsafe {
            libc::free(str as *mut c_void);
        }
        v
    };

    let second_output = output.native.map(Box::new(f));

    let output = CustomOutputId {
        native: second_output,
        ctx: Rc::downgrade(&engine.inner),
    };
    let raw = Box::into_raw(Box::new(output));
    inner_engine.outputs.push(raw);
    raw
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_output_combine(
    output: *const CustomOutputId,
    outputs: *const *const CustomOutputId,
    outputs_size: usize,
) -> *mut CustomOutputId {
    let output = unsafe { &*output };
    // let mut inner_engine = output.native.inner.borrow_mut();

    let mut other_outputs = Vec::new();
    unsafe {
        std::slice::from_raw_parts(outputs, outputs_size)
            .iter()
            .for_each(|field| {
                let field = *field;
                other_outputs.push(&(*field).native);
            });
    }

    let binding = output.ctx.upgrade().unwrap();
    let mut engine = binding.borrow_mut();

    let new_output = output.native.combine(&other_outputs);

    let output = CustomOutputId {
        native: new_output,
        ctx: output.ctx.clone(),
    };

    let raw = Box::into_raw(Box::new(output));
    engine.outputs.push(raw);
    raw
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_output_add_to_export(value: *const CustomOutputId, name: *const c_char) {
    let name = unsafe { CStr::from_ptr(name) }
        .to_str()
        .unwrap()
        .to_string();
    let value = unsafe { &*value };
    value.native.add_export(name);
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_composite_output_get_field(
    output: *mut CustomCompositeOutputId,
    field_name: *const c_char,
) -> *mut CustomOutputId {
    let field_name = unsafe { CStr::from_ptr(field_name) }
        .to_str()
        .unwrap()
        .to_string();
    let custom_register_output_id = unsafe { &*output };
    let new_output = custom_register_output_id.native.get_field(field_name);

    let binding = custom_register_output_id.ctx.upgrade().unwrap();
    let mut engine = binding.borrow_mut();

    let output = CustomOutputId {
        native: new_output,
        ctx: Rc::downgrade(&binding),
    };
    let raw = Box::into_raw(Box::new(output));
    engine.outputs.push(raw);
    raw
}

/// Receives value from configuration
/// `name`: Configuration bag's logical name. If null, the default (project name) is used.
/// `key`: Config key. Cannot be null
///
/// Returns null when the value is not found
#[unsafe(no_mangle)]
extern "C" fn pulumi_config_get_value(
    ctx: *mut PulumiContext,
    name: *const c_char,
    key: *const c_char,
) -> *mut ConfigValue {
    let engine = unsafe { &mut *ctx };

    let name = if (name as *const c_void).is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(name) }.to_str().unwrap())
    };
    let key = unsafe { CStr::from_ptr(key) }.to_str().unwrap();

    let inner_engine = engine.inner.borrow_mut();

    match inner_engine.ctx.get_config_value(name, key) {
        None => null_mut(),
        Some(pulumi_gestalt_rust_integration::ConfigValue::PlainText(s)) => {
            let value = CString::new(s).unwrap();
            let config_value = ConfigValue::PlainValue(value.into_raw());
            Box::into_raw(Box::new(config_value))
        }
        Some(pulumi_gestalt_rust_integration::ConfigValue::Secret(output)) => {
            let output = CustomOutputId {
                native: output,
                ctx: Rc::downgrade(&engine.inner),
            };
            let output = Box::into_raw(Box::new(output));
            let config_value = ConfigValue::Secret(output);
            Box::into_raw(Box::new(config_value))
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn pulumi_config_free(value: *mut ConfigValue) {
    unsafe {
        match &*value {
            ConfigValue::PlainValue(s) => {
                let _ = CString::from_raw(*s);
            }
            ConfigValue::Secret(output) => {
                let _ = Box::from_raw(*output);
            }
        }
        let _ = Box::from_raw(value);
    }
}

/// Returns protobuf encoded schema for the provider.
/// Modules for provider can be found in Pulumi registry on left side with (M) icon:
/// - [AWS](https://www.pulumi.com/registry/packages/aws/)
/// - [Azure](https://www.pulumi.com/registry/packages/azure/)
/// - [GCP](https://www.pulumi.com/registry/packages/gcp/)
///
/// Empty modules list means that no modules are used.
/// To use all modules, pass null for the modules pointer and 0 for the size.
#[unsafe(no_mangle)]
extern "C" fn pulumi_get_schema(
    provider_name: *const c_char,
    provider_version: *const c_char,
    modules: *const *const c_char,
    modules_size: usize,
) -> *mut CFFIString {
    let modules = if modules.is_null() {
        if modules_size > 0 {
            panic!("Modules pointer is null but size is greater than 0");
        }
        None
    } else {
        Some(extract_string_vec(modules, modules_size))
    };

    let provider_name = unsafe { CStr::from_ptr(provider_name) }.to_str().unwrap();
    let provider_version = unsafe { CStr::from_ptr(provider_version) }
        .to_str()
        .unwrap();

    let schema = integration::get_schema(provider_name, provider_version, modules.as_deref())
        .context("Failed to get schema")
        .unwrap();

    let protobuf = pulumi_gestalt_schema_protobuf::convert_to_protobuf(&schema)
        .context("Failed to convert schema to protobuf")
        .unwrap();

    Box::into_raw(Box::new(CFFIString::from_string(protobuf)))
}

fn extract_field<'a>(
    inputs: *const ObjectField,
    inputs_len: usize,
) -> Vec<integration::ObjectField<'a>> {
    let mut objects = Vec::new();
    unsafe {
        std::slice::from_raw_parts(inputs, inputs_len)
            .iter()
            .for_each(|field| {
                let name = CStr::from_ptr(field.name).to_str().unwrap().to_owned();
                let output = &(*field.value).native;
                objects.push(integration::ObjectField {
                    name,
                    value: output,
                });
            });
    }
    objects
}

fn extract_string_vec<'a>(inputs: *const *const c_char, inputs_len: usize) -> Vec<&'a str> {
    unsafe {
        std::slice::from_raw_parts(inputs, inputs_len)
            .iter()
            .map(|&s| CStr::from_ptr(s).to_str().unwrap())
            .collect()
    }
}
