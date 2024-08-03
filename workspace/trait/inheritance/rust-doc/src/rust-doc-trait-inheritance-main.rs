#![allow(dead_code, unused_variables)]

/// Main
///
/// ## Commands
///
/// ```RUST_BACKTRACE=full cargo run -q -p rust-doc-trait-inheritance_bin --bin rust-doc-trait-inheritance-main```
///
/// ```cargo doc  --package rust-doc-trait-inheritance_bin  --message-format short --no-deps --open --color always```
///
/// ```cargo test --doc  --package rust-doc-trait-inheritance_bin ```
///
/// ## What
/// `Supertrait-multi-trait`
///
/// ## How
/// `TODO`
///
/// # Arguments
///
/// * `Arg1` - This is the [your type] to [your verb] the [your struct/func name]
///
/// # Return
/// `nothing`
///
/// ## Example
///  `TODO`
///
///

trait Person {
    fn name(&self) -> String;
}
trait Student: Person {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
}
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
struct PersonImpl {
    name: String,
}
struct StudentImpl {
    person: PersonImpl,
    university: String,
}
struct ProgrammerImpl {
    fav_language: String,
}
struct CompSciStudentImpl {
    programmer: ProgrammerImpl,
    student: StudentImpl,
    git_username: String,
    name: String,
    university: String,
    fav_language: String,
}
impl Person for PersonImpl {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Person for CompSciStudentImpl {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl Person for StudentImpl {
    fn name(&self) -> String {
        self.name().clone()
    }
}
impl Student for StudentImpl {
    fn university(&self) -> String {
        self.university.clone()
    }
}
impl Programmer for ProgrammerImpl {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}
impl CompSciStudent for CompSciStudentImpl {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}
impl Programmer for CompSciStudentImpl {
    fn fav_language(&self) -> String{
        self.fav_language.clone()
    }
}
impl Student for CompSciStudentImpl {
    fn university(&self) -> String {
        self.university.clone()
    }
}
fn comp_sci_student_greeting(student: impl CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

fn main() {
    let cs_student = CompSciStudentImpl {
        programmer: ProgrammerImpl { fav_language: "Rust".to_string() },
        student: StudentImpl {
            person: PersonImpl { name: "Alice".to_string() },
            university: "MIT".to_string(),
        },        
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string()
    };
    println!("{}", comp_sci_student_greeting(cs_student));
}


