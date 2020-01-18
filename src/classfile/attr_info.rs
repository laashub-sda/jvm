use crate::classfile::types::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum AttrType {
    Invalid,
    ConstantValue {
        constant_value_index: U2,
    },
    Code(Code),
    //    StackMapTable,
    Exceptions {
        exceptions: Vec<U2>,
    },
    InnerClasses {
        classes: Vec<InnerClass>,
    },
    EnclosingMethod {
        class_index: U2,
        method_index: U2,
    },
    Synthetic,
    Signature {
        signature_index: U2,
    },
    SourceFile {
        source_file_index: U2,
    },
    SourceDebugExtension {
        debug_extension: Arc<Vec<U1>>
    },
    LineNumberTable {
        tables: Vec<LineNumber>,
    },
    LocalVariableTable {
        tables: Vec<LocalVariable>,
    },
    LocalVariableTypeTable {
        tables: Vec<LocalVariable>,
    },
    Deprecated,
    RuntimeVisibleAnnotations {
        annotations: Vec<AnnotationEntry>,
    },
    RuntimeInvisibleAnnotations {
        annotations: Vec<AnnotationEntry>,
    },
    RuntimeVisibleParameterAnnotations {
        annotations: Vec<AnnotationEntry>,
    },
    RuntimeInvisibleParameterAnnotations {
        annotations: Vec<AnnotationEntry>,
    },
//        RuntimeVisibleTypeAnnotations,
//        RuntimeInvisibleTypeAnnotations,
    AnnotationDefault {
        default_value: ElementValueType,
    },
    BootstrapMethods {
        n: U2,
        methods: Vec<BootstrapMethod>,
    },
    MethodParameters {
        parameters: Vec<MethodParameter>,
    },
    Unknown,
}

pub enum AttrTag {
    Invalid,
    ConstantValue,
    Code,
    //    StackMapTable,
    Exceptions,
    InnerClasses,
    EnclosingMethod,
    Synthetic,
    Signature,
    SourceFile,
    SourceDebugExtension,
    LineNumberTable,
    LocalVariableTable,
    LocalVariableTypeTable,
    Deprecated,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    //    RuntimeVisibleTypeAnnotations,
    //    RuntimeInvisibleTypeAnnotations,
    AnnotationDefault,
    BootstrapMethods,
    MethodParameters,
    Unknown,
}

impl From<&[u8]> for AttrTag {
    fn from(raw: &[u8]) -> Self {
        match raw {
            b"ConstantValue" => AttrTag::ConstantValue,
            b"Code" => AttrTag::Code,
            //            b"StackMapTable" => AttributeTag::StackMapTable,
            b"Exceptions" => AttrTag::Exceptions,
            b"InnerClasses" => AttrTag::InnerClasses,
            b"EnclosingMethod" => AttrTag::EnclosingMethod,
            b"Synthetic" => AttrTag::Synthetic,
            b"Signature" => AttrTag::Signature,
            b"SourceFile" => AttrTag::SourceFile,
            b"SourceDebugExtension" => AttrTag::SourceDebugExtension,
            b"LineNumberTable" => AttrTag::LineNumberTable,
            b"LocalVariableTable" => AttrTag::LocalVariableTable,
            b"LocalVariableTypeTable" => AttrTag::LocalVariableTypeTable,
            b"Deprecated" => AttrTag::Deprecated,
            b"RuntimeVisibleAnnotations" => AttrTag::RuntimeVisibleAnnotations,
            b"RuntimeInvisibleAnnotations" => AttrTag::RuntimeInvisibleAnnotations,
            b"RuntimeVisibleParameterAnnotations" => AttrTag::RuntimeVisibleParameterAnnotations,
            b"RuntimeInvisibleParameterAnnotations" => {
                AttrTag::RuntimeInvisibleParameterAnnotations
            }
//            b"RuntimeVisibleTypeAnnotations" => AttributeTag::RuntimeVisibleTypeAnnotations,
//            b"RuntimeInvisibleTypeAnnotations" => AttributeTag::RuntimeInvisibleTypeAnnotations,
            b"AnnotationDefault" => AttrTag::AnnotationDefault,
            b"BootstrapMethods" => AttrTag::BootstrapMethods,
            b"MethodParameters" => AttrTag::MethodParameters,
            _ => {
                warn!("Unknown attr {}", String::from_utf8_lossy(raw));
                AttrTag::Unknown
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Code {
    pub max_stack: U2,
    pub max_locals: U2,
    pub code: Arc<Vec<U1>>,
    pub exceptions: Vec<CodeException>,
    pub attrs: Vec<AttrType>
}

#[derive(Debug, Clone)]
pub struct CodeException {
    pub start_pc: U2,
    pub end_pc: U2,
    pub handler_pc: U2,
    pub catch_type: U2,
}

impl CodeException {
    pub fn contains(&self, pc: U2) -> bool {
        (self.start_pc..self.end_pc).contains(&pc)
    }

    pub fn is_finally(&self) -> bool {
        self.catch_type == 0
    }
}

#[derive(Debug, Clone)]
pub struct AttributeInfo {
    pub name_index: U2,
    pub length: U4,
    pub info: Vec<U1>
}

pub enum NestedClassAccessPropertyFlag {
    AccPublic,
    AccPrivate,
    AccProtected,
    AccStatic,
    AccFinal,
    AccInterface,
    AccAbstract,
    AccSynthetic,
    AccAnnotation,
    AccEnum,
}

#[derive(Debug, Copy, Clone)]
pub struct InnerClass {
    pub inner_class_info_index: U2,
    pub outer_class_info_index: U2,
    pub inner_name_index: U2,
    pub inner_class_access_flags: U2,
}

#[derive(Debug, Copy, Clone)]
pub struct LineNumber {
    pub start_pc: U2,
    pub number: U2,
}

#[derive(Debug, Copy, Clone)]
pub struct LocalVariable {
    pub start_pc: U2,
    pub length: U2,
    pub name_index: U2,
    pub signature_index: U2,
    pub index: U2,
}

#[derive(Debug)]
pub enum ElementValueTag {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    String,
    Enum,
    Class,
    Annotation,
    Array,
    Unknown,
}

impl From<u8> for ElementValueTag {
    fn from(v: u8) -> Self {
        match v {
            b'B' => ElementValueTag::Byte,
            b'C' => ElementValueTag::Char,
            b'D' => ElementValueTag::Double,
            b'F' => ElementValueTag::Float,
            b'I' => ElementValueTag::Int,
            b'J' => ElementValueTag::Long,
            b'S' => ElementValueTag::Short,
            b'Z' => ElementValueTag::Boolean,
            b's' => ElementValueTag::String,
            b'e' => ElementValueTag::Enum,
            b'c' => ElementValueTag::Class,
            b'@' => ElementValueTag::Annotation,
            b'[' => ElementValueTag::Array,
            _ => ElementValueTag::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ElementValueType {
    Byte {
        tag: U1,
        val_index: U2,
    },
    Char {
        tag: U1,
        val_index: U2,
    },
    Double {
        tag: U1,
        val_index: U2,
    },
    Float {
        tag: U1,
        val_index: U2,
    },
    Int {
        tag: U1,
        val_index: U2,
    },
    Long {
        tag: U1,
        val_index: U2,
    },
    Short {
        tag: U1,
        val_index: U2,
    },
    Boolean {
        tag: U1,
        val_index: U2,
    },
    String {
        tag: U1,
        val_index: U2,
    },
    Enum {
        tag: U1,
        type_index: U2,
        val_index: U2,
    },
    Class {
        tag: U1,
        index: U2,
    },
    Annotation(AnnotationElementValue),
    Array {
        n: U2,
        values: Vec<ElementValueType>,
    },
    Unknown,
}

#[derive(Debug, Clone)]
pub struct AnnotationElementValue {
    pub value: AnnotationEntry,
}

#[derive(Debug, Clone)]
pub struct ElementValuePair {
    pub name_index: U2,
    pub value: ElementValueType,
}

#[derive(Debug, Clone)]
pub struct AnnotationEntry {
    pub type_index: U2,
    pub pairs: Vec<ElementValuePair>,
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub method_ref: U2,
    pub args: Vec<U2>,
}

#[derive(Debug)]
pub enum MethodParameterAccessFlag {
    AccFinal = 0x0010,
    AccSynthetic = 0x1000,
    AccMandated = 0x8000,
}

#[derive(Debug, Copy, Clone)]
pub struct MethodParameter {
    pub name_index: U2,
    pub acc_flags: U2,
}

