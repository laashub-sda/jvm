#![allow(non_snake_case)]
use crate::native::{new_fn, JNIEnv, JNINativeMethod, JNIResult};
use crate::new_br;
use crate::oop::{self, Oop};
use crate::runtime::{self, exception, thread, JavaCall};
use classfile::consts as cls_consts;

pub fn get_native_methods() -> Vec<JNINativeMethod> {
    vec![
        new_fn(
            "doPrivileged",
            "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
            Box::new(jvm_doPrivileged),
        ),
        new_fn(
            "doPrivileged",
            "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
            Box::new(jvm_doPrivileged2),
        ),
        new_fn(
            "getStackAccessControlContext",
            "()Ljava/security/AccessControlContext;",
            Box::new(jvm_getStackAccessControlContext),
        ),
        new_fn("doPrivileged", "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;", Box::new(jvm_doPrivileged3)),
    ]
}

fn jvm_doPrivileged(_env: JNIEnv, args: &Vec<Oop>) -> JNIResult {
    let v = args.get(0).unwrap();

    let mir = {
        match v {
            Oop::Null => {
                let ex = exception::new(cls_consts::J_NPE, None);
                return Err(ex);
            }
            Oop::Ref(v) => {
                let inst = v.extract_inst();
                let cls = inst.class.get_class();
                cls.get_virtual_method(new_br("run"), new_br("()Ljava/lang/Object;"))
                    .unwrap()
            }
            _ => unreachable!(),
        }
    };

    let args = vec![v.clone()];
    let mut jc = JavaCall::new_with_args(mir, args);
    let area = runtime::DataArea::new(0, 1);
    jc.invoke(Some(&area), false);

    if !thread::is_meet_ex() {
        let mut stack = area.stack.borrow_mut();
        let r = stack.pop_ref();
        Ok(Some(r))
    } else {
        Ok(None)
    }
}

//todo: re impl
fn jvm_doPrivileged2(env: JNIEnv, args: &Vec<Oop>) -> JNIResult {
    jvm_doPrivileged(env, args)
}

//todo: re impl
fn jvm_doPrivileged3(env: JNIEnv, args: &Vec<Oop>) -> JNIResult {
    jvm_doPrivileged(env, args)
}

fn jvm_getStackAccessControlContext(_env: JNIEnv, _args: &Vec<Oop>) -> JNIResult {
    Ok(Some(oop::consts::get_null()))
}
