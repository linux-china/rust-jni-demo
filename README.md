Rust JNI Demo
=============

使用Rust编写Java JNI库，Java已经非常🐂啦，为何还要用Rust来写一些代码？

* WebAssembly支持方面Java还有一些问题，通过Rust可能更方便一些
* Java还是不能访问一些底层的API，如SIMD，通过Rust性能提升可能非常明显
* Rust也有非常强大的crates库 https://crates.io/ 自行选择
* JEP 389: Foreign Linker API: https://openjdk.java.net/jeps/389

# 如何生成Rust JNI函数钩子
在Rust的lib.rs文件中，我们要实现JNI函数，那么这个函数的声明样式什么样的？ 如函数名、签名等。

* 首先我们根据Java类的 static native 函数生成对应的".h"文件，如下：

```
$ javac -h . src/main/java/org/mvnsearch/RustService.java
```

* 在生成对应的".h"文件中，找到对应的声明函数，如：

```
JNIEXPORT jstring JNICALL Java_org_mvnsearch_RustService_hello
  (JNIEnv *, jclass, jstring);
```

* 接下来在lib.rs添加对应的声明函数，然后实现其逻辑即可。

```
pub extern "system" fn Java_org_mvnsearch_RustService_hello(env: JNIEnv,
                                                            _class: JClass,
                                                            name: JString)
                                                            -> jstring {
```

考虑到时间测试的需要，个人建议是编写另外一个完全Rust Native的函数声明，然后Rust JNI函数钩子调用Rust Native函数，其目的主要是方便单元测试，如下：

```rust
fn hello(name: &str) -> String {
    format!("Hello {}!", name)
}
```

# 动态链接库加载目录
Java的`System.load`会从指定的目录加载，默认目录列表为System.getProperty("java.library.path")，你可以使用jshell查询，如下：

```
$ jshell
jshell> System.getProperty("java.library.path")
$1 ==> "~/Library/Java/Extensions:/Library/Java/Extensions:/Network/Library/Java/Extensions:/System/Library/Java/Extensions:/usr/lib/java:."
jshell> /exit
```

然后将rust对应的开发包拷贝到上述之一目录就可以，如在我Mac笔记本下，命令如下：

```
$ cp -rf mylib/target/debug/libmylib.dylib ~/Library/Java/Extensions/
```

在测试的时候，我们可以通过设置java.library.path属性进行调整，只要指向Rust library的debug目录就可以，如下：

```
-Djava.library.path=/Users/xxxx/rust-jni-demo/mylib/target/debug
```

在IDE工具中，都有对应的VM options方便你进行设置。

# References

* jni-rs: Rust bindings to the JNI https://github.com/jni-rs/jni-rs
* jni-rs Docs:  https://docs.rs/jni/0.17.0/jni/
* JNI tips — general tips on JNI development and some Android-specific https://developer.android.com/training/articles/perf-jni
* Best practices for using the Java Native Interface: https://developer.ibm.com/articles/j-jni/
* Java Native Interface Specification: https://docs.oracle.com/javase/9/docs/specs/jni/index.html
* jni.h: https://github.com/openjdk/jdk/blob/master/src/java.base/share/native/include/jni.h
* wasmer-java: WebAssembly runtime for Java: https://github.com/wasmerio/wasmer-java
* wasmtime-java: Java or JVM-language binding for Wasmtime: https://github.com/kawamuray/wasmtime-java


