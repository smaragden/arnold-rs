//! AtArray API
//!
//! The AtArray object encapsulates an array of any of the Arnold built-in data types. like AI_TYPE_BYTE, AI_TYPE_FLOAT, AI_TYPE_STRING, etc. Its API has easy-to-use accessor functions for reading and writing elements, and there are a number of functions for manipulating arrays (such as copying them). An AtArray is specified by the element data type, the number of motion keys in the array, and the number of elements per motion key. The data is grouped together by motion keys.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use ai_bindings;
use ai_bindings::{AtArray, AtMatrix, AtRGB, AtRGBA, AtVector, AtVector2};
use ai_string::AtString;

use std::ffi::CString;

pub use ai_bindings::AI_TYPE_VECTOR2;
pub use ai_bindings::AI_TYPE_ARRAY;
pub use ai_bindings::AI_TYPE_BOOLEAN;
pub use ai_bindings::AI_TYPE_BYTE;
pub use ai_bindings::AI_TYPE_CLOSURE;
pub use ai_bindings::AI_TYPE_ENUM;
pub use ai_bindings::AI_TYPE_FLOAT;
pub use ai_bindings::AI_TYPE_HALF;
pub use ai_bindings::AI_TYPE_INT;
pub use ai_bindings::AI_TYPE_MATRIX;
pub use ai_bindings::AI_TYPE_NODE;
pub use ai_bindings::AI_TYPE_NONE;
pub use ai_bindings::AI_TYPE_POINTER;
pub use ai_bindings::AI_TYPE_RGB;
pub use ai_bindings::AI_TYPE_RGBA;
pub use ai_bindings::AI_TYPE_STRING;
pub use ai_bindings::AI_TYPE_UINT;
pub use ai_bindings::AI_TYPE_UNDEFINED;
pub use ai_bindings::AI_TYPE_USHORT;
pub use ai_bindings::AI_TYPE_VECTOR;

extern "C" {
    /// Create an array and initialize it with supplied data.
    ///
    /// This function should always be given nelements * nkeys number of data elements. To create an array without supplying any initial data see [AiArrayAllocate()] instead. To create an array out of an existing memory buffer, use [AiArrayConvert()].
    ///
    /// # Safety
    /// As this function use `varargs` we expose the unsafe binding.
    /// # Example:
    /// ```rust
    /// let AtArray a = unsafe{ AiArray(4, 1, AI_TYPE_FLOAT, 10.f, 11.f, 12.f, 13.f) };
    /// assert_eq!(AiArrayGetFlt(a,3), 13.0);
    /// ```
    /// Parameters
    /// nelements	number of elements per motion key in the new array
    /// nkeys	number of motion keys in the new array
    /// type	array type ([AI_TYPE_BYTE], etc.)
    /// ...	array data
    /// # Returns
    /// pointer to the new array filled with data, or NULL if the array couldn't be allocated
    /// # See also
    /// [AiArrayAllocate]
    pub fn AiArray(nelements: u32, nkeys: u8, type_: ::std::os::raw::c_int, ...) -> *mut AtArray;
}

/// Create an empty (uninitialized) array of the specified type.
///
/// The returned array has data fully allocated (but not initialized) and its elements can be set with the ArraySet*() functions/macros.
///
/// # Parameters
/// * `nelements` - number of elements per motion key in the new array
/// * `nkeys` - number of motion keys in the new array
/// * `type` - type of the elements in the new array (AI_TYPE_BYTE, etc.)
/// # Returns
/// pointer to an allocated array of nelements * nkeys elements, or NULL if the array couldn't be allocated
/// # See also
/// [AiArray]
pub fn AiArrayAllocate(nelements: u32, nkeys: u8, type_: u8) -> *mut AtArray {
    unsafe { ai_bindings::AiArrayAllocate(nelements, nkeys, type_) }
}

/// Deallocate an array object.
///
/// # Parameters
/// * `array` - the array to destroy
pub fn AiArrayDestroy(array: *mut AtArray) {
    unsafe { ai_bindings::AiArrayDestroy(array) }
}

/// Create an array and initialize it from an existing data buffer.
///
/// This is the recommended constructor when programmatically creating arrays of arbitrary size with existing data. For on-the-fly creation of small arrays of known values, the AiArray() constructor can also be used.
///
/// Usage example:
/// ```
/// float data[4];
/// data[0] = 10.f;
/// data[1] = 11.f;
/// data[2] = 12.f;
/// data[3] = 13.f;
/// AtArray* a = AiArrayConvert(4, 1, AI_TYPE_FLOAT, data);
/// assert(AiArrayGetFlt(a,3) == 13.f);
/// ```
/// # Parameters
/// * `nelements` - number of elements per motion key
/// * `nkeys` - number of motion keys
/// * `type` - element type
/// * `data` - input data buffer
/// # Returns
/// pointer to the new array filled with data, or NULL if the array couldn't be allocated
pub fn AiArrayConvert(
    nelements: u32,
    nkeys: u8,
    type_: u8,
    data: *const ::std::os::raw::c_void,
) -> *mut AtArray {
    unsafe { ai_bindings::AiArrayConvert(nelements, nkeys, type_, data) }
}

/// Resize an existing array contents in place.
///
/// This function will resize the array keeping the current contents.
///
/// # Parameters
/// * `array` - array to resize
/// * `nelements` - number of elements per motion key
/// * `nkeys` - number of motion keys
pub fn AiArrayResize(array: *mut AtArray, nelements: u32, nkeys: u8) {
    unsafe { ai_bindings::AiArrayResize(array, nelements, nkeys) }
}

/// Create a copy of an array.
///
/// # Parameters
/// * `array` - source array to copy
/// # Returns
/// new array which is a copy of the source array, or NULL if there was an error
pub fn AiArrayCopy(array: *const AtArray) -> *mut AtArray {
    unsafe { ai_bindings::AiArrayCopy(array) }
}

/// Initializes data for all the elements in a specific key of an array.
///
/// The data buffer will be copied into the appropiate place in the array, overwriting the existing values in memory. The values passed in aren't required later on; data can be safely destroyed.
///
/// Usage example:
/// ```
/// // create an array of two motion keys, three elements per key
/// AtArray* array = AiArray(3, 2, AI_TYPE_FLOAT, 1.0f, 1.0f, 1.0f, 2.0f, 2.0f, 2.0f);
/// // at this point, the array contains:   1, 1, 1, 2, 2, 2
///
/// float newdata[3] = { 9.0f, 9.0f, 9.0f };
/// bool success = AiArraySetKey(array, 1, newdata);
/// // at this point, the array contains:   1, 1, 1, 9, 9, 9
/// ```
/// # Parameters
/// * `array` - array to write to
/// * `key` - value of the key we want to write to (must be in 0 .. array->nkeys - 1)
/// * `data` - input data buffer, with a size of exactly array->getNumElements() * sizeof(type of elements in array)
/// # Returns
/// true if the write operation was successful
pub fn AiArraySetKey(array: *mut AtArray, key: u8, data: *const ::std::os::raw::c_void) -> bool {
    unsafe { ai_bindings::AiArraySetKey(array, key, data) }
}

/// Obtains a pointer to the internal array data for construction.
///
/// Return a pointer to the internal buffer storage for this array data. This pointer can be used to read or write directly into the array buffer, avoiding an extra memory copy, and it has to be unmap'ed when writing is finished.
///
/// **NOTE:** The array will be unmap'ed automatically when set to a node parameter.
///
/// # Parameters
/// * `array` - array to write to
/// # Returns
/// pointer to the internal array data buffer
pub fn AiArrayMap(array: *mut AtArray) -> *mut ::std::os::raw::c_void {
    unsafe { ai_bindings::AiArrayMap(array) }
}

/// Obtains a pointer to a specific key in the internal array data for construction.
///
/// Return a pointer to a specific key in the internal buffer storage for this array data. This pointer can be used to read or write directly into the key data, avoiding an extra memory copy, and it has to be unmap'ed when writing is finished.
///
/// **NOTE:** The array will be unmap'ed automatically when set to a node parameter.
///
/// # Parameters
/// * `array` - array to write to
/// * `key` - index of the specific key to read from/write to
/// # Returns
/// pointer to the specific key in the internal array data buffer
pub fn AiArrayMapKey(array: *mut AtArray, key: u8) -> *mut ::std::os::raw::c_void {
    unsafe { ai_bindings::AiArrayMapKey(array, key) }
}

/// Notifies the array that construction is finished.
///
/// After calling this function, any pointers previously obtained with AiArrayMap or AiArrayMapKey will no longer be valid.
///
/// **NOTE:** The array will be unmap'ed automatically when set to a node parameter.
///
/// # Parameters
/// * `array` - array to notify about the end of construction
pub fn AiArrayUnmap(array: *mut AtArray) {
    unsafe {
        ai_bindings::AiArrayUnmap(array);
    }
}

/// Get the number of elements on each key of the array.
///
/// # Parameters
/// * `array` - source array
/// # Returns
/// number of elements per array key
pub fn AiArrayGetNumElements(array: *const AtArray) -> u32 {
    unsafe { ai_bindings::AiArrayGetNumElements(array) }
}

/// Get the number of keys.
///
/// # Parameters
/// * `array` - source array
/// # Returns
/// number of keys for this array
pub fn AiArrayGetNumKeys(array: *const AtArray) -> u8 {
    unsafe { ai_bindings::AiArrayGetNumKeys(array) }
}

/// Get the type of array elements.
///
/// # Parameters
/// * `array` - source array
/// # Returns
/// type of elements in this array
pub fn AiArrayGetType(array: *const AtArray) -> u8 {
    unsafe { ai_bindings::AiArrayGetType(array) }
}

/// Get the total size in bytes of the data buffer for this array.
///
/// # Parameters
/// * `array` - source array
/// # Returns
/// size in bytes of the whole data
pub fn AiArrayGetDataSize(array: *const AtArray) -> usize {
    unsafe { ai_bindings::AiArrayGetDataSize(array) }
}

/// Get the total size in bytes of the data for one key.
///
/// # Parameters
/// * `array` - source array
/// # Returns
/// size in bytes of one key
pub fn AiArrayGetKeySize(array: *const AtArray) -> usize {
    unsafe { ai_bindings::AiArrayGetKeySize(array) }
}

/// Interpolate a vector at a given time from an array.
///
/// # Parameters
/// * `array` - source array
/// * `time` - time to calculate the interpolation for the array item
/// * `index` - index in the array of the element to interpolate
/// # Returns
/// value of the array item, interpolated to the given time
pub fn AiArrayInterpolateVec(array: *const AtArray, time: f32, idx: u32) -> AtVector {
    unsafe { ai_bindings::AiArrayInterpolateVec(array, time, idx) }
}

/// Interpolate a color at a given time from an array.
///
/// # Parameters
/// * `array` - source array
/// * `time` - time to calculate the interpolation for the array item
/// * `index` - index in the array of the element to interpolate
/// # Returns
/// value of the array item, interpolated to the given time
pub fn AiArrayInterpolateRGB(array: *const AtArray, time: f32, idx: u32) -> AtRGB {
    unsafe { ai_bindings::AiArrayInterpolateRGB(array, time, idx) }
}

/// Interpolate an AtRGBA at a given time from an array.
///
/// # Parameters
/// * `array` - source array
/// * `time` - time to calculate the interpolation for the array item
/// * `index` - index in the array of the element to interpolate
/// # Returns
/// value of the array item, interpolated to the given time
pub fn AiArrayInterpolateRGBA(array: *const AtArray, time: f32, idx: u32) -> AtRGBA {
    unsafe { ai_bindings::AiArrayInterpolateRGBA(array, time, idx) }
}

/// Interpolate a float at a given time from an array.
///
/// # Parameters
/// * `array` - source array
/// * `time` - time to calculate the interpolation for the array item
/// * `index` - index in the array of the element to interpolate
/// # Returns
/// value of the array item, interpolated to the given time
pub fn AiArrayInterpolateFlt(array: *const AtArray, time: f32, idx: u32) -> f32 {
    unsafe { ai_bindings::AiArrayInterpolateFlt(array, time, idx) }
}

/// Interpolate a matrix at a given time from an array.
///
/// # Parameters
/// * `array` - source array
/// * `time` - time to calculate the interpolation for the array item
/// * `index` - index in the array of the element to interpolate
/// # Returns
/// value of the array item, interpolated to the given time
pub fn AiArrayInterpolateMtx(array: *const AtArray, time: f32, idx: u32) -> AtMatrix {
    unsafe { ai_bindings::AiArrayInterpolateMtx(array, time, idx) }
}

pub fn AiArrayGetBool(a: *const AtArray, i: u32) -> bool {
    unsafe {
        ai_bindings::AiArrayGetBoolFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetByte(a: *const AtArray, i: u32) -> u8 {
    unsafe {
        ai_bindings::AiArrayGetByteFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetInt(a: *const AtArray, i: u32) -> ::std::os::raw::c_int {
    unsafe {
        ai_bindings::AiArrayGetIntFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetUInt(a: *const AtArray, i: u32) -> u32 {
    unsafe {
        ai_bindings::AiArrayGetUIntFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetFlt(a: *const AtArray, i: u32) -> f32 {
    unsafe {
        ai_bindings::AiArrayGetFltFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetRGB(a: *const AtArray, i: u32) -> AtRGB {
    unsafe {
        ai_bindings::AiArrayGetRGBFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetRGBA(a: *const AtArray, i: u32) -> AtRGBA {
    unsafe {
        ai_bindings::AiArrayGetRGBAFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetVec2(a: *const AtArray, i: u32) -> AtVector2 {
    unsafe {
        ai_bindings::AiArrayGetVec2Func(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetVec(a: *const AtArray, i: u32) -> AtVector {
    unsafe {
        ai_bindings::AiArrayGetVecFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetMtx(a: *const AtArray, i: u32) -> AtMatrix {
    unsafe {
        ai_bindings::AiArrayGetMtxFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetStr(a: *const AtArray, i: u32) -> AtString {
    unsafe {
        ai_bindings::AiArrayGetStrFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetPtr(a: *const AtArray, i: u32) -> *mut ::std::os::raw::c_void {
    unsafe {
        ai_bindings::AiArrayGetPtrFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArrayGetArray(a: *const AtArray, i: u32) -> *mut AtArray {
    unsafe {
        ai_bindings::AiArrayGetArrayFunc(
            a,
            i,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetBool(a: *mut AtArray, i: u32, val: bool) -> bool {
    unsafe {
        ai_bindings::AiArraySetBoolFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetByte(a: *mut AtArray, i: u32, val: u8) -> bool {
    unsafe {
        ai_bindings::AiArraySetByteFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetInt(a: *mut AtArray, i: u32, val: ::std::os::raw::c_int) -> bool {
    unsafe {
        ai_bindings::AiArraySetIntFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetUInt(a: *mut AtArray, i: u32, val: u32) -> bool {
    unsafe {
        ai_bindings::AiArraySetUIntFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetFlt(a: *mut AtArray, i: u32, val: f32) -> bool {
    unsafe {
        ai_bindings::AiArraySetFltFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetRGB(a: *mut AtArray, i: u32, val: AtRGB) -> bool {
    unsafe {
        ai_bindings::AiArraySetRGBFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetRGBA(a: *mut AtArray, i: u32, val: AtRGBA) -> bool {
    unsafe {
        ai_bindings::AiArraySetRGBAFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetVec2(a: *mut AtArray, i: u32, val: AtVector2) -> bool {
    unsafe {
        ai_bindings::AiArraySetVec2Func(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetVec(a: *mut AtArray, i: u32, val: AtVector) -> bool {
    unsafe {
        ai_bindings::AiArraySetVecFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetMtx(a: *mut AtArray, i: u32, val: AtMatrix) -> bool {
    unsafe {
        ai_bindings::AiArraySetMtxFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetStr(a: *mut AtArray, i: u32, val: AtString) -> bool {
    unsafe {
        ai_bindings::AiArraySetStrFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetPtr(a: *mut AtArray, i: u32, val: *mut ::std::os::raw::c_void) -> bool {
    unsafe {
        ai_bindings::AiArraySetPtrFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}

pub fn AiArraySetArray(a: *mut AtArray, i: u32, val: *mut AtArray) -> bool {
    unsafe {
        ai_bindings::AiArraySetArrayFunc(
            a,
            i,
            val,
            CString::new(file!()).unwrap().as_ptr(),
            line!() as i32,
        )
    }
}
