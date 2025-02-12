use std::str::FromStr;

use crate::error::{Result, ResymCoreError};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PrimitiveReconstructionFlavor {
    Portable,
    Microsoft,
    Raw,
    Msvc,
}

impl FromStr for PrimitiveReconstructionFlavor {
    type Err = ResymCoreError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "portable" => Ok(PrimitiveReconstructionFlavor::Portable),
            "ms" | "msft" | "microsoft" => Ok(PrimitiveReconstructionFlavor::Microsoft),
            "raw" => Ok(PrimitiveReconstructionFlavor::Raw),
            "msvc" => Ok(PrimitiveReconstructionFlavor::Msvc),
            _ => Err(ResymCoreError::ParsePrimitiveFlavorError(s.to_owned())),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AccessSpecifierReconstructionFlavor {
    Disabled,
    Always,
    Automatic,
}

impl FromStr for AccessSpecifierReconstructionFlavor {
    type Err = ResymCoreError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "disabled" | "false" => Ok(AccessSpecifierReconstructionFlavor::Disabled),
            "always" | "true" => Ok(AccessSpecifierReconstructionFlavor::Always),
            "automatic" => Ok(AccessSpecifierReconstructionFlavor::Automatic),
            _ => Err(ResymCoreError::ParsePrimitiveFlavorError(s.to_owned())),
        }
    }
}

pub fn include_headers_for_flavor(
    flavor: PrimitiveReconstructionFlavor,
    ignore_std_types: bool,
) -> String {
    let flavor_header = match flavor {
        PrimitiveReconstructionFlavor::Portable => "#include <cstdint>\n",
        PrimitiveReconstructionFlavor::Microsoft => "#include <Windows.h>\n",
        PrimitiveReconstructionFlavor::Raw => "",
        PrimitiveReconstructionFlavor::Msvc => "",
    };

    let common_std_headers = if ignore_std_types {
        concat!(
            "#include <array>\n",
            "#include <list>\n",
            "#include <map>\n",
            "#include <memory>\n",
            "#include <string>\n",
            "#include <unordered_map>\n",
            "#include <utility>\n",
            "#include <vector>\n",
        )
    } else {
        ""
    };

    format!("{flavor_header}{common_std_headers}")
}

pub fn primitive_kind_as_str(
    flavor: &PrimitiveReconstructionFlavor,
    primitive_kind: pdb::PrimitiveKind,
    indirection: bool,
) -> Result<String> {
    match flavor {
        PrimitiveReconstructionFlavor::Portable => {
            primitive_kind_as_str_portable(primitive_kind, indirection)
        }
        PrimitiveReconstructionFlavor::Microsoft => {
            primitive_kind_as_str_microsoft(primitive_kind, indirection)
        }
        PrimitiveReconstructionFlavor::Raw => {
            primitive_kind_as_str_raw(primitive_kind, indirection)
        }
        PrimitiveReconstructionFlavor::Msvc => {
            primitive_kind_as_str_msvc(primitive_kind, indirection)
        }
    }
}

fn primitive_kind_as_str_portable(
    primitive_kind: pdb::PrimitiveKind,
    indirection: bool,
) -> Result<String> {
    let str_representation = match primitive_kind {
        pdb::PrimitiveKind::Void => Ok("void"),
        pdb::PrimitiveKind::Char | pdb::PrimitiveKind::RChar => Ok("char"),
        pdb::PrimitiveKind::UChar => Ok("unsigned char"),
        pdb::PrimitiveKind::WChar => Ok("wchar_t"),
        pdb::PrimitiveKind::RChar16 => Ok("char16_t"),
        pdb::PrimitiveKind::RChar32 => Ok("char32_t"),
        pdb::PrimitiveKind::Char8 => Ok("char8_t"),

        pdb::PrimitiveKind::I8 => Ok("int8_t"),
        pdb::PrimitiveKind::U8 => Ok("uint8_t"),
        pdb::PrimitiveKind::I16 | pdb::PrimitiveKind::Short => Ok("int16_t"),
        pdb::PrimitiveKind::U16 | pdb::PrimitiveKind::UShort => Ok("uint16_t"),
        pdb::PrimitiveKind::I32 | pdb::PrimitiveKind::Long => Ok("int32_t"),
        pdb::PrimitiveKind::U32 | pdb::PrimitiveKind::ULong => Ok("uint32_t"),
        pdb::PrimitiveKind::I64 | pdb::PrimitiveKind::Quad => Ok("int64_t"),
        pdb::PrimitiveKind::U64 | pdb::PrimitiveKind::UQuad => Ok("uint64_t"),

        pdb::PrimitiveKind::F32 => Ok("float"),
        pdb::PrimitiveKind::F64 => Ok("double"),

        pdb::PrimitiveKind::Bool8 => Ok("bool"),
        pdb::PrimitiveKind::Bool32 => Ok("int32_t"),

        // Microsoft-specific, usually implemented as "long"
        pdb::PrimitiveKind::HRESULT => Ok("int32_t"),

        // TODO: Seems valid for C++ method parameters. Are there other
        // cases of legitimate "NoType" occurences?
        pdb::PrimitiveKind::NoType => Ok("..."),

        _ => Err(ResymCoreError::NotImplementedError(format!(
            "/* FIXME: Unhandled primitive kind: '{primitive_kind:?}' */ void"
        ))),
    };

    let mut string_representation = str_representation?.to_string();
    if indirection {
        string_representation.push('*');
    }

    Ok(string_representation)
}

fn primitive_kind_as_str_microsoft(
    primitive_kind: pdb::PrimitiveKind,
    indirection: bool,
) -> Result<String> {
    let str_representation = match primitive_kind {
        pdb::PrimitiveKind::Void => Ok(if indirection { "PVOID" } else { "VOID" }),
        pdb::PrimitiveKind::Char | pdb::PrimitiveKind::RChar | pdb::PrimitiveKind::I8 => {
            Ok(if indirection { "PCHAR" } else { "CHAR" })
        }
        pdb::PrimitiveKind::UChar | pdb::PrimitiveKind::U8 => {
            Ok(if indirection { "PUCHAR" } else { "UCHAR" })
        }
        pdb::PrimitiveKind::WChar => Ok(if indirection { "PWCHAR" } else { "WCHAR" }),
        pdb::PrimitiveKind::RChar16 => Ok(if indirection { "char16_t*" } else { "char16_t" }),
        pdb::PrimitiveKind::RChar32 => Ok(if indirection { "char32_t*" } else { "char32_t" }),
        pdb::PrimitiveKind::Char8 => Ok(if indirection { "char8_t*" } else { "char8_t" }),

        pdb::PrimitiveKind::I16 | pdb::PrimitiveKind::Short => {
            Ok(if indirection { "PSHORT" } else { "SHORT" })
        }
        pdb::PrimitiveKind::U16 | pdb::PrimitiveKind::UShort => {
            Ok(if indirection { "PUSHORT" } else { "USHORT" })
        }
        pdb::PrimitiveKind::I32 | pdb::PrimitiveKind::Long => {
            Ok(if indirection { "PLONG" } else { "LONG" })
        }
        pdb::PrimitiveKind::U32 | pdb::PrimitiveKind::ULong => {
            Ok(if indirection { "PULONG" } else { "ULONG" })
        }
        pdb::PrimitiveKind::I64 | pdb::PrimitiveKind::Quad => {
            Ok(if indirection { "PLONGLONG" } else { "LONGLONG" })
        }
        pdb::PrimitiveKind::U64 | pdb::PrimitiveKind::UQuad => Ok(if indirection {
            "PULONGLONG"
        } else {
            "ULONGLONG"
        }),

        pdb::PrimitiveKind::F32 => Ok(if indirection { "PFLOAT" } else { "FLOAT" }),
        pdb::PrimitiveKind::F64 => Ok(if indirection { "DOUBLE*" } else { "DOUBLE" }),

        pdb::PrimitiveKind::Bool8 => Ok(if indirection { "PBOOLEAN" } else { "BOOLEAN" }),
        pdb::PrimitiveKind::Bool32 => Ok(if indirection { "PBOOL" } else { "BOOL" }),

        // Microsoft-specific
        pdb::PrimitiveKind::HRESULT => Ok(if indirection { "HRESULT*" } else { "HRESULT" }),

        // TODO: Seems valid for C++ method parameters. Are there other
        // cases of legitimate "NoType" occurences?
        pdb::PrimitiveKind::NoType => Ok("..."),

        _ => Err(ResymCoreError::NotImplementedError(format!(
            "/* FIXME: Unhandled primitive kind: '{primitive_kind:?}' */ void"
        ))),
    };

    Ok(str_representation?.to_string())
}

fn primitive_kind_as_str_raw(
    primitive_kind: pdb::PrimitiveKind,
    indirection: bool,
) -> Result<String> {
    let str_representation = match primitive_kind {
        pdb::PrimitiveKind::Void => Ok("void"),
        pdb::PrimitiveKind::I8 | pdb::PrimitiveKind::Char | pdb::PrimitiveKind::RChar => Ok("char"),
        pdb::PrimitiveKind::U8 | pdb::PrimitiveKind::UChar => Ok("unsigned char"),
        pdb::PrimitiveKind::WChar => Ok("wchar_t"),
        pdb::PrimitiveKind::RChar16 => Ok("char16_t"),
        pdb::PrimitiveKind::RChar32 => Ok("char32_t"),
        pdb::PrimitiveKind::Char8 => Ok("char8_t"),

        pdb::PrimitiveKind::I16 | pdb::PrimitiveKind::Short => Ok("short"),
        pdb::PrimitiveKind::U16 | pdb::PrimitiveKind::UShort => Ok("unsigned short"),
        pdb::PrimitiveKind::I32 | pdb::PrimitiveKind::Long => Ok("int"),
        pdb::PrimitiveKind::U32 | pdb::PrimitiveKind::ULong => Ok("unsigned int"),
        pdb::PrimitiveKind::I64 | pdb::PrimitiveKind::Quad => Ok("long long int"),
        pdb::PrimitiveKind::U64 | pdb::PrimitiveKind::UQuad => Ok("unsigned long long int"),

        pdb::PrimitiveKind::F32 => Ok("float"),
        pdb::PrimitiveKind::F64 => Ok("double"),

        pdb::PrimitiveKind::Bool8 => Ok("bool"),
        pdb::PrimitiveKind::Bool32 => Ok("long"),

        // Microsoft-specific, usually implemented as "long"
        pdb::PrimitiveKind::HRESULT => Ok("long"),

        // TODO: Seems valid for C++ method parameters. Are there other
        // cases of legitimate "NoType" occurences?
        pdb::PrimitiveKind::NoType => Ok("..."),

        _ => Err(ResymCoreError::NotImplementedError(format!(
            "/* FIXME: Unhandled primitive kind: '{primitive_kind:?}' */ void"
        ))),
    };

    let mut string_representation = str_representation?.to_string();
    if indirection {
        string_representation.push('*');
    }

    Ok(string_representation)
}

fn primitive_kind_as_str_msvc(
    primitive_kind: pdb::PrimitiveKind,
    indirection: bool,
) -> Result<String> {
    let str_representation = match primitive_kind {
        pdb::PrimitiveKind::Void => Ok("void"),
        pdb::PrimitiveKind::Char | pdb::PrimitiveKind::RChar => Ok("char"),
        pdb::PrimitiveKind::UChar => Ok("unsigned char"),
        pdb::PrimitiveKind::WChar => Ok("wchar_t"),
        pdb::PrimitiveKind::RChar16 => Ok("char16_t"),
        pdb::PrimitiveKind::RChar32 => Ok("char32_t"),
        pdb::PrimitiveKind::Char8 => Ok("char8_t"),

        pdb::PrimitiveKind::I8 => Ok("__int8"),
        pdb::PrimitiveKind::U8 => Ok("unsigned __int8"),
        pdb::PrimitiveKind::I16 => Ok("__int16"),
        pdb::PrimitiveKind::U16 => Ok("unsigned __int16"),
        pdb::PrimitiveKind::I32 => Ok("int"),
        pdb::PrimitiveKind::U32 => Ok("unsigned int"),
        pdb::PrimitiveKind::I64 => Ok("__int64"),
        pdb::PrimitiveKind::U64 => Ok("unsigned __int64"),

        pdb::PrimitiveKind::Short => Ok("short"),
        pdb::PrimitiveKind::UShort => Ok("unsigned short"),
        pdb::PrimitiveKind::Long => Ok("long"),
        pdb::PrimitiveKind::ULong => Ok("unsigned long"),
        pdb::PrimitiveKind::Quad => Ok("long long"),
        pdb::PrimitiveKind::UQuad => Ok("unsigned long long"),

        pdb::PrimitiveKind::F32 => Ok("float"),
        pdb::PrimitiveKind::F64 => Ok("double"),

        pdb::PrimitiveKind::Bool8 => Ok("bool"),
        pdb::PrimitiveKind::Bool32 => Ok("long"),

        // Microsoft-specific, usually implemented as "long"
        pdb::PrimitiveKind::HRESULT => Ok("long"),

        // TODO: Seems valid for C++ method parameters. Are there other
        // cases of legitimate "NoType" occurences?
        pdb::PrimitiveKind::NoType => Ok("..."),

        _ => Err(ResymCoreError::NotImplementedError(format!(
            "/* FIXME: Unhandled primitive kind: '{primitive_kind:?}' */ void"
        ))),
    };

    let mut string_representation = str_representation?.to_string();
    if indirection {
        string_representation.push('*');
    }

    Ok(string_representation)
}
