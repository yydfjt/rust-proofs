use api::errors::SectorManagerErr;
use failure::Error;
use ffi_toolkit::c_str_to_rust_str;
use libc;
use std::ffi::CString;
use std::mem;
use std::ptr;

// TODO: libfilecoin_proofs.h and libsector_base.h will likely be consumed by
// the same program, so these names need to be unique. Alternatively, figure
// out a way to share this enum across crates in a way that won't cause
// cbindgen to fail.
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum SBResponseStatus {
    SBNoError = 0,
    SBUnclassifiedError = 1,
    SBCallerError = 2,
    SBReceiverError = 3,
}

// err_code_and_msg accepts an Error struct and produces a tuple of response
// status code and a pointer to a C string, both of which can be used to set
// fields in a response struct to be returned from an FFI call.
pub fn err_code_and_msg(err: &Error) -> (SBResponseStatus, *const libc::c_char) {
    use api::responses::SBResponseStatus::*;

    let msg = CString::new(format!("{}", err)).unwrap();
    let ptr = msg.as_ptr();
    mem::forget(msg);

    match err.downcast_ref() {
        Some(SectorManagerErr::UnclassifiedError(_)) => return (SBUnclassifiedError, ptr),
        Some(SectorManagerErr::CallerError(_)) => return (SBCallerError, ptr),
        Some(SectorManagerErr::ReceiverError(_)) => return (SBReceiverError, ptr),
        None => (),
    }

    (SBUnclassifiedError, ptr)
}

///////////////////////////////////////////////////////////////////////////////
/// NewStagingSectorAccessResponse
//////////////////////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct NewStagingSectorAccessResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_access: *const libc::c_char,
}

impl Default for NewStagingSectorAccessResponse {
    fn default() -> NewStagingSectorAccessResponse {
        NewStagingSectorAccessResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            sector_access: ptr::null(),
        }
    }
}

impl Drop for NewStagingSectorAccessResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
            drop(c_str_to_rust_str(self.sector_access));
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
/// NewSealedSectorAccessResponse
/////////////////////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct NewSealedSectorAccessResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_access: *const libc::c_char,
}

impl Default for NewSealedSectorAccessResponse {
    fn default() -> NewSealedSectorAccessResponse {
        NewSealedSectorAccessResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            sector_access: ptr::null(),
        }
    }
}

impl Drop for NewSealedSectorAccessResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
            drop(c_str_to_rust_str(self.sector_access));
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
/// WriteAndPreprocesssResponse
///////////////////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct WriteAndPreprocessResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes_written: u64,
}

impl Default for WriteAndPreprocessResponse {
    fn default() -> WriteAndPreprocessResponse {
        WriteAndPreprocessResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes_written: 0,
        }
    }
}

impl Drop for WriteAndPreprocessResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
/// ReadRawResponse
///////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct ReadRawResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub data_len: libc::size_t,
    pub data_ptr: *const u8,
}

impl Default for ReadRawResponse {
    fn default() -> ReadRawResponse {
        ReadRawResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            data_len: 0,
            data_ptr: ptr::null(),
        }
    }
}

impl Drop for ReadRawResponse {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(
                self.data_ptr as *mut u8,
                self.data_len,
                self.data_len,
            ));

            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
/// TruncateUnsealedResponse
////////////////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct TruncateUnsealedResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
}

impl Default for TruncateUnsealedResponse {
    fn default() -> TruncateUnsealedResponse {
        TruncateUnsealedResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
        }
    }
}

impl Drop for TruncateUnsealedResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
/// NumUnsealedBytesResponse
////////////////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct NumUnsealedBytesResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes: u64,
}

impl Default for NumUnsealedBytesResponse {
    fn default() -> NumUnsealedBytesResponse {
        NumUnsealedBytesResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes: 0,
        }
    }
}

impl Drop for NumUnsealedBytesResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
/// GetMaxUserBytesPerStagedSectorResponse
//////////////////////////////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct GetMaxUserBytesPerStagedSectorResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes: u64,
}

impl Default for GetMaxUserBytesPerStagedSectorResponse {
    fn default() -> GetMaxUserBytesPerStagedSectorResponse {
        GetMaxUserBytesPerStagedSectorResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes: 0,
        }
    }
}

impl Drop for GetMaxUserBytesPerStagedSectorResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

///////////////////////////////////////////////////////////////////////////////
/// MaxUnsealedBytesPerSectorResponse
/////////////////////////////////////

#[repr(C)]
#[derive(Destroy)]
pub struct MaxUnsealedBytesPerSectorResponse {
    pub status_code: SBResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes: u64,
}

impl Default for MaxUnsealedBytesPerSectorResponse {
    fn default() -> MaxUnsealedBytesPerSectorResponse {
        MaxUnsealedBytesPerSectorResponse {
            status_code: SBResponseStatus::SBNoError,
            error_msg: ptr::null(),
            num_bytes: 0,
        }
    }
}

impl Drop for MaxUnsealedBytesPerSectorResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}
