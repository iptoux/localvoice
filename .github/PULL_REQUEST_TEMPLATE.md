name: Pull Request
description: Submit changes to LocalVoice
labels: []
body:
  - type: markdown
    attributes:
      value: |
        Thanks for contributing to LocalVoice!

        Please make sure your PR:
        - Targets the `main` branch
        - Follows the coding standards in CONTRIBUTING.md
        - Includes tests where applicable
        - Updates documentation if needed

  - type: textarea
    id: description
    attributes:
      label: Description
      description: What does this PR do?
      placeholder: |
        - Add feature X
        - Fix bug Y
        - Refactor component Z
    validations:
      required: true

  - type: textarea
    id: related
    attributes:
      label: Related Issues
      description: Link any related issues (e.g., "Fixes #123")

  - type: textarea
    id: testing
    attributes:
      label: Testing Done
      description: How did you test your changes?
      placeholder: |
        - [ ] Tested recording flow
        - [ ] Tested transcription
        - [ ] Tested on [OS]

  - type: textarea
    id: checklist
    attributes:
      label: Checklist
      description: Make sure all applicable items are checked
      placeholder: |
        - [ ] Code follows the project's style guidelines
        - [ ] Self-review completed
        - [ ] Documentation updated
        - [ ] No new warnings introduced
