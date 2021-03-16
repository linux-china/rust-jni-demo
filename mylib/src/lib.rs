use std::ffi::CStr;

// This is the interface to the JVM that we'll call the majority of our methods on.
use jni::JNIEnv;
// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};
// This is just a pointer. We'll be returning it from our function. We
// can't return one of the objects with lifetime information because the
// lifetime checker won't let us.
use jni::sys::{jbyteArray, jint, jlong, jstring};
use libc::{c_char, c_int};

// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static native method.
#[no_mangle]
pub extern "system" fn Java_org_mvnsearch_RustService_hello(env: JNIEnv,
                                                            _class: JClass,
                                                            name: JString)
                                                            -> jstring {
    // convert Java's type to Rust's type
    let name: String = env.get_string(name).expect("Couldn't get java string!").into();
    println!("Received: {}", &name);
    // call Rust native function
    let result = hello(&name);

    // Finally, extract the raw pointer to return.
    env.new_string(result).expect("Couldn't create java string!").into_inner()
}

fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}

#[no_mangle]
pub extern "system" fn Java_org_mvnsearch_RustService_count(env: JNIEnv,
                                                            _class: JClass,
                                                            name: JString)
                                                            -> i32 {
    let name: String = env.get_string(name).expect("Couldn't get java string!").into();
    println!("Received: {}", &name);
    name.len() as i32
}


// Rust FFI standard
#[no_mangle]
pub extern "C" fn Java_org_mvnsearch_RustService_increase(base: c_int) -> c_int {
    base + 1
}

#[no_mangle]
pub extern "C" fn Java_org_mvnsearch_RustService_echo(s: *const c_char) -> c_int {
    let c_str = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as c_int
}


#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn test_hello() {
        let welcome = hello("Jackie");
        println!("{}", welcome);
    }
}
