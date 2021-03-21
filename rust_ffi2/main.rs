
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]


use std::env;
use std::convert::TryInto;
use std::ptr;
use std::ptr::null_mut;

use std::ffi::{CString, c_void};
//use std::fs;
use std::fs::File;
use std::io::Read;

mod ffi;

pub use ffi::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct main_context {
    //pub alloc: hl_alloc,
    //pub functions_ptrs: *mut *mut ::std::os::raw::c_void,
    //pub functions_types: *mut *mut hl_type,

	//hl_code *code;
	//hl_module *m;
	//vdynamic *ret;
	//pchar *file;
	//int file_time;
    //

    pub code: *mut hl_code,
    pub m: *mut hl_module,
    //pub ret: *mut vdynamic,
    //pub file: *const ::std::os::raw::c_char,
    //pub file_time: ::std::os::raw::c_int,
}


fn main() {

   let args: Vec<_> = env::args().collect();

   println!("args len: {}",args.len());

   if args.len() == 1 {
      println!("usage:  main.exe <filename path>");
      std::process::exit(0);

   }

   let ref fname_in = args[1];

   unsafe {
        let argv: [CString; 0] = [];
        let hlfile = CString::new("").expect("could not convert file contents to CString");
        let hot_reload: u8  = 0;

        hl_global_init();

        /*
         * extern "C" {
                  pub fn hl_sys_init(
                      args: *mut *mut ::std::os::raw::c_void,
                      nargs: ::std::os::raw::c_int,
                      hlfile: *mut ::std::os::raw::c_void,
                  );
              }
        */
        hl_sys_init(argv.as_ptr() as *mut *mut c_void, argv.len() as i32, hlfile.as_ptr() as *mut c_void);

//        let ctx = main_context{};
//        hl_register_thread(&ctx);

        //let fdata = CString::new("").unwrap();


        //let mut file = File::open("../haxe/hello.hl").unwrap();
        let mut file = File::open(fname_in).unwrap();
        let mut buf = Vec::new();
        let _ = file.read_to_end(&mut buf).unwrap();
        //let fdata = CString::new(buf).unwrap();
        let fdata = &buf[..];
        //println!("data:{:?}", fdata);

        let error_msg = CString::new("").unwrap();
        /*
         * extern "C" {
                    pub fn hl_code_read(
                        data: *const ::std::os::raw::c_uchar,
                        size: ::std::os::raw::c_int,
                        error_msg: *mut *mut ::std::os::raw::c_char,
                    ) -> *mut hl_code;
                }
        */
        //let code = hl_code_read(fdata.as_ptr() as *const u8, fdata.to_bytes().len() as i32, error_msg.as_ptr() as *mut *mut i8);
        let code = hl_code_read(fdata.as_ptr() as *const u8, fdata.len() as i32, error_msg.as_ptr() as *mut *mut i8);
        //println!("err:{:?}", error_msg);
        //println!("code ok:{:?}", (*code).strings_lens);
        let module = hl_module_alloc(code);
        let r = hl_module_init(module,hot_reload) ;
        //println!("module_init:{}",r);
        //println!("module_reloar:{}",hot_reload);
        hl_code_free(code);
        //cl.t = ctx.code->functions[ctx.m->functions_indexes[ctx.m->code->entrypoint]].type;
        //let t = (*code).functions[(*module).functions_indexes[(*(*module).code).entrypoint]].r#type;
 /*       
        let e = (*(*module).code).entrypoint;
        println!("entrypoint:{}",e);
        println!("e:{}",type_of(&e));
        println!("m:{}",type_of((*module).functions_indexes));

        let i = (*(*module).functions_indexes) + e;

        println!("i:{}",i);
        let ii:isize = i.try_into().unwrap();
        println!("ii:{}",ii);

        let t = (*((*code).functions).offset(ii)).type_;

//        //cl.fun = ctx.m->functions_ptrs[ctx.m->code->entrypoint];
        let mut fun = (*((*module).functions_ptrs).offset(ii));
        println!("t:{:?}",t);
        println!("fun:{:?}",fun);
        println!("fun:{:?}",*fun);
*/
        let e = (*(*module).code).entrypoint;
        println!("e:{}",type_of(&e));
        println!("code->entrypoint:{}",e);
        let e_:isize = e.try_into().unwrap();
        let fi = (*(*module).functions_indexes);
        println!("functions indexes base:{}",fi);
        println!("functions indexes :{:?}",*(*module).functions_indexes.offset(e_));
        let mfi = *(*module).functions_indexes.offset(e_);
        let mfi_:isize = mfi.try_into().unwrap();
        let tt = *((*code).functions).offset(mfi_);
        //println!("functions :{}",tt);

        //let i = fi + e;
        //let ii:isize = i.try_into().unwrap();
        let t = (*((*code).functions).offset(mfi_)).type_;
        let t2 = *((*((*code).functions).offset(mfi_)).type_);
        let mut fun = (*((*module).functions_ptrs).offset(e_));
        //let  mut fun = ((*module).functions_ptrs).offset(e_).as_mut_ptr() as *mut c_void;
        println!("fun:{}",type_of(&fun));

        //*t = 0x15a;
        //*fun = 0xccf54920;

        //println!("t:{:?}",t2);
        println!("fun:{:?}",fun);
        println!("fun:{:?}",*fun );
        dbg!(t);
        dbg!(fun);

        let hasValue: ::std::os::raw::c_int = 0;
        let stackCount: ::std::os::raw::c_int = 0;
        let mut vec = vec!(1, 2, 3);
        let value : *mut ::std::os::raw::c_void = vec.as_mut_ptr() as *mut c_void;;

        let mut cl = _vclosure{t,fun,hasValue, stackCount, value};
        let mut isExc = vec!(0);
        //let mut null: *const i32 = ptr::null();
        //
        //
        //
        //
        //let mut state = State { state: 20 };
        //let state_ptr: *mut c_void = &mut state as *mut _ as *mut c_void;
        //tsm_screen_draw(con, my_callback, state_ptr);

        let mut ctx = main_context{code:code,m:module};
        let ctx_ptr: *mut c_void = &mut ctx as *mut _ as *mut c_void; 
        hl_register_thread(ctx_ptr);

        let profile_count:i32 = -1;
        hl_profile_setup(profile_count);

        /*
             
             pub fn hl_dyn_call_safe(
                     c: *mut vclosure,
                     args: *mut *mut vdynamic,
                     nargs: ::std::os::raw::c_int,
                     isException: *mut bool_,
                 ) -> *mut vdynamic;


              pub struct vdynamic {
                  pub t: *mut hl_type,
                  pub v: vdynamic__bindgen_ty_1,
              }
              pub union vdynamic__bindgen_ty_1 {
                  pub b: bool_,
                  pub ui8: ::std::os::raw::c_uchar,
                  pub ui16: ::std::os::raw::c_ushort,
                  pub i: ::std::os::raw::c_int,
                  pub f: f32,
                  pub d: f64,
                  pub bytes: *mut vbyte,
                  pub ptr: *mut ::std::os::raw::c_void,
                  pub i64: int64,
                  _bindgen_union_align: u64,
                  }
           */

        hl_dyn_call_safe(&mut cl,null_mut(),0,isExc.as_mut_ptr());

        /* args call test
         *
         let mut ht = hl_type{ kind:  hl_type_kind_HI32, 
                               __bindgen_anon_1: hl_type__bindgen_ty_1{ abs_name:null_mut()},
                               vobj_proto: null_mut(),
                               mark_bits: null_mut()};
         let mut v = vdynamic{t: &mut ht,
                          v: vdynamic__bindgen_ty_1{i:999}
                         };
        let mut raw_v = &mut v as *mut vdynamic; 

        hl_dyn_call_safe(&mut cl,&mut raw_v,2,isExc.as_mut_ptr());
        */

        //hl_dyn_call_safe(&mut cl,null_mut(),0,null_mut());

        //let x: i32 = 346;
        //println!("x:{}",type_of(&x));
        //let f = (*module).functions_indexes[&x];
        //println!("{}",f);
        //println!("{}",type_of(&f));
        
        // dbg!(code);
    }
}

fn type_of<T>(_: T) -> String{
  let a = std::any::type_name::<T>();
  return a.to_string();
}
