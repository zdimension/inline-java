#[cfg(test)]
mod tests {
    use inline_java::to;
    use inline_java_macros::java;

    #[test]
    fn it_works() {
        java! {
            System.out.println("Hello, world!");
        }

        assert!(to::<bool>(java! { 2 + 2 == 4 }));
        assert_eq!(4, to::<i32>(java! { 2 + 2 }));
        assert_eq!("Hello, world!", to::<String>(java! { "Hello" + ", world!" }));
    }

    #[test]
    fn context() {
        let c = inline_java::Context::new();
        c.run(java! {
            var x = 5;
        });
        c.run(java! {
            System.out.println(x);
        });
    }
}