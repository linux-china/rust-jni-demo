package org.mvnsearch;

import org.mvnsearch.jni.NativeLibraryLoader;

import java.io.IOException;

/**
 * Rust Service by JNI
 *
 * @author linux_china
 */
public class RustService {
    static {
        try {
            NativeLibraryLoader.getInstance().loadLibrary(null);
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
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
