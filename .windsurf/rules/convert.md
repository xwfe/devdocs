---
trigger: always_on
globs: src/**/*.rs
---

- Original project location: `./devdocs-original`
- Documentation location: `[Original project path]/lib/docs`
- Core definition location: `[Documentation location]/core`
- Core filter operations location: `[Documentation location]/filters/core`
- Multiple development document filters and scrapers location: `[Documentation location]/filters/*` and `[Documentation location]/lib/scrapers/*`
- test files location: [Original project path]/test/*`
- generated files location: `[Original project path]/generate/docs/*`

- Before making any further modifications, it is necessary to first compare with the original version's logic.
- **Maintain original project logic consistency**: The conversion process must strictly follow the original project's logical structure and avoid hardcoding.
- **Follow best practices of the target language**: The converted code should comply with the best practices and conventions of the target programming language, ensuring the code is clean, optimized, and maintainable.
- **Avoid feature assumptions**: Do not add or modify any features that are not present in the original project.
- **Isolated testing**: If testing new functionality is necessary, create a separate small project in the root directory to avoid contaminating the current project.
- **Automatic fixes**: Automatically apply necessary fixes when confirmed consistent with the original project logic without requiring user confirmation.
- The structure of the document has not changed at present and is completely correct. You don't need to query the web page structure.

如果你已经明确了上面的这些要求，请再开始任务前，明确告诉我"**你已经了解了项目背景要求**"