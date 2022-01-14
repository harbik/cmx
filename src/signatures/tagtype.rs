use num_derive::FromPrimitive;
use serde::Serialize;

#[derive(FromPrimitive, PartialEq, Clone, Copy, Debug, Serialize)]
pub enum TagTypeSignature {
    UndefinedType                  = 0x00000000,
    ChromaticityType               = 0x6368726D,  /* 'chrm' */
    ColorantOrderType              = 0x636C726F,  /* 'clro' */
    ColorantTableType              = 0x636C7274,  /* 'clrt' */
    CrdInfoType                    = 0x63726469,  /* 'crdi' Removed in V4 */
    CurveType                      = 0x63757276,  /* 'curv' */
    DataType                       = 0x64617461,  /* 'data' */
    DictType                       = 0x64696374,  /* 'dict' */
    DateTimeType                   = 0x6474696D,  /* 'dtim' */
    DeviceSettingsType             = 0x64657673,  /* 'devs' Removed in V4 */
    EmbeddedHeightImageType        = 0x6568696D,  /* 'ehim' */
    EmbeddedNormalImageType        = 0x656e696d,  /* 'enim' */
    Float16ArrayType               = 0x666c3136,  /* 'fl16' */
    Float32ArrayType               = 0x666c3332,  /* 'fl32' */
    Float64ArrayType               = 0x666c3634,  /* 'fl64' */
    GamutBoundaryDescType	       = 0x67626420,  /* 'gbd ' */
    Lut16Type                      = 0x6d667432,  /* 'mft2' */
    Lut8Type                       = 0x6d667431,  /* 'mft1' */
    LutAtoBType                    = 0x6d414220,  /* 'mAB ' */
    LutBtoAType                    = 0x6d424120,  /* 'mBA ' */
    MeasurementType                = 0x6D656173,  /* 'meas' */
    MakeAndModelType               = 0x6d6d6f64,  /* 'mmod' Apple Make and Model */
    MultiLocalizedUnicodeType      = 0x6D6C7563,  /* 'mluc' */
    MultiProcessElementType        = 0x6D706574,  /* 'mpet' */
    NamedColor2Type                = 0x6E636C32,  /* 'ncl2' use v2-v4*/
    ParametricCurveType            = 0x70617261,  /* 'para' */
    ProfileSequenceDescType        = 0x70736571,  /* 'pseq' */
    ProfileSequceIdType            = 0x70736964,  /* 'psid' */
    ResponseCurveSet16Type         = 0x72637332,  /* 'rcs2' */
    S15Fixed16ArrayType            = 0x73663332,  /* 'sf32' */
    ScreeningType                  = 0x7363726E,  /* 'scrn' Removed in V4 */
    SegmentedCurveType             = 0x63757266,  /* 'curf' */
    SignatureType                  = 0x73696720,  /* 'sig ' */
    SparseMatrixArrayType          = 0x736D6174,  /* 'smat' */
    SpectralViewingConditionsType  = 0x7376636e,  /* 'svcn' */
    SpectralDataInfoType           = 0x7364696e,  /* 'sdin' */
    TagArrayType                   = 0x74617279,  /* 'tary' */
    TagStructType                  = 0x74737472,  /* 'tstr' */
    TextType                       = 0x74657874,  /* 'text' */
    TextDescriptionType            = 0x64657363,  /* 'desc' Removed in V4 */
    U16Fixed16ArrayType            = 0x75663332,  /* 'uf32' */
    UcrBgType                      = 0x62666420,  /* 'bfd ' Removed in V4 */
    UInt16ArrayType                = 0x75693136,  /* 'ui16' */
    UInt32ArrayType                = 0x75693332,  /* 'ui32' */
    UInt64ArrayType                = 0x75693634,  /* 'ui64' */
    UInt8ArrayType                 = 0x75693038,  /* 'ui08' */
    ViewingConditionsType          = 0x76696577,  /* 'view' */
    VcgtType                       = 0x76636774,  /* 'vcgt' not icc, defacto standard, used for tag and type */
    Utf8TextType                   = 0x75746638,  /* 'utf8' */
    Utf16TextType                  = 0x75743136,  /* 'ut16' */
    /* XYZType                      = 0x58595A20, // 'XYZ ' Name changed to XYZArrayType */ 
    XYZArrayType                   = 0x58595A20,  /* 'XYZ ' */
    ZipUtf8TextType                = 0x7a757438,  /* 'zut8' */
    ZipXmlType                     = 0x5a584d4c,  /* 'ZXML' */      
    EmbeddedProfileType            = 0x49434370,  /* 'ICCp' */
}