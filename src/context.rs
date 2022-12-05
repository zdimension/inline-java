use j4rs::{ClasspathEntry, Instance, InvocationArg, Jvm, JvmBuilder};
use crate::JavaCode;

pub struct Context {
    pub(crate) jvm: Jvm,
    jshell: Instance
}

const JAR: &[u8] = include_bytes!("../target/debug/inline-java.jar");

impl Context {
    pub fn new() -> Self {
        // write JAR to temp file
        let jar_path = std::env::temp_dir().join("inline-java.jar");
        std::fs::write(&jar_path, JAR).unwrap();
        let entry = ClasspathEntry::new(jar_path.to_str().unwrap());

        let jvm = JvmBuilder::new()
            .classpath_entry(entry)
            .build()
            .unwrap();

        let jshell = jvm.create_instance("JShellFix", &Vec::new()).unwrap();
        //let jshell = jvm.invoke_static("jdk.jshell.JShell", "create", &Vec::new()).unwrap();

        Self {
            jvm,
            jshell
        }
    }
/*
    pub fn get(&self, name: &str) -> Instance {
        //self.jvm.invoke(&self.jshell, "eval", &vec![name]).unwrap()
        todo!()
    }

    pub fn set<T>(&self, name: &str, value: T) {
        todo!()
    }
*/
    pub fn run(&self, code: JavaCode) -> Instance {
        let list = self.jvm.invoke(&self.jshell, "eval", &vec![
            InvocationArg::try_from(code.code).unwrap()
        ]).unwrap();
        list
        /*let first = self.jvm.invoke(&list, "get", &vec![
            InvocationArg::try_from(0).unwrap().into_primitive().unwrap()
        ]).unwrap();
        let event = self.jvm.cast(&first, "jdk.jshell.SnippetEvent").unwrap();
        let value = self.jvm.invoke(&event, "value", &Vec::new()).unwrap();
        value*/
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
