#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::{c_int, c_uint, c_void, c_char, c_ushort};
use std::mem;

type BOOL = c_int;
type UINT = c_uint;
type WORD = c_ushort;
type LPVOID = *mut c_void;

type CHAR = c_char;
type PCSTR = *const CHAR;
type PCZZSTR = *const CHAR;

type HWND = LPVOID;
type FILEOP_FLAGS = WORD;

const FO_DELETE: WORD = 0x0003;
const FOF_ALLOWUNDO: WORD = 0x0040;

#[repr(C)]
pub struct SHFILEOPSTRUCTA {
    pub hwnd: HWND,
    pub wFunc: UINT,
    pub pFrom: PCZZSTR,
    pub pTo: PCZZSTR,
    pub fFlags: FILEOP_FLAGS,
    pub fAnyOperationsAborted: BOOL,
    pub hNameMappings: LPVOID,
    pub lpszProgressTitle: PCSTR,
}

type LPSHFILEOPSTRUCTA = *mut SHFILEOPSTRUCTA;

#[link(name="shell32")]
extern "system" {
    fn SHFileOperationA(lpFileOp: LPSHFILEOPSTRUCTA) -> c_int;
}

pub fn recycle(paths: &[&str]) -> bool {
    let mut bytes: Vec<u8> = Vec::new();
    for s in paths {
        bytes.extend_from_slice(s.as_bytes());
        bytes.push(0);
    }
    bytes.push(0);
    let mut shfileops: SHFILEOPSTRUCTA = unsafe { mem::zeroed() };
    shfileops.wFunc = FO_DELETE as UINT;
    shfileops.pFrom = bytes.as_ptr() as PCZZSTR;
    shfileops.fFlags = FOF_ALLOWUNDO;
    let res = unsafe { SHFileOperationA(&mut shfileops as *mut _) };
    res == 0
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let names = ["test0.txt", "test1.txt", "test2.txt"];
        for name in &names {
            if let Some(_) = name.find("2") {
                std::fs::create_dir(name).expect("Could not create dir");
            } else {
                std::fs::write(name, b"testing").expect("Could not write file");
            }
        }
        super::recycle(&names);
    }
}
