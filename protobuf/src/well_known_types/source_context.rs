// This file is generated by rust-protobuf 4.0.0-alpha.0. Do not edit
// .proto file is parsed by protoc --rust_out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `google/protobuf/source_context.proto`

///  `SourceContext` represents information about the source of a
///  protobuf element, like the file in which it is defined.
// @@protoc_insertion_point(message:google.protobuf.SourceContext)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SourceContext {
    // message fields
    ///  The path-qualified name of the .proto file that contained the associated
    ///  protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    // @@protoc_insertion_point(field:google.protobuf.SourceContext.file_name)
    pub file_name: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:google.protobuf.SourceContext.special_fields)
    pub special_fields: crate::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SourceContext {
    fn default() -> &'a SourceContext {
        <SourceContext as crate::Message>::default_instance()
    }
}

impl SourceContext {
    pub fn new() -> SourceContext {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> crate::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(crate::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "file_name",
            |m: &SourceContext| { &m.file_name },
            |m: &mut SourceContext| { &mut m.file_name },
        ));
        crate::reflect::GeneratedMessageDescriptorData::new_2::<SourceContext>(
            "SourceContext",
            fields,
            oneofs,
        )
    }
}

impl crate::Message for SourceContext {
    const NAME: &'static str = "SourceContext";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut crate::CodedInputStream<'_>) -> crate::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.file_name = is.read_string()?;
                },
                tag => {
                    crate::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.file_name.is_empty() {
            my_size += crate::rt::string_size(1, &self.file_name);
        }
        my_size += crate::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut crate::CodedOutputStream<'_>) -> crate::Result<()> {
        if !self.file_name.is_empty() {
            os.write_string(1, &self.file_name)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &crate::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut crate::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> SourceContext {
        SourceContext::new()
    }

    fn clear(&mut self) {
        self.file_name.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SourceContext {
        static instance: SourceContext = SourceContext {
            file_name: ::std::string::String::new(),
            special_fields: crate::SpecialFields::new(),
        };
        &instance
    }
}

impl crate::MessageFull for SourceContext {
    fn descriptor() -> crate::reflect::MessageDescriptor {
        static descriptor: crate::rt::Lazy<crate::reflect::MessageDescriptor> = crate::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SourceContext").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SourceContext {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        crate::text_format::fmt(self, f)
    }
}

impl crate::reflect::ProtobufValue for SourceContext {
    type RuntimeType = crate::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n$google/protobuf/source_context.proto\x12\x0fgoogle.protobuf\",\n\rSou\
    rceContext\x12\x1b\n\tfile_name\x18\x01\x20\x01(\tR\x08fileNameB\x8a\x01\
    \n\x13com.google.protobufB\x12SourceContextProtoP\x01Z6google.golang.org\
    /protobuf/types/known/sourcecontextpb\xa2\x02\x03GPB\xaa\x02\x1eGoogle.P\
    rotobuf.WellKnownTypesJ\xc1\x10\n\x06\x12\x04\x1e\0/\x01\n\xcc\x0c\n\x01\
    \x0c\x12\x03\x1e\0\x122\xc1\x0c\x20Protocol\x20Buffers\x20-\x20Google's\
    \x20data\x20interchange\x20format\n\x20Copyright\x202008\x20Google\x20In\
    c.\x20\x20All\x20rights\x20reserved.\n\x20https://developers.google.com/\
    protocol-buffers/\n\n\x20Redistribution\x20and\x20use\x20in\x20source\
    \x20and\x20binary\x20forms,\x20with\x20or\x20without\n\x20modification,\
    \x20are\x20permitted\x20provided\x20that\x20the\x20following\x20conditio\
    ns\x20are\n\x20met:\n\n\x20\x20\x20\x20\x20*\x20Redistributions\x20of\
    \x20source\x20code\x20must\x20retain\x20the\x20above\x20copyright\n\x20n\
    otice,\x20this\x20list\x20of\x20conditions\x20and\x20the\x20following\
    \x20disclaimer.\n\x20\x20\x20\x20\x20*\x20Redistributions\x20in\x20binar\
    y\x20form\x20must\x20reproduce\x20the\x20above\n\x20copyright\x20notice,\
    \x20this\x20list\x20of\x20conditions\x20and\x20the\x20following\x20discl\
    aimer\n\x20in\x20the\x20documentation\x20and/or\x20other\x20materials\
    \x20provided\x20with\x20the\n\x20distribution.\n\x20\x20\x20\x20\x20*\
    \x20Neither\x20the\x20name\x20of\x20Google\x20Inc.\x20nor\x20the\x20name\
    s\x20of\x20its\n\x20contributors\x20may\x20be\x20used\x20to\x20endorse\
    \x20or\x20promote\x20products\x20derived\x20from\n\x20this\x20software\
    \x20without\x20specific\x20prior\x20written\x20permission.\n\n\x20THIS\
    \x20SOFTWARE\x20IS\x20PROVIDED\x20BY\x20THE\x20COPYRIGHT\x20HOLDERS\x20A\
    ND\x20CONTRIBUTORS\n\x20\"AS\x20IS\"\x20AND\x20ANY\x20EXPRESS\x20OR\x20I\
    MPLIED\x20WARRANTIES,\x20INCLUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\
    \x20THE\x20IMPLIED\x20WARRANTIES\x20OF\x20MERCHANTABILITY\x20AND\x20FITN\
    ESS\x20FOR\n\x20A\x20PARTICULAR\x20PURPOSE\x20ARE\x20DISCLAIMED.\x20IN\
    \x20NO\x20EVENT\x20SHALL\x20THE\x20COPYRIGHT\n\x20OWNER\x20OR\x20CONTRIB\
    UTORS\x20BE\x20LIABLE\x20FOR\x20ANY\x20DIRECT,\x20INDIRECT,\x20INCIDENTA\
    L,\n\x20SPECIAL,\x20EXEMPLARY,\x20OR\x20CONSEQUENTIAL\x20DAMAGES\x20(INC\
    LUDING,\x20BUT\x20NOT\n\x20LIMITED\x20TO,\x20PROCUREMENT\x20OF\x20SUBSTI\
    TUTE\x20GOODS\x20OR\x20SERVICES;\x20LOSS\x20OF\x20USE,\n\x20DATA,\x20OR\
    \x20PROFITS;\x20OR\x20BUSINESS\x20INTERRUPTION)\x20HOWEVER\x20CAUSED\x20\
    AND\x20ON\x20ANY\n\x20THEORY\x20OF\x20LIABILITY,\x20WHETHER\x20IN\x20CON\
    TRACT,\x20STRICT\x20LIABILITY,\x20OR\x20TORT\n\x20(INCLUDING\x20NEGLIGEN\
    CE\x20OR\x20OTHERWISE)\x20ARISING\x20IN\x20ANY\x20WAY\x20OUT\x20OF\x20TH\
    E\x20USE\n\x20OF\x20THIS\x20SOFTWARE,\x20EVEN\x20IF\x20ADVISED\x20OF\x20\
    THE\x20POSSIBILITY\x20OF\x20SUCH\x20DAMAGE.\n\n\x08\n\x01\x02\x12\x03\
    \x20\0\x18\n\x08\n\x01\x08\x12\x03\"\0;\n\t\n\x02\x08%\x12\x03\"\0;\n\
    \x08\n\x01\x08\x12\x03#\0,\n\t\n\x02\x08\x01\x12\x03#\0,\n\x08\n\x01\x08\
    \x12\x03$\03\n\t\n\x02\x08\x08\x12\x03$\03\n\x08\n\x01\x08\x12\x03%\0\"\
    \n\t\n\x02\x08\n\x12\x03%\0\"\n\x08\n\x01\x08\x12\x03&\0!\n\t\n\x02\x08$\
    \x12\x03&\0!\n\x08\n\x01\x08\x12\x03'\0M\n\t\n\x02\x08\x0b\x12\x03'\0M\n\
    \x83\x01\n\x02\x04\0\x12\x04+\0/\x01\x1aw\x20`SourceContext`\x20represen\
    ts\x20information\x20about\x20the\x20source\x20of\x20a\n\x20protobuf\x20\
    element,\x20like\x20the\x20file\x20in\x20which\x20it\x20is\x20defined.\n\
    \n\n\n\x03\x04\0\x01\x12\x03+\x08\x15\n\xa3\x01\n\x04\x04\0\x02\0\x12\
    \x03.\x02\x17\x1a\x95\x01\x20The\x20path-qualified\x20name\x20of\x20the\
    \x20.proto\x20file\x20that\x20contained\x20the\x20associated\n\x20protob\
    uf\x20element.\x20\x20For\x20example:\x20`\"google/protobuf/source_conte\
    xt.proto\"`.\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03.\x02\x08\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03.\t\x12\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03.\x15\
    \x16b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static crate::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: crate::rt::Lazy<crate::descriptor::FileDescriptorProto> = crate::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        crate::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static crate::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: crate::rt::Lazy<crate::reflect::GeneratedFileDescriptor> = crate::rt::Lazy::new();
    static file_descriptor: crate::rt::Lazy<crate::reflect::FileDescriptor> = crate::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SourceContext::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            crate::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        crate::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
