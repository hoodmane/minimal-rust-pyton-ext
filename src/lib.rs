pub type Py_ssize_t = isize;
use std::os::raw::{c_int, c_uchar, c_void};

// opaque struct
#[repr(C)] 
pub struct PyTypeObject { private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct PyObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
}

pub const PyObject_HEAD_INIT: PyObject = PyObject {
    ob_refcnt: 1,
    ob_type: std::ptr::null_mut(),
};


#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyModuleDef_Base {
    pub ob_base: PyObject,
    pub m_init: Option<extern "C" fn() -> *mut PyObject>,
    pub m_index: Py_ssize_t,
    pub m_copy: *mut PyObject,
}

pub const PyModuleDef_HEAD_INIT: PyModuleDef_Base = PyModuleDef_Base {
    ob_base: PyObject_HEAD_INIT,
    m_init: None,
    m_index: 0,
    m_copy: std::ptr::null_mut(),
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyModuleDef_Slot {
    pub slot: c_int,
    pub value: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyModuleDef {
    pub m_base: PyModuleDef_Base,
    pub m_name: *const c_uchar,
    pub m_doc: *const c_uchar,
    pub m_size: Py_ssize_t,
    pub m_methods: *mut PyMethodDef,
    pub m_slots: *mut PyModuleDef_Slot,
    pub m_traverse: Option<traverseproc>,
    pub m_clear: Option<inquiry>,
    pub m_free: Option<freefunc>,
}

pub type inquiry = unsafe extern "C" fn(arg1: *mut PyObject) -> c_int;
pub type visitproc = unsafe extern "C" fn(object: *mut PyObject, arg: *mut c_void) -> c_int;
pub type traverseproc =
    unsafe extern "C" fn(slf: *mut PyObject, visit: visitproc, arg: *mut c_void) -> c_int;
pub type freefunc = unsafe extern "C" fn(arg1: *mut c_void);

unsafe fn Py_DECREF(obj : *mut PyObject ){
    (*obj).ob_refcnt -= 1;
}

pub const METH_VARARGS: c_int = 0x0001;
pub const METH_KEYWORDS: c_int = 0x0002;
/* METH_NOARGS and METH_O must not be combined with the flags above. */
pub const METH_NOARGS: c_int = 0x0004;
pub const METH_O: c_int = 0x0008;

pub type PyCFunction =
    unsafe extern "C" fn(slf: *mut PyObject, args: *mut PyObject) -> *mut PyObject;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PyMethodDef {
    pub ml_name: *const c_uchar,
    pub ml_meth: Option<PyCFunction>,
    pub ml_flags: c_int,
    pub ml_doc: *const c_uchar,
}

// impl !Sync for PyMethodDef {}


extern "C" {
    pub fn PyModule_NewObject(name: *mut PyObject) -> *mut PyObject;
    pub fn PyModule_New(name: *const c_uchar) -> *mut PyObject;
    pub fn PyModule_Create2(module: *mut PyModuleDef, abi : c_int) -> *mut PyObject;
    pub fn PyObject_Print(o: *mut PyObject, fp: c_int, flags: c_int) -> c_int;
    pub fn PyCFunction_NewEx(
        ml: *const PyMethodDef,
        module: *mut PyObject,
        module_name : *mut PyObject,
    ) -> *mut PyObject;
    pub fn PyUnicode_FromString(u: *const c_uchar) -> *mut PyObject;
    pub fn PyObject_SetAttrString(obj : *mut PyObject, key : *const c_uchar, value : *mut PyObject) -> c_int;
}

pub unsafe extern "C" fn my_func(
    slf: *mut PyObject,
    arg: *mut PyObject,
) -> *mut PyObject {
    PyUnicode_FromString("hello there?\0".as_ptr())
}


#[no_mangle]
pub unsafe extern "C" fn PyInit_mymod() -> *mut PyObject
{
    let mut m_methods = [
        // PyMethodDef {
        //     ml_name : "my_func".as_ptr(),
        //     ml_meth : Some(my_func),
        //     ml_flags : METH_NOARGS,
        //     ml_doc : "Says hello".as_ptr() 
        // },
        PyMethodDef {
            ml_name : std::ptr::null(),
            ml_meth : None,
            ml_flags : 0,
            ml_doc : std::ptr::null(),
        },
    ];
    println!("hi?");
    // let mut mod_def = PyModuleDef {
    //     m_base : PyModuleDef_HEAD_INIT,
    //     m_name: "mymod\0".as_ptr(),
    //     m_doc: "A test!\0".as_ptr(),
    //     m_size: -1,
    //     m_methods : &mut m_methods as *mut PyMethodDef,
    //     m_slots: unsafe { std::mem::transmute(0usize) },
    //     m_traverse: None,
    //     m_clear: None,
    //     m_free: None,
    // };

    const INIT: PyModuleDef = PyModuleDef {
        m_base: PyModuleDef_HEAD_INIT,
        m_name: std::ptr::null(),
        m_doc: std::ptr::null(),
        m_size: 0,
        m_methods: std::ptr::null_mut(),
        m_slots: std::ptr::null_mut(),
        m_traverse: None,
        m_clear: None,
        m_free: None,
    };

    const mod_def : PyModuleDef = PyModuleDef {
        m_name: "mymod\0".as_ptr() as *const _,
        m_doc: "A test!\0".as_ptr() as *const _,
        ..INIT
    };
    PyModule_Create2(&mut mod_def as *mut PyModuleDef, 1013)

    // println!("1");
    // let module = PyModule_New("mymod".as_ptr());
    // println!("2");
    // // PyObject_Print(module, 1, 0);
    // println!("3");

    // let nameobj = PyUnicode_FromString("mymod".as_ptr());
    // if nameobj.is_null(){
    //     return std::ptr::null_mut();
    // }
    // println!("3");

    // // PyObject_Print(nameobj, 1, 0);
    // if nameobj.is_null() {
    //     return std::ptr::null_mut();
    // }        
    // println!("4");
    // if module.is_null() {
    //     return std::ptr::null_mut();
    // }
    // println!("5");
    // let func = PyCFunction_NewEx((&fdef) as *const PyMethodDef, module, nameobj);
    // println!("6");
    // if func.is_null() {
    //     Py_DECREF(module);
    //     return std::ptr::null_mut();
    // }
    // println!("7");
    // if PyObject_SetAttrString(module, fdef.ml_name, func) != 0 {
    //     Py_DECREF(module);
    //     Py_DECREF(func);
    //     return std::ptr::null_mut();
    // }
    // println!("8");
    // Py_DECREF(func);
    // return module;
}


