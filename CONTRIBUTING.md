# How to Contribute
 1. Create a release branch and a pull request to the `main` branch
 2. Push your changes, and ensure versions for the following packages are incremented appropriately:
    * `Cargo.toml`
    * `xyo-http/Cargo.toml`
    * `example/Cargo.toml`
 3. Monitor the pipeline to pass with a green light
 4. Merge to the `main` branch and delete the release branch
 5. Create a new tag according matching incremented version in `Cargo.toml` files
    * `git tag 1.x.x`
    * `git push origin 1.x.x`
 6. Push the new tag to the remote repository and monitor two pipelines
    * Pipeline will be triggered to first publish `xyo-http` package
    * On Failure or Success from previous pipeline, it will execute the publication for `xyo-sdk`
 7. All pipelines should pass with a green light indicating the successful publishing on Crates.io
