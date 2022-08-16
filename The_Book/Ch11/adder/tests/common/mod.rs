//older naming convention that rust also udnerstands, tells rust to not treat it as a test integration file

//Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

//we can use this from any of the integration test files as a module.

pub fn setup() {
    // setup code specific to your library's tests would go here
}