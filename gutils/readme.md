# Global Utility

This module provides utilities to make Rust application development easier.
Regarding the re-exporting of crates: as a Rust application grows, it's common to start creating several submodules within the workspace. As this happens, many of these modules often require the same packages, which makes it tedious to maintain dependencies in five, six, or more different places. To simplify this, I'm centralizing and re-exporting the common crates here.
