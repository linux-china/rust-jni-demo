package org.mvnsearch;


import org.junit.jupiter.api.Test;

public class RustServiceTest {

    @Test
    public void testHello() {
        System.out.println(RustService.hello("Jackie"));
    }

    @Test
    public void testIncrement() {
        System.out.println(RustService.count("AA"));
    }
}
