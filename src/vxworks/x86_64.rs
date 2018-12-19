pub type c_long = i64;
pub type c_ulong = u64;

#[cfg(feature = "_WRS_KERNEL")]
pub type _Vx_TASK_ID = *mut ::windTcb;

#[cfg(feature = "_WRS_KERNEL")]
s! {
    pub struct OBJ_CORE {
        pub handle        : ::HANDLE,
        pub ownerList     : ::DL_LIST,
        pub ownerNode     : ::DL_NODE,
        pub classNode     : ::DL_NODE,
        pub ownerId       : *mut ::OBJ_CORE,
        pub ownerRtpId    : ::RTP_ID,
        pub name          : *mut ::c_char,
        pub pObjClass     : *mut ::wind_class,
        pub objHandleList : ::DL_LIST,
        pub refCnt        : u16,
        pub accessCnt     : u16,
        pub padding       : u32, // There is a chance that Rust automatically pads, but
                                 // no point in risking it 
    }

    // semLibP.h
    pub struct semaphore {
        #[repr(align(16))]
        pub magic          : ::OBJ_CORE,
        pub semType        : u8,
        pub options        : u8,
        pub recurse        : u16,
        pub priInheritFlag : ::BOOL,
        pub qHead          : ::Q_HEAD,
        pub state          : ::size_t, //state is union of UINT and struct pointer
        pub events         : ::EVENTS_RSRC,
    }
    
}
