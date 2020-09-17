Rust JNI Demo
=============

使用Rust编写Java JNI库，Java已经非常🐂啦，为何还要用Rust来写一些代码？

* WebAssembly还有一些问题
* Java还是不能访问一些底层的指令，如SIMD，性能提升非常明显
* Rust也有非常强大的crates库 https://crates.io/ 自行选择

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