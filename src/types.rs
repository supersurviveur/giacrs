//! Internal giac types and enums

/// Represents a giac expression type
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub enum GenType {
    /// A C integer
    Int = 0, // int val
    /// A C double
    Double = 1, // double _DOUBLE_val
    /// A C float
    Float = 21, // immediate, _FLOAT_val
    /// An arbitrary precision GMP integer
    MPZInt = 2, // mpz_t * _ZINTptr
    /// An arbitrary precision GMP float
    MPZFloat = 3, // mpf_t * _REALptr
    /// A complex number
    Complex = 4, // gen * _CPLXptr
    /// A polynom
    Polynom = 5, // polynome * _POLYptr
    /// An identifier, like `pi` or `x`
    Ident = 6, // identificateur * _IDNTptr
    /// A vector object, matrix also share this type (2D vector)
    Vector = 7, // vecteur * _VECTptr
    /// A symbolic expression, `̂x^2 + a`
    Symbolic = 8, // symbolic * _SYMBptr
    /// TODO what's this?
    Spol1 = 9, // sparse_poly1 * _SPOL1ptr
    /// A rational expression
    Fraction = 10, // fraction * _FRACptr
    /// TODO what's this?
    Ext = 11, // gen * _EXTptr
    /// A string
    String = 12, // string * _STRNGptr
    /// A giac function
    Function = 13, // unary_fonction_ptr * _FUNCptr
    /// TODO what's this?
    Root = 14, // real_complex_rootof *_ROOTptr
    /// A modulo expression, in `ℤ/pℤ`
    Modulo = 15, // gen * _MODptr
    /// TODO what's this?
    User = 16, // gen_user * _USERptr
    /// TODO what's this?
    Map = 17, // map<gen.gen> * _MAPptr
    /// TODO what's this?
    Eqw = 18, // eqwdata * _EQWptr
    /// TODO what's this?
    Grob = 19, // grob * _GROBptr
    /// TODO what's this?
    Pointer = 20, // void * _POINTER_val
}
