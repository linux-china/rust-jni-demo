package org.mvnsearch;

/**
 * Rust Service by JNI
 *
 * @author linux_china
 */
public class RustService {
    static {
        System.loadLibrary("mylib");
    }

    /**
     * static hello function which is provided by a native library.
     *
     * @param name input
     * @return welcome
     */
    public static native String hello(String name);

    public static native int count(String name);
}
