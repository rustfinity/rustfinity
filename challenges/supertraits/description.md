In Rust, supertraits allow you to define a trait hierarchy where a derived trait requires another trait to be implemented first. This is useful when modeling interfaces that build upon or depend on other capabilities.

In this challenge, you will define two traits: `Person` and `Student`. The `Person` trait provides the ability to retrieve a name, while the `Student` trait extends `Person` by adding additional fields specific to students, such as an ID and a field of study.

## Your Task

1. Define a trait `Person`:
   - It should require a method `fn name(&self) -> String` that returns the name of the person.

2. Define a trait `Student` that has `Person` as a supertrait:
   - It should require methods:
     - `fn id(&self) -> u32` to return the student ID.
     - `fn field_of_study(&self) -> String` to return the student's field of study.

3. Implement both traits for a struct `Undergraduate`:
   - The `Undergraduate` struct should have fields `id`, `name`, and `field_of_study`.
   - Use the `Student` trait to provide the student's ID, name, and field of study.

## Hints

If you're stuck, here are some hints to help you solve the challenge:

<details>
    <summary>Click here to reveal hints</summary>

- Use the syntax `trait Student: Person {}` to define `Student` as a supertrait of `Person`.
- Remember to implement both `Person` and `Student` for the `Undergraduate` struct.
- Use `self.name.clone()` and `self.field_of_study.clone()` to return `String` values without ownership issues.
- You can call methods from the supertrait explicitly using `<YourType as YourTrait>::method_name()`.

</details>
