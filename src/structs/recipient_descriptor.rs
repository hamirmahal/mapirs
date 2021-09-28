use std::convert::TryFrom;

use crate::ffi::conversion;
use crate::types::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawMapiRecipDesc {
    reserved: ULong,
    // ULONG ulReserved - reserved for future use
    recip_class: ULong,
    // ULONG ulRecipClass - recipient class
    name: LpStr,
    // LPSTR lpszName - recipient name
    address: LpStr,
    // LPSTR lpszAddress - recitpient address (optional)
    eid_size: ULong,
    // ULONG ulEIDSize count in bytes of size of pEntryID
    entry_id: *const libc::c_uchar, // LPVOID lpEntryID system-specific recipient reference
}

#[derive(Debug)]
pub struct RecipientDescriptor {
    recip_class: ULong,
    name: String,
    pub address: Option<String>,
    entry_id: Vec<u8>,
}

impl TryFrom<*const RawMapiRecipDesc> for RecipientDescriptor {
    type Error = ();
    fn try_from(raw_ptr: *const RawMapiRecipDesc) -> Result<Self, Self::Error> {
        if raw_ptr.is_null() {
            Err(())
        } else {
            /*
            SAFETY: https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer
            Raw Pointers:
            * Are allowed to ignore the borrowing rules by having both immutable and mutable
              pointers or multiple mutable pointers to the same location:
                -> we don't copy these pointers or mutate the pointees, so the only way this can
                   cause problems would be a bug in the calling app
            * Aren’t guaranteed to point to valid memory:
                -> this would be a bug in the calling app, we're using repr(C) to make
                   RawMapiRecipDesc as defined in mapi.h
            * Are allowed to be null:
                -> we checked that
            * Don’t implement any automatic cleanup:
                -> we got the ptr over ffi, so the calling app needs to clean this up
            */
            let raw: &RawMapiRecipDesc = unsafe { &*raw_ptr };
            Ok(Self::from(raw))
        }
    }
}

impl From<&RawMapiRecipDesc> for RecipientDescriptor {
    fn from(raw: &RawMapiRecipDesc) -> Self {
        RecipientDescriptor {
            recip_class: raw.recip_class,
            name: conversion::maybe_string_from_raw_ptr(raw.name)
                .unwrap_or_else(|| "MISSING_RECIP_NAME".to_owned()),
            address: conversion::maybe_string_from_raw_ptr(raw.address),
            entry_id: conversion::copy_c_array_to_vec(raw.entry_id, raw.eid_size as usize),
        }
    }
}

impl RecipientDescriptor {
    #[cfg(test)]
    pub fn new(address: &str) -> Self {
        Self {
            recip_class: 0,
            name: "".to_owned(),
            address: Some(address.to_owned()),
            entry_id: vec![0, 0, 0, 0],
        }
    }
}