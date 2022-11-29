use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Visibility : u8 {
        const Public =    0b0000_0001;
        const Protected = 0b0000_0010;
        const Private =   0b0000_0100;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MethodModifiers : u16 {
        const Static =       0b0000_0000_0000_1000;
        const Final =        0b0000_0000_0001_0000;
        const Transient =    0b0000_0000_0010_0000;
        const Volatile =     0b0000_0000_0100_0000;
        const Strictfp =     0b0000_0000_1000_0000;
        const Abstract =     0b0000_0001_0000_0000;
        const Native =       0b0000_0010_0000_0000;
        const Synchronized = 0b0000_0100_0000_0000;
        const Default =      0b0000_1000_0000_0000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct FieldModifiers : u8 {
        const Static =    0b00001000;
        const Final =     0b00010000;
        const Transient = 0b00100000;
        const Volatile =  0b01000000;
        const Strictfp =  0b10000000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ClassModifiers : u8 {
        const Static =    0b00001000;
        const Final =     0b00010000;
        const Abstract =  0b00100000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct AnnotationModifiers : u8 {
        const Static =    0b00001000;
        const Final =     0b00010000;
        const Abstract =  0b00100000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct InterfaceModifiers : u8 {
        const Static =    0b00001000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct EnumModifiers : u8 {
        const Static =    0b00001000;
    }
}

bitflags! {
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ParameterModifiers : u8 {
        const Final =     0b00000001;
    }
}
