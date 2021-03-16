package org.mvnsearch;

import jdk.incubator.foreign.*;

import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodType;
import java.nio.charset.StandardCharsets;
import java.util.Optional;

import static jdk.incubator.foreign.CLinker.C_INT;
import static jdk.incubator.foreign.CLinker.C_POINTER;

public class ForeignApp {
    public static void main(String[] args) {
        LibraryLookup mylib = LibraryLookup.ofLibrary("mylib");
        callPrimitiveParam(mylib);
        callStringParam(mylib);
    }

    public static void callPrimitiveParam(LibraryLookup mylib) {

        Optional<LibraryLookup.Symbol> hello = mylib.lookup("Java_org_mvnsearch_RustService_increase");
        MethodHandle ffiFunction = CLinker.getInstance().downcallHandle(
                hello.get(),
                MethodType.methodType(int.class, int.class),
                FunctionDescriptor.of(C_INT, C_INT)
        );
        try {
            int len = (int) ffiFunction.invokeExact(111); // 5
            System.out.println(len);
        } catch (Throwable throwable) {
            throwable.printStackTrace();
        }
    }

    public static void callStringParam(LibraryLookup mylib) {
        Optional<LibraryLookup.Symbol> hello = mylib.lookup("Java_org_mvnsearch_RustService_echo");
        MethodHandle ffiFunction = CLinker.getInstance().downcallHandle(
                hello.get(),
                MethodType.methodType(int.class, MemoryAddress.class),
                FunctionDescriptor.of(C_INT, C_POINTER)
        );
        try (MemorySegment str = CLinker.toCString("Leijuan", StandardCharsets.UTF_8)) {
            int byteCount = (int) ffiFunction.invokeExact(str.address()); // 5
            System.out.println(byteCount);
        } catch (Throwable e) {
            e.printStackTrace();
        }
    }
}
