1. Read the current `CHANGELOG.md`.
2. Analyze the commits since the last update (from `adc25014` to `HEAD`) which include:
   - Fixes to `phloem` `MATCH` optimization where it confused internal node IDs with the "id" property.
   - Fallback graphics display initialization for headless systems using `display_fake`.
   - Implementation of the `fortune` application for Daily Inspiration.
   - Addition of Developer Workflow BDD tests using GQL queries via the `anther` service, and toolchain pinning.
   - An experimental migration to `no_std` for userspace apps, which was later reverted.
3. Draft a comprehensive release section formatted according to previous changelog conventions, with a descriptive intro and categorized bullet points linking to feature artifacts.
4. Merge the new section into the top of `CHANGELOG.md`.
5. Run tests locally (`cargo test --workspace --doc` avoiding failing tests if necessary, or `cargo xtask bdd`) to verify nothing is broken.
6. Call the pre_commit_instructions tool to ensure proper testing, verification, review, and reflection are done.
7. Submit the changes.
