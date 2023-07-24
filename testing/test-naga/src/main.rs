use std::error::Error;

use elysian::{core::ir::module::SpecializationData, naga::NagaWriter};
use naga::{
    back::{
        glsl::{Options as BackOptions, PipelineOptions, WriterFlags as GlslWriterFlags},
        wgsl::WriterFlags as WgslWriterFlags,
    },
    front::glsl::Options as FrontOptions,
    proc::{BoundsCheckPolicies, BoundsCheckPolicy},
    valid::{Capabilities, ValidationFlags},
    ShaderStage,
};

fn main() {
    env_logger::init();
    handle_result(main_impl())
}

fn main_impl() -> Result<(), Box<dyn Error>> {
    let foo = naga::front::glsl::Frontend::default()
        .parse(
            &FrontOptions {
                stage: ShaderStage::Fragment,
                defines: Default::default(),
            },
            r#"
struct Context {
    vec2 position_2d;
    float distance;
    vec2 gradient_2d;
};

Context point_distance_gradient_2d_position_2d(Context context) {
    context.distance = length(context.position_2d);
    context.gradient_2d = normalize(context.position_2d);
    return context;
}

Context field(Context context) {
    return point_distance_gradient_2d_position_2d(context);
}

void main() {
}
"#,
        )
        .unwrap();

    //panic!("{foo:#?}");

    let elysian_module = test_shapes::test_shape().module(&SpecializationData::new_2d());
    let naga_writer = NagaWriter::new(&elysian_module);
    let naga_module = naga_writer.module_to_naga();
    println!("{naga_module:#?}\n");

    let mut validator =
        naga::valid::Validator::new(ValidationFlags::all(), Capabilities::default());
    let module_info = validator.validate(&naga_module)?;

    let out = naga::back::wgsl::write_string(
        &naga_module,
        &module_info,
        WgslWriterFlags::EXPLICIT_TYPES,
    )?;

    let mut buf = String::default();

    let options = BackOptions {
        version: naga::back::glsl::Version::Embedded {
            version: 310,
            is_webgl: true,
        },
        writer_flags: GlslWriterFlags::INCLUDE_UNUSED_ITEMS,
        binding_map: Default::default(),
        zero_initialize_workgroup_memory: false,
    };

    let pipeline_options = PipelineOptions {
        shader_stage: ShaderStage::Fragment,
        entry_point: "mainImage".to_string(),
        multiview: None,
    };

    let mut writer = naga::back::glsl::Writer::new(
        &mut buf,
        &naga_module,
        &module_info,
        &options,
        &pipeline_options,
        BoundsCheckPolicies {
            index: BoundsCheckPolicy::Unchecked,
            buffer: BoundsCheckPolicy::Unchecked,
            image_load: BoundsCheckPolicy::Unchecked,
            image_store: BoundsCheckPolicy::Unchecked,
            binding_array: BoundsCheckPolicy::Unchecked,
        },
    )?;

    writer.write()?;

    println!("Output:\n{buf:}");

    Ok(())
}

fn handle_result<T>(result: Result<T, Box<dyn Error>>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => {
            handle_error(e.as_ref());
            eprintln!();
            panic!("{e:#?}")
        }
    }
}

fn handle_error(e: &dyn Error) {
    log::error!("{e:}");
    if let Some(source) = e.source() {
        handle_error(source);
    }
}
