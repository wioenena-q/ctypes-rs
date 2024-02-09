use crate::def_type;

def_type!(ATOM, WORD);
def_type!(BOOL, crate::core::int_t);
def_type!(BOOLEAN, BYTE);
def_type!(BYTE, crate::core::u_char_t);
def_type!(CCHAR, crate::core::char_t);
def_type!(CHAR, crate::core::char_t);
def_type!(COLORREF, DWORD);
def_type!(DWORD, crate::core::u_long_t);
def_type!(DWORDLONG, crate::core::u_long_t);
def_type!(DWORD_PTR, ULONG_PTR);
def_type!(DWORD32, crate::core::u_int_t);
def_type!(DWORD64, crate::core::u_long_t);
def_type!(FLOAT, crate::core::float_t);
def_type!(HACCEL, HANDLE);
#[cfg(target_pointer_width = "32")]
def_type!(HALF_PTR, crate::core::short_t);
#[cfg(target_pointer_width = "64")]
def_type!(HALF_PTR, crate::core::int_t);
def_type!(HANDLE, PVOID);
def_type!(HBITMAP, HANDLE);
def_type!(HBRUSH, HANDLE);
def_type!(HCOLORSPACE, HANDLE);
def_type!(HCONV, HANDLE);
def_type!(HCONVLIST, HANDLE);
def_type!(HCURSOR, HICON);
def_type!(HDC, HANDLE);
def_type!(HDDEDATA, HANDLE);
def_type!(HDESK, HANDLE);
def_type!(HDROP, HANDLE);
def_type!(HDWP, HANDLE);
def_type!(HENHMETAFILE, HANDLE);
def_type!(HFILE, crate::core::int_t);
def_type!(HFONT, HANDLE);
def_type!(HGDIOBJ, HANDLE);
def_type!(HGLOBAL, HANDLE);
def_type!(HHOOK, HANDLE);
def_type!(HICON, HANDLE);
def_type!(HINSTANCE, HANDLE);
def_type!(HKEY, HANDLE);
def_type!(HKL, HANDLE);
def_type!(HLOCAL, HANDLE);
def_type!(HMENU, HANDLE);
def_type!(HMETAFILE, HANDLE);
def_type!(HMODULE, HINSTANCE);
def_type!(HMONITOR, HANDLE);
def_type!(HPALETTE, HANDLE);
def_type!(HPEN, HANDLE);
def_type!(HRESULT, LONG);
def_type!(HRGN, HANDLE);
def_type!(HRSRC, HANDLE);
def_type!(HSZ, HANDLE);
def_type!(HWINSTA, HANDLE);
def_type!(HWND, HANDLE);
def_type!(INT, crate::core::int_t);
def_type!(INT_PTR, crate::core::long_t);
def_type!(INT8, crate::core::char_t);
def_type!(INT16, crate::core::short_t);
def_type!(INT32, crate::core::int_t);
def_type!(INT64, crate::core::long_t);
def_type!(LANGID, WORD);
def_type!(LCID, DWORD);
def_type!(LCTYPE, DWORD);
def_type!(LGRPID, DWORD);
def_type!(LONG, crate::core::long_t);
def_type!(LONGLONG, crate::core::long_t);
def_type!(LONG_PTR, crate::core::long_t);
def_type!(LONG32, crate::core::int_t);
def_type!(LONG64, crate::core::long_t);
def_type!(LPARAM, LONG_PTR);
def_type!(LPBOOL, *mut BOOL);
def_type!(LPBYTE, *mut BYTE);
def_type!(LPCOLORREF, *mut DWORD);
def_type!(LPCSTR, *const CHAR);
def_type!(LPCTSTR, LPCSTR);
def_type!(LPCVOID, crate::core::ptr::void_ptr_t_mut);
def_type!(LPCWSTR, *const WCHAR);
def_type!(LPDWORD, *mut DWORD);
def_type!(LPHANDLE, *mut HANDLE);
def_type!(LPINT, *mut INT);
def_type!(LPLONG, *mut LONG);
def_type!(LPSTR, *mut CHAR);
def_type!(LPTSTR, LPSTR);
def_type!(LPVOID, crate::core::ptr::void_ptr_t_mut);
def_type!(LPWORD, *mut WORD);
def_type!(LPWSTR, *mut WCHAR);
def_type!(LRESULT, LONG_PTR);
def_type!(PBOOL, *mut BOOL);
def_type!(PBOOLEAN, *mut BOOLEAN);
def_type!(PBYTE, *mut BYTE);
def_type!(PCHAR, *mut CHAR);
def_type!(PCSTR, *const CHAR);
def_type!(PCTSTR, LPCSTR);
def_type!(PCWSTR, *const WCHAR);
def_type!(PDWORD, *mut DWORD);
def_type!(PDWORDLONG, *mut DWORDLONG);
def_type!(PDWORD_PTR, *mut DWORD_PTR);
def_type!(PDWORD32, *mut DWORD32);
def_type!(PDWORD64, *mut DWORD64);
def_type!(PFLOAT, *mut FLOAT);
def_type!(PHALF_PTR, *mut HALF_PTR);
def_type!(PHANDLE, *mut HANDLE);
def_type!(PHKEY, *mut HKEY);
def_type!(PINT, *mut INT);
def_type!(PINT_PTR, *mut INT_PTR);
def_type!(PINT8, *mut INT8);
def_type!(PINT16, *mut INT16);
def_type!(PINT32, *mut INT32);
def_type!(PINT64, *mut INT64);
def_type!(PLCID, PDWORD);
def_type!(PLONG, *mut LONG);
def_type!(PLONGLONG, *mut LONGLONG);
def_type!(PLONG_PTR, *mut LONG_PTR);
def_type!(PLONG32, *mut LONG32);
def_type!(PLONG64, *mut LONG64);
def_type!(POINTER_32, *mut PINT32);
def_type!(POINTER_64, *mut PINT64);
def_type!(POINTER_SIGNED, *mut INT_PTR);
def_type!(POINTER_UNSIGNED, *mut UINT_PTR);
def_type!(PSHORT, *mut SHORT);
def_type!(PSIZE_T, *mut SIZE_T);
def_type!(PSSIZE_T, *mut SSIZE_T);
def_type!(PSTR, *mut CHAR);
def_type!(PTBYTE, *mut TBYTE);
def_type!(PTCHAR, *mut TCHAR);
def_type!(PTSTR, LPSTR);
def_type!(PUCHAR, *mut UCHAR);
def_type!(PUHALF_PTR, *mut UHALF_PTR);
def_type!(PUINT, *mut UINT);
def_type!(PUINT_PTR, *mut UINT_PTR);
def_type!(PUINT8, *mut UINT8);
def_type!(PUINT16, *mut UINT16);
def_type!(PUINT32, *mut UINT32);
def_type!(PUINT64, *mut UINT64);
def_type!(PULONG, *mut LONG);
def_type!(PULONGLONG, *mut ULONGLONG);
def_type!(PULONG_PTR, *mut ULONG_PTR);
def_type!(PULONG32, *mut ULONG32);
def_type!(PULONG64, *mut ULONG64);
def_type!(PUSHORT, *mut USHORT);
def_type!(PVOID, crate::core::ptr::void_ptr_t_mut);
def_type!(PWCHAR, *mut WCHAR);
def_type!(PWORD, *mut WORD);
def_type!(PWSTR, *mut WCHAR);
def_type!(QWORD, crate::core::u_long_t);
def_type!(SC_HANDLE, HANDLE);
def_type!(SC_LOCK, LPVOID);
def_type!(SERVICE_STATUS_HANDLE, HANDLE);
def_type!(SHORT, crate::core::short_t);
def_type!(SIZE_T, ULONG_PTR);
def_type!(SSIZE_T, LONG_PTR);
def_type!(TBYTE, crate::core::u_char_t);
def_type!(TCHAR, crate::core::char_t);
def_type!(UCHAR, crate::core::u_char_t);
#[cfg(target_pointer_width = "32")]
def_type!(UHALF_PTR, crate::core::u_short_t);
#[cfg(target_pointer_width = "64")]
def_type!(UHALF_PTR, crate::core::u_int_t);
def_type!(UINT, crate::core::u_int_t);
#[cfg(target_pointer_width = "32")]
def_type!(UINT_PTR, crate::core::u_int_t);
#[cfg(target_pointer_width = "64")]
def_type!(UINT_PTR, crate::core::u_long_t);
def_type!(UINT8, crate::core::u_char_t);
def_type!(UINT16, crate::core::u_short_t);
def_type!(UINT32, crate::core::u_int_t);
def_type!(UINT64, crate::core::u_long_t);
def_type!(ULONG, crate::core::u_long_t);
def_type!(ULONGLONG, crate::core::u_long_t);
def_type!(ULONG_PTR, crate::core::u_long_t);
def_type!(ULONG32, crate::core::u_int_t);
def_type!(ULONG64, crate::core::u_long_t);

#[allow(non_snake_case)]
pub struct UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PWSTR,
}
pub type PUNICODE_STRING = *mut UNICODE_STRING;
pub type PCUNICODE_STRING = *const UNICODE_STRING;

def_type!(USHORT, crate::core::u_short_t);
def_type!(USN, LONGLONG);
def_type!(VOID, crate::core::void);
pub enum WCHAR {
    U16(crate::core::wchar16_t),
    U32(crate::core::wchar32_t),
}
def_type!(WORD, crate::core::u_short_t);
def_type!(WPARAM, UINT_PTR);
