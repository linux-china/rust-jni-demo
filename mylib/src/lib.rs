// This is the interface to the JVM that we'll call the majority of our methods on.
use jni::JNIEnv;
// These objects are what you should use as arguments to your native
// function. They carry extra lifetime information to prevent them escaping
// this context and getting used after being GC'd.
use jni::objects::{JClass, JString};

// This is the class that owns our static method. It's not going to be used,
// but still must be present to match the expected signature of a static native method.
#[no_mangle]
pub extern "system" fn Java_org_mvnsearch_RustService_hello<'local>(mut env: JNIEnv<'local>,
                                                                    _class: JClass<'local>,
                                                                    name: JString<'local>)
                                                                    -> JString<'local> {
    // convert Java's type to Rust's type
    let name: String = env.get_string(&name).expect("Couldn't get java string!").into();
    println!("Received: {}", &name);
    // call Rust native function
    let result = hello(&name);

    // Finally, extract the raw pointer to return.
    env.new_string(result).expect("Couldn't create java string!")
}

fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}

#[no_mangle]
pub extern "system" fn Java_org_mvnsearch_RustService_count(mut env: JNIEnv,
                                                            _class: JClass,
                                                            name: JString)
                                                            -> i32 {
    let name: String = env.get_string(&name).expect("Couldn't get java string!").into();
    println!("Received: {}", &name);
    name.len() as i32
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
