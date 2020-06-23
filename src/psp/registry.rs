use super::c_void;

pub const SYSTEM_REGISTRY: [u8; 7] = *b"/system";
pub const REG_KEYNAME_SIZE: u32 = 27;

#[repr(transparent)]
#[allow(missing_copy_implementations)]
pub struct Handle(u32);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Key {
    pub key_type: KeyType,
    pub name: [u8; 256usize],
    pub name_len: u32,
    pub unk2: u32,
    pub unk3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum KeyType {
    Directory = 1,
    Integer = 2,
    String = 3,
    Bytes = 4,
}

extern {
    pub fn sceRegOpenRegistry(
        reg: *mut Key,
        mode: i32,
        handle: *mut Handle,
    ) -> i32;
    pub fn sceRegFlushRegistry(handle: Handle) -> i32;
    pub fn sceRegCloseRegistry(handle: Handle) -> i32;
    pub fn sceRegOpenCategory(
        handle: Handle,
        name: *const u8,
        mode: i32,
        dir_handle: *mut Handle,
    ) -> i32;
    pub fn sceRegRemoveCategory(
        handle: Handle,
        name: *const u8,
    ) -> i32;
    pub fn sceRegCloseCategory(dir_handle: Handle) -> i32;
    pub fn sceRegFlushCategory(dir_handle: Handle) -> i32;
    pub fn sceRegGetKeyInfo(
        dir_handle: Handle,
        name: *const u8,
        key_handle: *mut Handle,
        type_: *mut KeyType,
        size: *mut usize,
    ) -> i32;
    pub fn sceRegGetKeyInfoByName(
        dir_handle: Handle,
        name: *const u8,
        type_: *mut KeyType,
        size: *mut usize,
    ) -> i32;
    pub fn sceRegGetKeyValue(
        dir_handle: Handle,
        key_handle: Handle,
        buf: *mut c_void,
        size: usize,
    ) -> i32;
    pub fn sceRegGetKeyValueByName(
        dir_handle: Handle,
        name: *const u8,
        buf: *mut c_void,
        size: usize,
    ) -> i32;
    pub fn sceRegSetKeyValue(
        dir_handle: Handle,
        name: *const u8,
        buf: *const c_void,
        size: usize,
    ) -> i32;
    pub fn sceRegGetKeysNum(
        dir_handle: Handle,
        num: *mut i32,
    ) -> i32;
    pub fn sceRegGetKeys(
        dir_handle: Handle,
        buf: *mut u8,
        num: i32,
    ) -> i32;
    pub fn sceRegCreateKey(
        dir_handle: Handle,
        name: *const u8,
        type_: i32,
        size: usize,
    ) -> i32;
    pub fn sceRegRemoveRegistry(key: *mut Key) -> i32;
}
