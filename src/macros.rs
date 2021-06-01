//! Convenience macros

/// Macro to produce an array of [VertexAttribute](crate::VertexAttribute).
///
/// Output has type: `[VertexAttribute; _]`. Usage is as follows:
/// ```
/// # use wgpu::vertex_attr_array;
/// let attrs = vertex_attr_array![0 => Float32x2, 1 => Float32, 2 => Uint16x4];
/// ```
/// This example specifies a list of three [VertexAttribute](crate::VertexAttribute),
/// each with the given `shader_location` and `format`.
/// Offsets are calculated automatically.
#[macro_export]
macro_rules! vertex_attr_array {
    ($($loc:expr => $fmt:ident),* $(,)?) => {
        $crate::vertex_attr_array!([] ; 0; $($loc => $fmt ,)*)
    };
    ([$($t:expr,)*] ; $off:expr ;) => { [$($t,)*] };
    ([$($t:expr,)*] ; $off:expr ; $loc:expr => $item:ident, $($ll:expr => $ii:ident ,)*) => {
        $crate::vertex_attr_array!(
            [$($t,)*
            $crate::VertexAttribute {
                format: $crate::VertexFormat :: $item,
                offset: $off,
                shader_location: $loc,
            },];
            $off + $crate::VertexFormat :: $item.size();
            $($ll => $ii ,)*
        )
    };
}

#[test]
fn test_vertex_attr_array() {
    let attrs = vertex_attr_array![0 => Float32x2, 3 => Uint16x4];
    // VertexAttribute does not support PartialEq, so we cannot test directly
    assert_eq!(attrs.len(), 2);
    assert_eq!(attrs[0].offset, 0);
    assert_eq!(attrs[0].shader_location, 0);
    assert_eq!(attrs[1].offset, std::mem::size_of::<(f32, f32)>() as u64);
    assert_eq!(attrs[1].shader_location, 3);
}

/// Macro to load a SPIR-V module statically.
///
/// It ensures the word alignment as well as the magic number.
#[macro_export]
macro_rules! include_spirv {
    ($($token:tt)*) => {
        {
            //log::info!("including '{}'", $($token)*);
            $crate::ShaderModuleDescriptor {
                label: Some($($token)*),
                source: $crate::util::make_spirv(include_bytes!($($token)*)),
                flags: $crate::ShaderFlags::VALIDATION,
            }
        }
    };
}

/// Macro to load a WGSL module statically.
#[macro_export]
macro_rules! include_wgsl {
    ($($token:tt)*) => {
        {
            //log::info!("including '{}'", $($token)*);
            $crate::ShaderModuleDescriptor {
                label: Some($($token)*),
                source: $crate::ShaderSource::Wgsl(include_str!($($token)*).into()),
                flags: $crate::ShaderFlags::VALIDATION,
            }
        }
    };
}

#[test]
pub fn test_include_wgsl() {
    let macro_desc = include_wgsl!("../examples/hello-triangle/shader.wgsl");
    let struct_desc = crate::ShaderModuleDescriptor {
        label: Some("../examples/hello-triangle/shader.wgsl"),
        source: crate::ShaderSource::Wgsl(
            include_str!("../examples/hello-triangle/shader.wgsl").into(),
        ),
        flags: crate::ShaderFlags::VALIDATION,
    };

    // ShaderModuleDescriptor does not support PartialEq, so we cannot test directly
    assert_eq!(macro_desc.label, struct_desc.label);
    assert_eq!(
        match macro_desc.source {
            crate::ShaderSource::Wgsl(source) => source,
            crate::ShaderSource::SpirV(_) => panic!(),
        },
        match struct_desc.source {
            crate::ShaderSource::Wgsl(source) => source,
            crate::ShaderSource::SpirV(_) => panic!(),
        }
    );
    assert_eq!(macro_desc.flags, struct_desc.flags);
}
