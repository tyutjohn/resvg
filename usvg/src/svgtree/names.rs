// This file is autogenerated. Do not edit it!
// See ./codegen for details.

use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum EId {
    A,
    Circle,
    ClipPath,
    Defs,
    Ellipse,
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLighting,
    FeDisplacementMap,
    FeDistantLight,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FePointLight,
    FeSpecularLighting,
    FeSpotLight,
    FeTile,
    FeTurbulence,
    Filter,
    G,
    Image,
    Line,
    LinearGradient,
    Marker,
    Mask,
    Path,
    Pattern,
    Polygon,
    Polyline,
    RadialGradient,
    Rect,
    Stop,
    Style,
    Svg,
    Switch,
    Symbol,
    Text,
    TextPath,
    Tref,
    Tspan,
    Use
}

static ELEMENTS: Map<EId> = Map {
    key: 732231254413039614,
    disps: &[
        (0, 12),
        (1, 11),
        (10, 26),
        (2, 42),
        (1, 19),
        (0, 5),
        (1, 13),
        (8, 50),
        (0, 0),
        (1, 0),
        (7, 45),
    ],
    entries: &[
        ("feFlood", EId::FeFlood),
        ("radialGradient", EId::RadialGradient),
        ("feImage", EId::FeImage),
        ("stop", EId::Stop),
        ("fePointLight", EId::FePointLight),
        ("feConvolveMatrix", EId::FeConvolveMatrix),
        ("feComposite", EId::FeComposite),
        ("clipPath", EId::ClipPath),
        ("feMerge", EId::FeMerge),
        ("defs", EId::Defs),
        ("mask", EId::Mask),
        ("svg", EId::Svg),
        ("symbol", EId::Symbol),
        ("linearGradient", EId::LinearGradient),
        ("feSpecularLighting", EId::FeSpecularLighting),
        ("feFuncB", EId::FeFuncB),
        ("filter", EId::Filter),
        ("feFuncG", EId::FeFuncG),
        ("circle", EId::Circle),
        ("g", EId::G),
        ("tref", EId::Tref),
        ("feFuncA", EId::FeFuncA),
        ("image", EId::Image),
        ("text", EId::Text),
        ("line", EId::Line),
        ("pattern", EId::Pattern),
        ("use", EId::Use),
        ("feDropShadow", EId::FeDropShadow),
        ("feSpotLight", EId::FeSpotLight),
        ("marker", EId::Marker),
        ("style", EId::Style),
        ("switch", EId::Switch),
        ("tspan", EId::Tspan),
        ("feColorMatrix", EId::FeColorMatrix),
        ("feOffset", EId::FeOffset),
        ("path", EId::Path),
        ("feGaussianBlur", EId::FeGaussianBlur),
        ("feTile", EId::FeTile),
        ("feTurbulence", EId::FeTurbulence),
        ("feMergeNode", EId::FeMergeNode),
        ("feMorphology", EId::FeMorphology),
        ("a", EId::A),
        ("textPath", EId::TextPath),
        ("ellipse", EId::Ellipse),
        ("feComponentTransfer", EId::FeComponentTransfer),
        ("feDistantLight", EId::FeDistantLight),
        ("polyline", EId::Polyline),
        ("polygon", EId::Polygon),
        ("feBlend", EId::FeBlend),
        ("feDisplacementMap", EId::FeDisplacementMap),
        ("feDiffuseLighting", EId::FeDiffuseLighting),
        ("rect", EId::Rect),
        ("feFuncR", EId::FeFuncR),
    ],
};

impl EId {
    pub fn from_str(text: &str) -> Option<EId> {
        ELEMENTS.get(text).cloned()
    }

    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ELEMENTS.key(&self)
    }
}

impl fmt::Debug for EId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for EId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum AId {
    Amplitude,
    Azimuth,
    BaseFrequency,
    BaselineShift,
    Bias,
    Class,
    ClipPath,
    ClipRule,
    ClipPathUnits,
    Color,
    ColorInterpolationFilters,
    Cx,
    Cy,
    D,
    DiffuseConstant,
    Direction,
    Display,
    Divisor,
    Dx,
    Dy,
    EdgeMode,
    Elevation,
    EnableBackground,
    Exponent,
    Fill,
    FillOpacity,
    FillRule,
    Filter,
    FilterUnits,
    FloodColor,
    FloodOpacity,
    FontFamily,
    FontSize,
    FontStretch,
    FontStyle,
    FontVariant,
    FontWeight,
    Fx,
    Fy,
    GradientTransform,
    GradientUnits,
    Height,
    Href,
    Id,
    ImageRendering,
    In,
    In2,
    Intercept,
    K1,
    K2,
    K3,
    K4,
    KernelMatrix,
    KernelUnitLength,
    LetterSpacing,
    LightingColor,
    LimitingConeAngle,
    MarkerEnd,
    MarkerMid,
    MarkerStart,
    MarkerHeight,
    MarkerUnits,
    MarkerWidth,
    Mask,
    MaskContentUnits,
    MaskUnits,
    Mode,
    NumOctaves,
    Offset,
    Opacity,
    Operator,
    Order,
    Orient,
    Overflow,
    PaintOrder,
    PatternContentUnits,
    PatternTransform,
    PatternUnits,
    Points,
    PointsAtX,
    PointsAtY,
    PointsAtZ,
    PreserveAlpha,
    PreserveAspectRatio,
    PrimitiveUnits,
    R,
    Radius,
    RefX,
    RefY,
    RequiredExtensions,
    RequiredFeatures,
    Result,
    Rotate,
    Rx,
    Ry,
    Scale,
    Seed,
    ShapeRendering,
    Slope,
    Space,
    SpecularConstant,
    SpecularExponent,
    SpreadMethod,
    StartOffset,
    StdDeviation,
    StitchTiles,
    StopColor,
    StopOpacity,
    Stroke,
    StrokeDasharray,
    StrokeDashoffset,
    StrokeLinecap,
    StrokeLinejoin,
    StrokeMiterlimit,
    StrokeOpacity,
    StrokeWidth,
    Style,
    SurfaceScale,
    SystemLanguage,
    TableValues,
    TargetX,
    TargetY,
    TextAnchor,
    TextDecoration,
    TextRendering,
    Transform,
    Type,
    Values,
    ViewBox,
    Visibility,
    Width,
    WordSpacing,
    WritingMode,
    X,
    X1,
    X2,
    XChannelSelector,
    Y,
    Y1,
    Y2,
    YChannelSelector,
    Z
}

static ATTRIBUTES: Map<AId> = Map {
    key: 3213172566270843353,
    disps: &[
        (0, 13),
        (1, 7),
        (4, 131),
        (0, 7),
        (16, 105),
        (0, 20),
        (3, 17),
        (0, 5),
        (95, 79),
        (5, 45),
        (0, 14),
        (0, 24),
        (0, 0),
        (0, 6),
        (0, 77),
        (1, 3),
        (1, 8),
        (23, 127),
        (129, 108),
        (3, 18),
        (0, 56),
        (0, 4),
        (1, 3),
        (2, 95),
        (6, 102),
        (2, 72),
        (0, 68),
        (0, 4),
        (7, 45),
    ],
    entries: &[
        ("diffuseConstant", AId::DiffuseConstant),
        ("color", AId::Color),
        ("text-decoration", AId::TextDecoration),
        ("kernelUnitLength", AId::KernelUnitLength),
        ("baseFrequency", AId::BaseFrequency),
        ("paint-order", AId::PaintOrder),
        ("fx", AId::Fx),
        ("targetY", AId::TargetY),
        ("elevation", AId::Elevation),
        ("in2", AId::In2),
        ("filter", AId::Filter),
        ("y", AId::Y),
        ("kernelMatrix", AId::KernelMatrix),
        ("stroke-miterlimit", AId::StrokeMiterlimit),
        ("color-interpolation-filters", AId::ColorInterpolationFilters),
        ("font-size", AId::FontSize),
        ("r", AId::R),
        ("cx", AId::Cx),
        ("writing-mode", AId::WritingMode),
        ("maskUnits", AId::MaskUnits),
        ("primitiveUnits", AId::PrimitiveUnits),
        ("surfaceScale", AId::SurfaceScale),
        ("stitchTiles", AId::StitchTiles),
        ("refY", AId::RefY),
        ("orient", AId::Orient),
        ("clip-rule", AId::ClipRule),
        ("marker-mid", AId::MarkerMid),
        ("x1", AId::X1),
        ("ry", AId::Ry),
        ("in", AId::In),
        ("x2", AId::X2),
        ("direction", AId::Direction),
        ("marker-end", AId::MarkerEnd),
        ("rotate", AId::Rotate),
        ("divisor", AId::Divisor),
        ("viewBox", AId::ViewBox),
        ("xChannelSelector", AId::XChannelSelector),
        ("style", AId::Style),
        ("pointsAtZ", AId::PointsAtZ),
        ("values", AId::Values),
        ("stroke-linejoin", AId::StrokeLinejoin),
        ("stroke-dasharray", AId::StrokeDasharray),
        ("k1", AId::K1),
        ("maskContentUnits", AId::MaskContentUnits),
        ("id", AId::Id),
        ("startOffset", AId::StartOffset),
        ("radius", AId::Radius),
        ("targetX", AId::TargetX),
        ("patternTransform", AId::PatternTransform),
        ("result", AId::Result),
        ("display", AId::Display),
        ("dy", AId::Dy),
        ("slope", AId::Slope),
        ("systemLanguage", AId::SystemLanguage),
        ("dx", AId::Dx),
        ("fill", AId::Fill),
        ("font-weight", AId::FontWeight),
        ("cy", AId::Cy),
        ("offset", AId::Offset),
        ("bias", AId::Bias),
        ("letter-spacing", AId::LetterSpacing),
        ("numOctaves", AId::NumOctaves),
        ("space", AId::Space),
        ("patternContentUnits", AId::PatternContentUnits),
        ("intercept", AId::Intercept),
        ("text-rendering", AId::TextRendering),
        ("k2", AId::K2),
        ("yChannelSelector", AId::YChannelSelector),
        ("k4", AId::K4),
        ("tableValues", AId::TableValues),
        ("z", AId::Z),
        ("refX", AId::RefX),
        ("mask", AId::Mask),
        ("order", AId::Order),
        ("overflow", AId::Overflow),
        ("clipPathUnits", AId::ClipPathUnits),
        ("text-anchor", AId::TextAnchor),
        ("stop-opacity", AId::StopOpacity),
        ("spreadMethod", AId::SpreadMethod),
        ("font-style", AId::FontStyle),
        ("stroke", AId::Stroke),
        ("class", AId::Class),
        ("image-rendering", AId::ImageRendering),
        ("type", AId::Type),
        ("shape-rendering", AId::ShapeRendering),
        ("exponent", AId::Exponent),
        ("fill-opacity", AId::FillOpacity),
        ("x", AId::X),
        ("clip-path", AId::ClipPath),
        ("d", AId::D),
        ("stroke-linecap", AId::StrokeLinecap),
        ("operator", AId::Operator),
        ("opacity", AId::Opacity),
        ("fy", AId::Fy),
        ("font-variant", AId::FontVariant),
        ("edgeMode", AId::EdgeMode),
        ("font-stretch", AId::FontStretch),
        ("y2", AId::Y2),
        ("mode", AId::Mode),
        ("specularExponent", AId::SpecularExponent),
        ("baseline-shift", AId::BaselineShift),
        ("k3", AId::K3),
        ("patternUnits", AId::PatternUnits),
        ("azimuth", AId::Azimuth),
        ("font-family", AId::FontFamily),
        ("limitingConeAngle", AId::LimitingConeAngle),
        ("points", AId::Points),
        ("fill-rule", AId::FillRule),
        ("requiredExtensions", AId::RequiredExtensions),
        ("markerWidth", AId::MarkerWidth),
        ("amplitude", AId::Amplitude),
        ("seed", AId::Seed),
        ("href", AId::Href),
        ("flood-opacity", AId::FloodOpacity),
        ("rx", AId::Rx),
        ("transform", AId::Transform),
        ("pointsAtX", AId::PointsAtX),
        ("stop-color", AId::StopColor),
        ("marker-start", AId::MarkerStart),
        ("preserveAspectRatio", AId::PreserveAspectRatio),
        ("pointsAtY", AId::PointsAtY),
        ("height", AId::Height),
        ("filterUnits", AId::FilterUnits),
        ("y1", AId::Y1),
        ("lighting-color", AId::LightingColor),
        ("width", AId::Width),
        ("gradientUnits", AId::GradientUnits),
        ("stroke-width", AId::StrokeWidth),
        ("gradientTransform", AId::GradientTransform),
        ("stdDeviation", AId::StdDeviation),
        ("preserveAlpha", AId::PreserveAlpha),
        ("stroke-opacity", AId::StrokeOpacity),
        ("flood-color", AId::FloodColor),
        ("requiredFeatures", AId::RequiredFeatures),
        ("scale", AId::Scale),
        ("enable-background", AId::EnableBackground),
        ("specularConstant", AId::SpecularConstant),
        ("word-spacing", AId::WordSpacing),
        ("markerUnits", AId::MarkerUnits),
        ("stroke-dashoffset", AId::StrokeDashoffset),
        ("markerHeight", AId::MarkerHeight),
        ("visibility", AId::Visibility),
    ],
};

impl AId {
    pub fn from_str(text: &str) -> Option<AId> {
        ATTRIBUTES.get(text).cloned()
    }

    #[inline(never)]
    pub fn to_str(self) -> &'static str {
        ATTRIBUTES.key(&self)
    }
}

impl fmt::Debug for AId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl fmt::Display for AId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// A stripped down `phf` crate fork.
//
// https://github.com/sfackler/rust-phf

struct Map<V: 'static> {
    pub key: u64,
    pub disps: &'static [(u32, u32)],
    pub entries: &'static[(&'static str, V)],
}

impl<V: PartialEq> Map<V> {
    fn get(&self, key: &str) -> Option<&V> {
        use std::borrow::Borrow;

        let hash = hash(key, self.key);
        let index = get_index(hash, &*self.disps, self.entries.len());
        let entry = &self.entries[index as usize];
        let b = entry.0.borrow();
        if b == key {
            Some(&entry.1)
        } else {
            None
        }
    }

    fn key(&self, value: &V) -> &'static str {
        self.entries.iter().find(|kv| kv.1 == *value).unwrap().0
    }
}

#[inline]
fn hash(x: &str, key: u64) -> u64 {
    use std::hash::Hasher;

    let mut hasher = siphasher::sip::SipHasher13::new_with_keys(0, key);
    hasher.write(x.as_bytes());
    hasher.finish()
}

#[inline]
fn get_index(hash: u64, disps: &[(u32, u32)], len: usize) -> u32 {
    let (g, f1, f2) = split(hash);
    let (d1, d2) = disps[(g % (disps.len() as u32)) as usize];
    displace(f1, f2, d1, d2) % (len as u32)
}

#[inline]
fn split(hash: u64) -> (u32, u32, u32) {
    const BITS: u32 = 21;
    const MASK: u64 = (1 << BITS) - 1;

    ((hash & MASK) as u32,
     ((hash >> BITS) & MASK) as u32,
     ((hash >> (2 * BITS)) & MASK) as u32)
}

#[inline]
fn displace(f1: u32, f2: u32, d1: u32, d2: u32) -> u32 {
    d2 + f1 * d1 + f2
}
