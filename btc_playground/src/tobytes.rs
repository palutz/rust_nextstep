use core::mem::size_of;

pub trait ToBeBytes {
    type ReturnType;
    fn to_be_bytes(&self) -> Self::ReturnType;
}

impl ToBeBytes for i32 {
    type ReturnType = [u8; size_of::<Self>()];
    fn to_be_bytes(&self) -> Self::ReturnType {
        return i32::to_be_bytes(*self);
    }
} 

impl ToBeBytes for i64 {
    type ReturnType = [u8; size_of::<Self>()];
    fn to_be_bytes(&self) -> Self::ReturnType {
        return i64::to_be_bytes(*self);
    }
} 


impl ToBeBytes for u16 {
    type ReturnType = [u8; size_of::<Self>()];
    fn to_be_bytes(&self) -> Self::ReturnType {
        return u16::to_be_bytes(*self);
    }
} 

impl ToBeBytes for u64 {
    type ReturnType = [u8; size_of::<Self>()];
    fn to_be_bytes(&self) -> Self::ReturnType {
        return u64::to_be_bytes(*self);
    }
} 
pub trait ToLeBytes {
    type ReturnType;
    fn to_be_bytes(&self) -> Self::ReturnType;
}
