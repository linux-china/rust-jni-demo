package org.mvnsearch;

public class HelloWorldApp {

    public static void main(String[] args) {
        String welcome = RustService.hello("Jackie");
        System.out.println(welcome);
    }
}
